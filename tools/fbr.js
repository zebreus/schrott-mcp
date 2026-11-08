// Fachbetrieberegister-Client (Vaadin, clientseitig gerendert) via Playwright/Chromium.
// Nur lesende öffentliche Abfragen: keine Anmeldung, keine Schreiboperationen.
//
// Aufruf: node tools/fbr.js <queries.jsonl> <workers>
// Ausgabe: eine JSON-Zeile je Abfrage (Trefferzahl + vollstaendig gelesene Zeilen).
const {chromium} = require('/tmp/opencode/pw/node_modules/playwright-core');
const fs = require('fs');

const BASE = 'https://fachbetrieberegister.gadsys.de/fachbetrieberegister';
const CHROME = process.env.CHROMIUM_PATH
  || '/root/.cache/ms-playwright/chromium-1254/chrome-linux64/chrome';

function buildUrl(q) {
  const p = [];
  if (q.adr) {
    const r = q.dst || 10;
    const dlat = r / 111.32;
    const dlon = r / (111.32 * Math.cos((q.lat * Math.PI) / 180));
    const adr = `${q.adr}, Deutschland;${q.lon};${q.lat};`
      + `${(q.lon + dlon).toFixed(7)};${(q.lat + dlat).toFixed(7)};`
      + `${(q.lon - dlon).toFixed(7)};${(q.lat - dlat).toFixed(7)}`;
    p.push(`adr=${encodeURIComponent(adr)}`);
  }
  p.push(`dst=${q.dst || 10}`);
  p.push('is=1');
  p.push('e=1');
  if (q.name) p.push(`n=${encodeURIComponent(q.name)}`);
  return `${BASE}/${q.route}/${p.join('&')}`;
}

async function runQuery(page, q) {
  const url = buildUrl(q);
  await page.goto(url, {waitUntil: 'domcontentloaded', timeout: 120000});
  let hits = null;
  for (let i = 0; i < 45; i++) {
    await page.waitForTimeout(1500);
    const txt = await page.evaluate(() => document.body.innerText);
    const m = txt.match(/([\d.]+)\s+Treffer gefunden/);
    if (m) { hits = parseInt(m[1].replace(/\./g, ''), 10); break; }
  }
  const rows = await page.evaluate(async () => {
    const g = document.getElementById('Ergebnisliste');
    if (!g) return [];
    g.style.height = '40000px';
    g.style.minHeight = '40000px';
    await new Promise((r) => setTimeout(r, 2500));
    const sr = g.shadowRoot;
    const items = sr && sr.getElementById('items');
    if (!items) return [];
    const out = [];
    for (const r of items.querySelectorAll('tr')) {
      const tds = Array.from(r.querySelectorAll('td'));
      const cells = tds.map((td) => {
        const slot = td.querySelector('slot');
        const el = slot && slot.assignedElements ? slot.assignedElements() : [];
        return el.map((e) => (e.innerText || e.textContent || '').trim().replace(/\s+/g, ' ')).join(' ');
      });
      out.push(cells);
    }
    return out;
  });
  return {url, hits, rows};
}

async function worker(queries, out) {
  const browser = await chromium.launch({
    headless: true,
    executablePath: CHROME,
    args: ['--no-sandbox', '--disable-dev-shm-usage'],
  });
  const page = await browser.newPage({viewport: {width: 1600, height: 1400}});
  for (const q of queries) {
    try {
      const res = await runQuery(page, q);
      out.push(JSON.stringify({query: q, ...res}));
    } catch (e) {
      out.push(JSON.stringify({query: q, error: e.message}));
    }
    fs.appendFileSync(process.env.FBR_OUT || '/tmp/opencode/fbr_out.jsonl',
      out[out.length - 1] + '\n');
  }
  await browser.close();
}

async function main() {
  const file = process.argv[2];
  const workers = parseInt(process.argv[3] || '3', 10);
  const queries = fs.readFileSync(file, 'utf8').split('\n').filter((l) => l.trim())
    .map((l) => JSON.parse(l));
  const chunks = Array.from({length: workers}, () => []);
  queries.forEach((q, i) => chunks[i % workers].push(q));
  const out = [];
  await Promise.all(chunks.map((c) => worker(c, out)));
  console.log(JSON.stringify({done: out.length}));
}
main().catch((e) => { console.error('ERR', e.message); process.exit(1); });
