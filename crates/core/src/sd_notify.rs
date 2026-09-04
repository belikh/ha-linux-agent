//! Minimal sd_notify support: READY on startup plus watchdog pings, over
//! the NOTIFY_SOCKET unix datagram socket. No-op when the socket (or
//! WATCHDOG_USEC) is absent — i.e. outside systemd — so tests and manual
//! runs are unaffected.
//!
//! The unit pairs this with `WatchdogSec=`: the pings fire from the poll
//! loop after every turn, so a wedged loop stops pinging and systemd
//! restarts the agent — a panic or hang costs one restart, not the host's
//! telemetry until the next reboot.

use std::time::Duration;
use tokio::net::UnixDatagram;

/// Send `READY=1` to the service manager. Best-effort: failures are
/// returned for the caller to log at debug level, never fatal.
pub async fn notify_ready() -> std::io::Result<()> {
    send(b"READY=1").await
}

/// Send one watchdog ping. Call this at least every `watchdog_interval()/2`.
pub async fn watchdog_ping() -> std::io::Result<()> {
    send(b"WATCHDOG=1").await
}

/// The ping cadence implied by WATCHDOG_USEC (half the window, clamped to
/// at least 1 s). `None` when no watchdog is configured.
pub fn watchdog_interval() -> Option<Duration> {
    let usec: u64 = std::env::var("WATCHDOG_USEC").ok()?.parse().ok()?;
    let secs = (usec as f64 / 1_000_000.0 / 2.0).max(1.0);
    Some(Duration::from_secs_f64(secs))
}

async fn send(payload: &[u8]) -> std::io::Result<()> {
    let path = std::env::var("NOTIFY_SOCKET").map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "NOTIFY_SOCKET not set")
    })?;
    // systemd may hand us an abstract socket (@-prefixed) or a path; the
    // @ form uses the Linux abstract namespace with a leading NUL.
    let addr = path.replacen('@', "\0", 1);
    let sock = UnixDatagram::unbound()?;
    sock.send_to(payload, addr).await.map(|_| ())
}
