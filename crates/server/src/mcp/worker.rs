//! Isolated SQL execution: each ad-hoc query runs in a throwaway worker
//! process `(schrott-mcp-query-worker binary next to this server) instead of the main
//! address space.
//!
//! Confinement, applied in `pre_exec` (post-fork, pre-exec — syscalls only):
//! address-space cap (OOMs kill the worker, never us), CPU-time cap,
//! no core dumps, no new processes, few fds, `NO_NEW_PRIVS`, and a drop to
//! the `nobody` user when we started as root. The worker opens the public
//! database read-only and speaks one JSON request/response over stdio.
//! A wall-clock timeout kills stragglers from the outside.

use std::path::{Path, PathBuf};

use serde::Deserialize;
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};

/// What the worker is allowed to eat. Normal queries use megabytes.
const WORKER_AS_BYTES: u64 = 512 * 1024 * 1024;
/// CPU seconds before the kernel kills the worker.
const WORKER_CPU_SECS: u64 = 30;
/// Wall-clock seconds before we kill the worker ourselves.
const WORKER_WALL_SECS: u64 = 60;
/// Largest worker stdout we will even parse (the blob cap sits below this).
const WORKER_OUT_CAP: usize = 12 * 1024 * 1024;
/// Unprivileged user the worker becomes when we started as root.
const NOBODY: u32 = 65534;

/// Every way an isolated query can fail. Worker internals never leak:
/// details stay in our logs, callers get one line.
#[derive(Debug, thiserror::Error)]
pub enum WorkerError {
    /// The worker binary is missing or would not start.
    #[error("query worker unavailable")]
    Unavailable,
    /// Confinement or stdio setup failed before exec.
    #[error("query worker could not start")]
    Spawn(#[source] std::io::Error),
    /// Wall-clock timeout, or the kernel CPU cap; the worker was killed.
    #[error("query timed out and was killed (CPU limit 30s, wall-clock 60s); narrow it with WHERE / LIMIT")]
    TimedOut,
    /// The worker died without an answer (crash, panic). Stderr is logged,
    /// not shown.
    #[error("query worker died unexpectedly; retry, and narrow the query with WHERE / LIMIT if it persists")]
    Died,
    /// The worker answered, but not in the protocol.
    #[error("query worker gave an unreadable answer; retry with a narrower query (WHERE / LIMIT)")]
    BadOutput,
    /// The worker rejected the query (validation, database error).
    #[error("{0}")]
    Rejected(String),
    /// The answer was bigger than we are willing to parse.
    #[error("result too large to share; narrow it with WHERE / LIMIT")]
    TooLarge,
}

#[derive(Deserialize)]
struct WireReply {
    #[serde(default)]
    ok: Option<WireOk>,
    #[serde(default)]
    error: Option<String>,
}

/// Actionable cause shared by every out-of-memory kill path.
const OOM_MESSAGE: &str = "query used too much memory and was killed; narrow it with WHERE / LIMIT";

/// Turn a bare worker death (non-zero exit, no `{"error"}` envelope) into
/// an actionable error. Kernel kills carry the cause in the exit status:
/// SIGXCPU is the 30 s CPU cap (→ timeout); SIGKILL from anyone but us is
/// the OOM killer — our own wall-clock kill returns earlier and never
/// reaches here (the worker installs no SIGXCPU handler, so the CPU cap's
/// second-stage SIGKILL is unreachable too). A Rust/SQLite allocation
/// failure aborts with SIGABRT or SIGSEGV and prints
/// `memory allocation … failed` to stderr. Stderr is only matched for
/// those keywords — never forwarded — so internal paths cannot leak to
/// the client. Anything else stays [`WorkerError::Died`].
fn classify_death(status: &std::process::ExitStatus, stderr: &[u8]) -> WorkerError {
    use std::os::unix::process::ExitStatusExt as _;
    match status.signal() {
        Some(sig) if sig == libc::SIGXCPU => WorkerError::TimedOut,
        Some(sig) if sig == libc::SIGKILL => WorkerError::Rejected(OOM_MESSAGE.to_owned()),
        _ => {
            let lower = String::from_utf8_lossy(stderr).to_lowercase();
            if lower.contains("memory allocation") || lower.contains("out of memory") {
                WorkerError::Rejected(OOM_MESSAGE.to_owned())
            } else {
                WorkerError::Died
            }
        }
    }
}

#[derive(Deserialize)]
struct WireOk {
    columns: Vec<schrott_mcp_store::SqlColumn>,
    rows: Vec<Vec<serde_json::Value>>,
}

/// Apply OS confinement inside `pre_exec`. Syscalls only — no allocation,
/// no locks, no Rust runtime calls; the process is freshly forked.
fn confine(drop_privs: bool) -> std::io::Result<()> {
    // SAFETY: raw syscalls that are async-signal-safe; all values are
    // plain integers copied onto the stack, nothing borrowed heap state.
    unsafe {
        let cap = |resource: u32, value: u64| -> std::io::Result<()> {
            let lim = libc::rlimit {
                rlim_cur: value as libc::rlim_t,
                rlim_max: value as libc::rlim_t,
            };
            if libc::setrlimit(resource, &lim) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        };
        cap(libc::RLIMIT_AS, WORKER_AS_BYTES)?;
        cap(libc::RLIMIT_CPU, WORKER_CPU_SECS)?;
        cap(libc::RLIMIT_FSIZE, 64 * 1024 * 1024)?;
        cap(libc::RLIMIT_NOFILE, 32)?;
        cap(libc::RLIMIT_NPROC, 0)?;
        cap(libc::RLIMIT_CORE, 0)?;
        if libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0 {
            return Err(std::io::Error::last_os_error());
        }
        if drop_privs {
            if libc::setgid(NOBODY) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            if libc::setuid(NOBODY) != 0 {
                return Err(std::io::Error::last_os_error());
            }
        }
    }
    Ok(())
}

/// Path of the worker binary sitting next to this server binary.
fn worker_path() -> Result<PathBuf, WorkerError> {
    let mut path = std::env::current_exe().map_err(|e| {
        tracing::warn!("query worker path unavailable: {e}");
        WorkerError::Unavailable
    })?;
    path.set_file_name("schrott-mcp-query-worker");
    Ok(path)
}

/// Run one SQL query in a confined worker and return the full result.
pub async fn run_query(
    data_dir: &Path,
    sql: &str,
) -> Result<schrott_mcp_store::SqlResult, WorkerError> {
    let request = serde_json::json!({"sql": sql}).to_string();
    // SAFETY: `getuid` is async-signal-safe; read once, outside pre_exec.
    let drop_privs = unsafe { libc::getuid() == 0 };
    let mut cmd = tokio::process::Command::new(worker_path()?);
    // SAFETY: `confine` performs only async-signal-safe syscalls on stack
    // values (see its own contract); the closure captures nothing but
    // plain integers, so post-fork execution is sound.
    unsafe {
        cmd.arg(data_dir)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .env_clear()
            .pre_exec(move || confine(drop_privs));
    }
    let mut child = cmd.spawn().map_err(|e| {
        // Includes pre_exec confinement failures: the errno stays in our
        // log, the client gets the safe one-liner below.
        tracing::warn!("query worker spawn failed: {e}");
        WorkerError::Spawn(e)
    })?;
    if let Some(mut stdin) = child.stdin.take() {
        // The worker already started here: a broken pipe means it died
        // mid-flight, not that it could not start.
        stdin.write_all(request.as_bytes()).await.map_err(|e| {
            tracing::warn!("query worker stdin failed: {e}");
            WorkerError::Died
        })?;
    }
    let mut stdout_taken = child.stdout.take();
    let mut stderr_taken = child.stderr.take();
    let collect = async {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let mut stdout_overflow = false;
        if let Some(ref mut o) = stdout_taken {
            // Read only the head: the check below reports TooLarge, and
            // draining the rest lets the worker exit so a big result is
            // not misreported as a timeout.
            (&mut *o)
                .take(WORKER_OUT_CAP as u64 + 1)
                .read_to_end(&mut out)
                .await
                .map_err(|e| {
                    tracing::warn!("query worker stdout failed: {e}");
                    WorkerError::Died
                })?;
            if out.len() > WORKER_OUT_CAP {
                stdout_overflow = true;
                tokio::io::copy(&mut *o, &mut tokio::io::sink())
                    .await
                    .map_err(|e| {
                        tracing::warn!("query worker stdout drain failed: {e}");
                        WorkerError::Died
                    })?;
            }
        }
        if let Some(ref mut e) = stderr_taken {
            e.read_to_end(&mut err).await.map_err(|e| {
                tracing::warn!("query worker stderr failed: {e}");
                WorkerError::Died
            })?;
        }
        let status = child.wait().await.map_err(|e| {
            tracing::warn!("query worker wait failed: {e}");
            WorkerError::Died
        })?;
        Ok::<_, WorkerError>((status, out, err, stdout_overflow))
    };
    let (status, out, stderr, stdout_overflow) =
        match tokio::time::timeout(std::time::Duration::from_secs(WORKER_WALL_SECS), collect).await
        {
            Err(_) => {
                // Kill AND reap: dropping the handle without wait() would
                // leave a zombie. SIGKILL cannot be ignored; the extra wait
                // only lingers if the child is in uninterruptible sleep.
                tracing::warn!("query worker wall-clock timeout ({WORKER_WALL_SECS}s); killed");
                let _ = child.kill().await;
                let _ = tokio::time::timeout(std::time::Duration::from_secs(5), child.wait()).await;
                return Err(WorkerError::TimedOut);
            }
            Ok(Err(e)) => return Err(e),
            Ok(Ok(t)) => t,
        };
    if !status.success() {
        // The worker prints {"error": "..."} before exiting — surface that
        // instead of a generic death notice, so callers know whether to
        // rephrase (validation) or retry (crash).
        if let Ok(reply) = serde_json::from_slice::<WireReply>(&out) {
            if let Some(message) = reply.error {
                return Err(WorkerError::Rejected(message));
            }
        }
        // No envelope: classify the kernel kill (CPU cap, OOM) so the
        // client gets the cause, not a generic death notice.
        let error = classify_death(&status, &stderr);
        if matches!(error, WorkerError::Died) {
            tracing::warn!(
                "query worker died: status={status} stderr={}",
                String::from_utf8_lossy(&stderr)
                    .chars()
                    .take(500)
                    .collect::<String>()
            );
        }
        return Err(error);
    }
    if stdout_overflow {
        return Err(WorkerError::TooLarge);
    }
    let reply: WireReply = serde_json::from_slice(&out).map_err(|_| WorkerError::BadOutput)?;
    if let Some(message) = reply.error {
        return Err(WorkerError::Rejected(message));
    }
    reply.ok.map_or(Err(WorkerError::BadOutput), |ok| {
        Ok(schrott_mcp_store::SqlResult {
            columns: ok.columns,
            rows: ok.rows,
        })
    })
}

#[cfg(test)]
mod tests {
    use std::os::unix::process::ExitStatusExt as _;

    use super::{classify_death, WireReply, WorkerError, OOM_MESSAGE};
    use super::{WORKER_CPU_SECS, WORKER_WALL_SECS};

    fn signal_status(sig: i32) -> std::process::ExitStatus {
        std::process::ExitStatus::from_raw(sig)
    }

    fn exit_status(code: u8) -> std::process::ExitStatus {
        std::process::ExitStatus::from_raw((code as i32) << 8)
    }

    #[test]
    fn wire_protocol_parses() {
        let ok: WireReply = serde_json::from_str(
            r#"{"ok":{"columns":[{"name":"n","type":"integer"}],"rows":[[1]]}}"#,
        )
        .expect("ok parses");
        assert!(ok.error.is_none());
        let inner = ok.ok.expect("has ok");
        assert_eq!(inner.columns[0].name, "n");
        assert_eq!(inner.rows.len(), 1);

        let err: WireReply = serde_json::from_str(r#"{"error":"nope"}"#).expect("err parses");
        assert_eq!(err.error.as_deref(), Some("nope"));
        assert!(err.ok.is_none());

        let empty: WireReply = serde_json::from_str("{}").expect("empty parses");
        assert!(empty.ok.is_none() && empty.error.is_none());
    }

    #[test]
    fn timeout_message_names_both_caps_and_hint() {
        // The message must stay in sync with the constants: clients need
        // to tell the 30 s CPU kill from the 60 s wall-clock kill.
        let msg = WorkerError::TimedOut.to_string();
        assert!(msg.contains(&WORKER_CPU_SECS.to_string()), "{msg}");
        assert!(msg.contains(&WORKER_WALL_SECS.to_string()), "{msg}");
        assert!(msg.contains("WHERE / LIMIT"), "{msg}");
    }

    #[test]
    fn cpu_kill_is_timeout_not_death() {
        let err = classify_death(&signal_status(libc::SIGXCPU), b"");
        assert!(
            matches!(err, WorkerError::TimedOut),
            "SIGXCPU must map to TimedOut, got {err}"
        );
    }

    #[test]
    fn sigkill_is_oom_with_hint() {
        let err = classify_death(&signal_status(libc::SIGKILL), b"");
        match err {
            WorkerError::Rejected(m) => assert_eq!(m, OOM_MESSAGE),
            other => panic!("SIGKILL must map to OOM Rejected, got {other}"),
        }
    }

    #[test]
    fn abort_with_oom_stderr_is_oom() {
        let err = classify_death(
            &signal_status(libc::SIGABRT),
            b"memory allocation of 123456 bytes failed",
        );
        match err {
            WorkerError::Rejected(m) => assert_eq!(m, OOM_MESSAGE),
            other => panic!("OOM abort must map to OOM Rejected, got {other}"),
        }
    }

    #[test]
    fn abort_without_oom_keywords_stays_died() {
        // A panic carries internal paths in stderr — the client must get
        // the safe fallback, never the stderr text.
        let stderr = b"thread 'main' panicked at crates/query-worker/src/main.rs:42";
        let err = classify_death(&signal_status(libc::SIGABRT), stderr);
        assert!(matches!(err, WorkerError::Died), "got {err}");
        assert!(
            !WorkerError::Died.to_string().contains("panicked"),
            "Died must not echo stderr"
        );
    }

    #[test]
    fn plain_exit_without_envelope_stays_died() {
        let err = classify_death(&exit_status(1), b"");
        assert!(matches!(err, WorkerError::Died), "got {err}");
    }

    #[test]
    fn fallback_messages_are_actionable() {
        for err in [WorkerError::Died, WorkerError::BadOutput] {
            let msg = err.to_string();
            assert!(msg.contains("WHERE / LIMIT"), "{msg}");
            assert!(msg.contains("retr"), "{msg}");
        }
    }

    #[test]
    fn classified_messages_never_echo_stderr_paths() {
        let stderr = b"cannot open /secret/path/public.db: permission denied";
        for status in [signal_status(libc::SIGKILL), signal_status(libc::SIGXCPU)] {
            let msg = classify_death(&status, stderr).to_string();
            assert!(!msg.contains("/secret/path"), "{msg}");
        }
    }
}
