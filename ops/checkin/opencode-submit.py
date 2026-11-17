#!/usr/bin/python3
"""Admit scheduled Schrott prompts to the web runner; never wait for the agent.

Unlike `opencode run`, this neither discovers a second server nor holds a CLI
process for the lifetime of the session. It preserves prompt text and refuses
to wake archived sessions. Authentication is read locally, never printed.
"""
import argparse
import base64
import fcntl
import json
import os
from pathlib import Path
import shlex
import subprocess
import sys
import urllib.error
import urllib.request


def submit(api, session_id, text, kind):
    info = api('/api/session/' + session_id)['data']
    if info.get('time', {}).get('archived'):
        raise RuntimeError('Refusing to resume an archived session')
    inbox = api('/api/session/' + session_id + '/inbox')['data']
    for item in inbox:
        payload = item.get('payload', {})
        if payload.get('text') == text or payload.get('metadata', {}).get('schrottAutomation') == kind:
            return {'status': 'coalesced', 'sessionID': session_id}
    admitted = api('/api/session/' + session_id + '/prompt', {
        'text': text,
        'delivery': 'queue',
        'resume': True,
        'metadata': {'schrottAutomation': kind},
    })
    return {'status': 'admitted', 'sessionID': session_id, 'messageID': admitted['data']['id']}


def connection():
    environment = subprocess.check_output([
        'systemctl', 'show', 'opencode-server', '-p', 'Environment', '--value',
    ], text=True, timeout=10)
    values = dict(x.split('=', 1) for x in shlex.split(environment) if '=' in x)
    password = values.get('OPENCODE_SERVER_PASSWORD', '')
    headers = {'Content-Type': 'application/json'}
    if password:
        headers['Authorization'] = 'Basic ' + base64.b64encode(('opencode:' + password).encode()).decode()

    def api(path, body=None):
        request = urllib.request.Request(
            'http://127.0.0.1:3000' + path,
            data=json.dumps(body).encode() if body is not None else None,
            headers=headers,
        )
        try:
            with urllib.request.urlopen(request, timeout=30) as response:
                return json.load(response)
        except urllib.error.HTTPError as error:
            raise RuntimeError('OpenCode API returned HTTP ' + str(error.code)) from None
    return api


def model_reference(value):
    if '/' not in value:
        raise ValueError('Model must be provider/model or provider/model#variant')
    provider, name = value.split('/', 1)
    model, separator, variant = name.partition('#')
    result = {'providerID': provider, 'id': model}
    if separator:
        result['variant'] = variant
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--session')
    parser.add_argument('--model')
    parser.add_argument('--kind', choices=['hourly', 'wave'], required=True)
    parser.add_argument('--text', required=True)
    parser.add_argument('--dry-run', action='store_true')
    args = parser.parse_args()
    if args.dry_run:
        print(json.dumps({'server': 'http://127.0.0.1:3000', 'kind': args.kind,
                          'session': args.session or 'fresh', 'model': args.model,
                          'promptChars': len(args.text), 'waitForCompletion': False}))
        return
    state_dir = Path('/var/lib/opencode-automation')
    state_dir.mkdir(mode=0o700, parents=True, exist_ok=True)
    lock_fd = os.open(state_dir / 'schrott-submit.lock', os.O_CREAT | os.O_RDWR, 0o600)
    with os.fdopen(lock_fd, 'w') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX)
        api = connection()
        session_id = args.session
        state_file = state_dir / (args.kind + '.json')
        if not session_id:
            if state_file.exists():
                previous = json.loads(state_file.read_text())['sessionID']
                previous_info = api('/api/session/' + previous)['data']
                if not previous_info.get('time', {}).get('archived'):
                    queued = api('/api/session/' + previous + '/inbox')['data']
                    tail = api('/api/session/' + previous + '/message?limit=1')['data']
                    busy = queued or (tail and tail[0].get('type') == 'assistant'
                                      and not tail[0].get('time', {}).get('completed'))
                    if busy:
                        print(json.dumps({'status': 'coalesced-active', 'sessionID': previous}))
                        return
            create = {'title': 'Schrott MCP — hourly owner check-in',
                      'location': {'directory': '/root/Documents/schrott-mcp'}}
            if args.model:
                create['model'] = model_reference(args.model)
            session_id = api('/api/session', create)['data']['id']
        # Remember the session before admission, so a retry reuses it.
        fd = os.open(state_file, os.O_CREAT | os.O_WRONLY | os.O_TRUNC, 0o600)
        with os.fdopen(fd, 'w') as state:
            json.dump({'sessionID': session_id}, state)
        print(json.dumps(submit(api, session_id, args.text, args.kind)))


if __name__ == '__main__':
    try:
        main()
    except Exception as error:
        print('Submission failed: ' + str(error), file=sys.stderr)
        sys.exit(1)
