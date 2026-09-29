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
    /// Wall-clock timeout; the worker was killed.
    #[error("query timed out and was killed")]
    TimedOut,
    /// The worker died (OOM, CPU cap, crash). Stderr is logged, not shown.
    #[error("query worker died")]
    Died,
    /// The worker answered, but not in the protocol.
    #[error("query worker gave an unreadable answer")]
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
    let mut path = std::env::current_exe().map_err(|_| WorkerError::Unavailable)?;
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
    let mut child = cmd.spawn().map_err(WorkerError::Spawn)?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(request.as_bytes())
            .await
            .map_err(WorkerError::Spawn)?;
    }
    let mut stdout_taken = child.stdout.take();
    let mut stderr_taken = child.stderr.take();
    let collect = async {
        let mut out = Vec::new();
        let mut err = Vec::new();
        if let Some(ref mut o) = stdout_taken {
            o.read_to_end(&mut out).await.map_err(WorkerError::Spawn)?;
        }
        if let Some(ref mut e) = stderr_taken {
            e.read_to_end(&mut err).await.map_err(WorkerError::Spawn)?;
        }
        let status = child.wait().await.map_err(WorkerError::Spawn)?;
        Ok::<_, WorkerError>((status, out, err))
    };
    let (status, out, err) =
        match tokio::time::timeout(std::time::Duration::from_secs(WORKER_WALL_SECS), collect).await
        {
            Err(_) => {
                // Kill AND reap: dropping the handle without wait() would
                // leave a zombie. SIGKILL cannot be ignored; the extra wait
                // only lingers if the child is in uninterruptible sleep.
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
        tracing::warn!(
            "query worker died: status={status} stderr={}",
            String::from_utf8_lossy(&err)
                .chars()
                .take(500)
                .collect::<String>()
        );
        return Err(WorkerError::Died);
    }
    if out.len() > WORKER_OUT_CAP {
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
    use super::WireReply;

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
}
