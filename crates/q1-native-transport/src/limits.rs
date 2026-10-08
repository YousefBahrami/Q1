//! Local TESTNET resource admission, independent of ledger validity.
use super::*;
use std::sync::{OnceLock, atomic::AtomicBool};
const AUTH_LIMIT: usize = 8;
const PENDING_LIMIT: usize = 2;
const PER_PEER: usize = 2;
const FRAME_BURST: usize = 32;
struct Window {
    start: Instant,
    used: usize,
}
impl Window {
    fn new() -> Self {
        Self {
            start: Instant::now(),
            used: 0,
        }
    }
    fn allow(&mut self, maximum: usize) -> bool {
        if self.start.elapsed() >= Duration::from_secs(1) {
            self.start = Instant::now();
            self.used = 0;
        }
        if self.used >= maximum {
            return false;
        }
        self.used += 1;
        true
    }
}
struct State {
    pending: usize,
    authenticated: usize,
    peers: [usize; 6],
    frames: [Window; 6],
}
pub(super) struct Limits {
    pub(super) stopping: Arc<AtomicBool>,
    state: Mutex<State>,
    bytes: AtomicUsize,
}
pub(super) struct Permit {
    limits: Arc<Limits>,
    peer: Option<usize>,
}
pub(super) struct Buffer {
    limits: Arc<Limits>,
}
impl Limits {
    pub(super) fn new() -> Arc<Self> {
        Arc::new(Self {
            stopping: Arc::new(AtomicBool::new(false)),
            state: Mutex::new(State {
                pending: 0,
                authenticated: 0,
                peers: [0; 6],
                frames: std::array::from_fn(|_| Window::new()),
            }),
            bytes: AtomicUsize::new(0),
        })
    }
    pub(super) fn pending(self: &Arc<Self>) -> R<Permit> {
        let mut s = self.state.lock().map_err(|_| "ADMISSION_LOCK")?;
        need(s.pending < PENDING_LIMIT, "PREAUTH_CAP")?;
        s.pending += 1;
        Ok(Permit {
            limits: self.clone(),
            peer: None,
        })
    }
    pub(super) fn frame(&self, peer: usize) -> R<()> {
        need(
            self.state.lock().map_err(|_| "ADMISSION_LOCK")?.frames[peer].allow(FRAME_BURST),
            "PEER_FRAME_RATE",
        )
    }
    pub(super) fn buffer(self: &Arc<Self>) -> R<Buffer> {
        // A conservative two-frame reservation per active exchange. No network
        // work queue is permitted to accumulate beyond this global byte budget.
        self.bytes
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| {
                (n + 2 * MAX <= 1024 * 1024).then_some(n + 2 * MAX)
            })
            .map_err(|_| "NETWORK_QUEUE_BYTES")?;
        Ok(Buffer {
            limits: self.clone(),
        })
    }
}
impl Permit {
    pub(super) fn authenticate(&mut self, peer: usize) -> R<()> {
        let mut s = self.limits.state.lock().map_err(|_| "ADMISSION_LOCK")?;
        need(
            s.authenticated < AUTH_LIMIT && s.peers[peer] < PER_PEER,
            "AUTH_PEER_CAP",
        )?;
        s.pending -= 1;
        s.authenticated += 1;
        s.peers[peer] += 1;
        self.peer = Some(peer);
        Ok(())
    }
}
impl Drop for Permit {
    fn drop(&mut self) {
        if let Ok(mut s) = self.limits.state.lock() {
            if let Some(p) = self.peer {
                s.authenticated -= 1;
                s.peers[p] -= 1;
            } else {
                s.pending -= 1;
            }
        }
    }
}
impl Drop for Buffer {
    fn drop(&mut self) {
        self.limits.bytes.fetch_sub(2 * MAX, Ordering::SeqCst);
    }
}
pub(super) fn log(event: &str, detail: &str) {
    static LOG: OnceLock<Mutex<(Window, usize)>> = OnceLock::new();
    if let Ok(mut state) = LOG.get_or_init(|| Mutex::new((Window::new(), 0))).lock() {
        if state.0.allow(20) {
            let omitted = std::mem::take(&mut state.1);
            eprintln!(
                "{}",
                json!({"profile":PROFILE,"event":event,"detail":detail.chars().take(512).collect::<String>(),"suppressed":omitted})
            );
        } else {
            state.1 = state.1.saturating_add(1);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn diagnostic_window_is_bounded_and_resets() {
        let mut window = Window::new();
        for _ in 0..20 {
            assert!(window.allow(20));
        }
        assert!(!window.allow(20));
        window.start = Instant::now() - Duration::from_secs(2);
        assert!(window.allow(20));
    }
    #[test]
    fn pending_and_peer_caps_release() {
        let l = Limits::new();
        let mut a = l.pending().unwrap();
        let b = l.pending().unwrap();
        assert!(l.pending().is_err());
        a.authenticate(0).unwrap();
        drop(b);
        let mut b = l.pending().unwrap();
        b.authenticate(0).unwrap();
        let mut c = l.pending().unwrap();
        assert!(c.authenticate(0).is_err());
        drop(a);
        c.authenticate(0).unwrap();
    }
    #[test]
    fn byte_and_message_bounds() {
        let l = Limits::new();
        let mut guards = Vec::new();
        for _ in 0..8 {
            guards.push(l.buffer().unwrap());
        }
        assert!(l.buffer().is_err());
        guards.pop();
        assert!(l.buffer().is_ok());
        for _ in 0..32 {
            l.frame(0).unwrap();
        }
        assert!(l.frame(0).is_err());
        assert!(l.frame(1).is_ok());
    }
}
