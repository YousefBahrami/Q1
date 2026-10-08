//! Permissioned TESTNET ONLY TLS transport. No public binds or consensus rules.
mod limits;
mod pool;
use limits::{Limits, Permit, log};
use q1_primitives::cbor::{self, Value as C};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, ServerName};
use rustls::{
    ClientConfig, ClientConnection, RootCertStore, ServerConfig, ServerConnection, StreamOwned,
};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    os::unix::{fs::PermissionsExt, net::UnixStream},
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
    time::{Duration, Instant},
};
type E = Box<dyn std::error::Error + Send + Sync>;
type R<T> = Result<T, E>;
const PROFILE: &str = "TESTNET_NATIVE_TRANSPORT_V0";
const ALPN: &[u8] = b"q1-testnet-native-v0";
const MAX: usize = 65536;
fn need(ok: bool, s: &str) -> R<()> {
    if ok { Ok(()) } else { Err(s.into()) }
}
fn hex(b: &[u8]) -> String {
    b.iter().map(|b| format!("{b:02x}")).collect()
}
fn unhex(s: &str) -> R<Vec<u8>> {
    need(s.len() == 64 && s.is_ascii(), "HASH_WIDTH")?;
    let b = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect::<Result<Vec<_>, _>>()?;
    need(hex(&b) == s, "HASH_CANONICAL")?;
    Ok(b)
}
fn hash(b: &[u8]) -> String {
    hex(&Sha256::digest(b))
}
fn canonical(v: &Value) -> R<Vec<u8>> {
    Ok(serde_json::to_vec(v)?)
}
fn read_json(p: &Path) -> R<Value> {
    let f = File::open(p)?;
    let mut b = Vec::new();
    f.take(131073).read_to_end(&mut b)?;
    need(b.len() <= 131072, "STORE_LIMIT")?;
    let v: Value = serde_json::from_slice(&b)?;
    need(canonical(&v)? == b, "NONCANONICAL_STORE")?;
    Ok(v)
}
fn private_file(p: &Path) -> R<()> {
    let m = fs::symlink_metadata(p)?;
    need(
        m.is_file() && m.permissions().mode() & 0o077 == 0 && m.len() <= 65536,
        "PRIVATE_FILE",
    )
}
fn atomic(p: &Path, v: &Value) -> R<()> {
    let b = canonical(v)?;
    need(b.len() <= 131072, "STORE_LIMIT")?;
    let t = p.with_extension("pending");
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&t)?;
    f.set_permissions(fs::Permissions::from_mode(0o600))?;
    f.write_all(&b)?;
    f.sync_all()?;
    fs::rename(t, p)?;
    File::open(p.parent().ok_or("parent")?)?.sync_all()?;
    Ok(())
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Peer {
    id: String,
    role: u64,
    address: SocketAddr,
    name: String,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    profile: String,
    warning: String,
    who: usize,
    chain: String,
    genesis: String,
    ca: PathBuf,
    certificate: PathBuf,
    key: PathBuf,
    state: PathBuf,
    worker: PathBuf,
    peers: Vec<Peer>,
}
fn allowed(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(a) => {
            a.is_loopback()
                || a.is_private()
                || (a.octets()[0] == 100 && (64..128).contains(&a.octets()[1]))
        }
        IpAddr::V6(a) => a.is_loopback() || a.is_unique_local(),
    }
}
fn load(p: &Path) -> R<Config> {
    let v = read_json(p)?;
    let c: Config = serde_json::from_value(v)?;
    need(
        c.profile == PROFILE && c.warning == "TESTNET ONLY",
        "PROFILE",
    )?;
    need(c.peers.len() == 6 && c.who < 6, "ROLES")?;
    unhex(&c.chain)?;
    unhex(&c.genesis)?;
    let mut seen = std::collections::BTreeSet::new();
    let mut addresses = std::collections::BTreeSet::new();
    for (i, p) in c.peers.iter().enumerate() {
        need(
            p.role == i as u64 && seen.insert(p.id.clone()) && addresses.insert(p.address),
            "DUPLICATE_PEER",
        )?;
        unhex(&p.id)?;
        need(
            allowed(p.address.ip()) && p.address.port() >= 1024,
            "PRIVATE_ENDPOINT_REQUIRED",
        )?;
    }
    private_file(&c.key)?;
    fs::create_dir_all(&c.state)?;
    let m = fs::symlink_metadata(&c.state)?;
    need(
        m.is_dir() && m.permissions().mode() & 0o077 == 0,
        "PRIVATE_STATE",
    )?;
    for path in [&c.ca, &c.certificate] {
        let m = fs::symlink_metadata(path)?;
        need(m.is_file() && m.len() <= 65536, "CERTIFICATE_FILE_LIMIT")?;
    }
    need(
        peer_id(&fs::read(&c.certificate)?)? == c.peers[c.who].id,
        "LOCAL_IDENTITY",
    )?;
    Ok(c)
}
fn peer_id(cert: &[u8]) -> R<String> {
    let (rest, x) = x509_parser::parse_x509_certificate(cert).map_err(|_| "CERTIFICATE_PARSE")?;
    need(rest.is_empty(), "CERTIFICATE_TRAILING")?;
    let mut b = b"TESTNET_NATIVE_TRANSPORT_V0_PEER\0".to_vec();
    b.extend_from_slice(x.public_key().raw);
    Ok(hash(&b))
}
fn roots(c: &Config) -> R<RootCertStore> {
    let mut r = RootCertStore::empty();
    r.add(CertificateDer::from(fs::read(&c.ca)?))?;
    Ok(r)
}
fn cert(c: &Config) -> R<Vec<CertificateDer<'static>>> {
    Ok(vec![CertificateDer::from(fs::read(&c.certificate)?)])
}
fn key(c: &Config) -> R<PrivateKeyDer<'static>> {
    Ok(PrivateKeyDer::try_from(fs::read(&c.key)?).map_err(|_| "KEY_FORMAT")?)
}
fn client_config(c: &Config) -> R<Arc<ClientConfig>> {
    let mut x = ClientConfig::builder_with_protocol_versions(&[&rustls::version::TLS13])
        .with_root_certificates(roots(c)?)
        .with_client_auth_cert(cert(c)?, key(c)?)?;
    x.alpn_protocols = vec![ALPN.to_vec()];
    x.resumption = rustls::client::Resumption::disabled();
    x.enable_early_data = false;
    Ok(Arc::new(x))
}
fn server_config(c: &Config) -> R<Arc<ServerConfig>> {
    let verifier = rustls::server::WebPkiClientVerifier::builder(Arc::new(roots(c)?)).build()?;
    let mut x = ServerConfig::builder_with_protocol_versions(&[&rustls::version::TLS13])
        .with_client_cert_verifier(verifier)
        .with_single_cert(cert(c)?, key(c)?)?;
    x.alpn_protocols = vec![ALPN.to_vec()];
    x.max_early_data_size = 0;
    x.send_tls13_tickets = 0;
    Ok(Arc::new(x))
}
// Every socket operation shares a phase deadline, so byte-at-a-time input cannot
// reset the timeout. rustls owns all TLS record processing and verification.
struct Deadline {
    cancel: Option<Arc<std::sync::atomic::AtomicBool>>,
    socket: TcpStream,
    end: Instant,
}
impl Deadline {
    fn phase(&mut self, seconds: u64) {
        self.end = Instant::now() + Duration::from_secs(seconds)
    }
    fn remaining(&self) -> std::io::Result<Duration> {
        if self
            .cancel
            .as_ref()
            .is_some_and(|s| s.load(Ordering::SeqCst))
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionAborted,
                "node stopping",
            ));
        }
        self.end
            .checked_duration_since(Instant::now())
            .filter(|d| !d.is_zero())
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::TimedOut, "phase deadline"))
    }
}
impl Read for Deadline {
    fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> {
        loop {
            self.remaining()?;
            match self.socket.read(b) {
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(1))
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                result => return result,
            }
        }
    }
}
impl Write for Deadline {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        loop {
            self.remaining()?;
            match self.socket.write(b) {
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(1))
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                result => return result,
            }
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
fn send(w: &mut impl Write, v: &C) -> R<()> {
    let b = cbor::encode(v)?;
    need(!b.is_empty() && b.len() <= MAX, "FRAME_LIMIT")?;
    w.write_all(&(b.len() as u32).to_be_bytes())?;
    w.write_all(&b)?;
    w.flush()?;
    Ok(())
}
fn receive(r: &mut impl Read) -> R<C> {
    let mut n = [0; 4];
    r.read_exact(&mut n)?;
    let n = u32::from_be_bytes(n) as usize;
    need(n > 0 && n <= MAX, "FRAME_LIMIT")?;
    let mut b = vec![0; n];
    r.read_exact(&mut b)?;
    let v = cbor::decode(&b)?;
    need(cbor::encode(&v)? == b, "FRAME_CANONICAL")?;
    Ok(v)
}
fn receive_prefixed(r: &mut impl Read, first: u8) -> R<C> {
    let mut prefix = [first, 0, 0, 0];
    r.read_exact(&mut prefix[1..])?;
    let n = u32::from_be_bytes(prefix) as usize;
    need(n > 0 && n <= MAX, "FRAME_LIMIT")?;
    let mut raw = vec![0; n];
    r.read_exact(&mut raw)?;
    let v = cbor::decode(&raw)?;
    need(cbor::encode(&v)? == raw, "FRAME_CANONICAL")?;
    Ok(v)
}
fn arr(v: C) -> R<Vec<C>> {
    if let C::Array(a) = v {
        Ok(a)
    } else {
        Err("ARRAY".into())
    }
}
fn num(v: &C) -> R<u64> {
    if let C::Unsigned(n) = v {
        Ok(*n)
    } else {
        Err("INTEGER".into())
    }
}
fn blob(v: &C) -> R<&[u8]> {
    if let C::Bytes(b) = v {
        Ok(b)
    } else {
        Err("BYTES".into())
    }
}
fn b(s: &str) -> R<C> {
    Ok(C::Bytes(unhex(s)?))
}
fn hello(c: &Config, who: usize) -> R<C> {
    Ok(C::Array(vec![
        C::Unsigned(1),
        C::Unsigned(0),
        C::Bytes(PROFILE.as_bytes().to_vec()),
        b(&c.chain)?,
        b(&c.genesis)?,
        b(&c.peers[who].id)?,
    ]))
}
fn op(v: &Value) -> R<u64> {
    match v["body"]["kind"].as_str() {
        Some("status") => Ok(0),
        Some("prepare") => Ok(1),
        Some("accept") => Ok(2),
        Some("sync") => Ok(3),
        Some("run") => Ok(4),
        _ => Err("OPERATION".into()),
    }
}
fn envelope(c: &Config, peer: usize, seq: u64, kind: u64, status: u64, payload: Vec<u8>) -> R<C> {
    Ok(C::Array(vec![
        C::Unsigned(1),
        C::Unsigned(kind),
        b(&c.chain)?,
        b(&c.genesis)?,
        b(&c.peers[c.who].id)?,
        b(&c.peers[peer].id)?,
        C::Unsigned(seq),
        C::Unsigned(status),
        C::Bytes(payload),
    ]))
}
fn check<'a>(c: &Config, peer: usize, a: &'a [C], kind: u64) -> R<(u64, u64, &'a [u8])> {
    need(a.len() == 9, "ARITY")?;
    need(num(&a[0])? == 1 && num(&a[1])? == kind, "VERSION_TYPE")?;
    need(
        blob(&a[2])? == unhex(&c.chain)? && blob(&a[3])? == unhex(&c.genesis)?,
        "NETWORK_IDENTITY",
    )?;
    need(
        blob(&a[4])? == unhex(&c.peers[peer].id)? && blob(&a[5])? == unhex(&c.peers[c.who].id)?,
        "PEER_BINDING",
    )?;
    let seq = num(&a[6])?;
    need(seq > 0, "SEQUENCE")?;
    Ok((seq, num(&a[7])?, blob(&a[8])?))
}
fn call_worker(c: &Config, payload: &[u8]) -> R<Vec<u8>> {
    let mut s = UnixStream::connect(&c.worker)?;
    s.set_read_timeout(Some(Duration::from_secs(15)))?;
    s.set_write_timeout(Some(Duration::from_secs(5)))?;
    s.write_all(&(payload.len() as u32).to_be_bytes())?;
    s.write_all(payload)?;
    let mut n = [0; 4];
    s.read_exact(&mut n)?;
    let n = u32::from_be_bytes(n) as usize;
    need(n > 0 && n <= MAX, "WORKER_FRAME")?;
    let mut b = vec![0; n];
    s.read_exact(&mut b)?;
    Ok(b)
}
struct Reservations {
    path: PathBuf,
    value: Value,
    poisoned: bool,
}
impl Reservations {
    fn open(c: &Config) -> R<Self> {
        let path = c.state.join("receive.json");
        let value = if path.exists() {
            read_json(&path)?
        } else {
            json!({"chain":c.chain,"genesis":c.genesis,"peers":{}})
        };
        need(
            value["chain"] == c.chain && value["genesis"] == c.genesis,
            "STORE_NETWORK",
        )?;
        let peers = value["peers"].as_object().ok_or("STORE_SCHEMA")?;
        need(peers.len() <= 6, "STORE_PEERS")?;
        for (id, entry) in peers {
            need(c.peers.iter().any(|p| p.id == *id), "STORE_UNKNOWN_PEER")?;
            need(
                entry["seq"].as_u64().is_some_and(|x| x > 0),
                "STORE_SEQUENCE",
            )?;
            unhex(entry["digest"].as_str().ok_or("STORE_DIGEST")?)?;
        }
        Ok(Self {
            path,
            value,
            poisoned: false,
        })
    }
    fn reserve(&mut self, id: &str, seq: u64, digest: &str) -> R<u64> {
        need(!self.poisoned, "STORE_POISONED")?;
        let old = &self.value["peers"][id];
        if let Some(n) = old["seq"].as_u64() {
            if seq < n {
                return Ok(1);
            }
            if seq == n {
                return Ok(if old["digest"] == digest { 1 } else { 2 });
            }
        }
        self.value["peers"][id] = json!({"seq":seq,"digest":digest});
        self.poisoned = true;
        atomic(&self.path, &self.value)?;
        self.poisoned = false;
        Ok(0)
    }
}
fn handle(
    c: &Config,
    sc: Arc<ServerConfig>,
    sock: TcpStream,
    store: &Mutex<Reservations>,
    limits: &Arc<Limits>,
    mut permit: Permit,
) -> R<()> {
    sock.set_nonblocking(true)?;
    let mut stream = StreamOwned::new(
        ServerConnection::new(sc)?,
        Deadline {
            cancel: Some(limits.stopping.clone()),
            socket: sock,
            end: Instant::now() + Duration::from_secs(5),
        },
    );
    stream.conn.set_buffer_limit(Some(MAX));
    while stream.conn.is_handshaking() {
        stream
            .conn
            .complete_io(&mut stream.sock)
            .map_err(|e| format!("TLS_IO: {e}"))?;
    }
    need(stream.conn.alpn_protocol() == Some(ALPN), "ALPN")?;
    let remote = peer_id(
        stream
            .conn
            .peer_certificates()
            .ok_or("CERTIFICATE")?
            .first()
            .ok_or("CERTIFICATE")?
            .as_ref(),
    )?;
    let peer = c
        .peers
        .iter()
        .position(|p| p.id == remote)
        .ok_or("UNKNOWN_PEER")?;
    need(peer != c.who, "SELF_PEER")?;
    permit.authenticate(peer)?;
    stream.sock.phase(5);
    need(
        receive(&mut stream)? == hello(c, peer)?,
        "HELLO_NETWORK_IDENTITY",
    )?;
    send(&mut stream, &hello(c, c.who)?)?;
    loop {
        if c.state.join("stop").exists() {
            break;
        }
        stream.sock.phase(30);
        let mut first = [0];
        stream.read_exact(&mut first)?;
        stream.sock.phase(5);
        limits.frame(peer)?;
        let _buffer = limits.buffer()?;
        let frame = receive_prefixed(&mut stream, first[0])?;
        let a = arr(frame)?;
        let kind = num(a.get(1).ok_or("ARITY")?)?;
        need(kind == 1 || kind == 3, "VERSION_TYPE")?;
        let (seq, operation, payload) = check(c, peer, &a, kind)?;
        let mut original = a.clone();
        original[1] = C::Unsigned(1);
        let raw = cbor::encode(&C::Array(original))?;
        let request: Value = serde_json::from_slice(payload)?;
        need(canonical(&request)? == payload, "APPLICATION_CANONICAL")?;
        need(
            request["signer"].as_u64() == Some(peer as u64) && op(&request)? == operation,
            "SIGNER_OPERATION",
        )?;
        need(operation != 4 || peer == 5, "CONTROL_ROLE")?;
        let mut digest = b"TESTNET_NATIVE_TRANSPORT_V0_REQUEST\0".to_vec();
        digest.extend_from_slice(&raw);
        // One mutex spans reservation + application dispatch: same-sender requests
        // cannot overtake each other across simultaneous connections.
        let queue_deadline = Instant::now() + Duration::from_secs(5);
        let mut locked = loop {
            match store.try_lock() {
                Ok(guard) => break guard,
                Err(std::sync::TryLockError::Poisoned(_)) => return Err("STORE_LOCK".into()),
                Err(std::sync::TryLockError::WouldBlock) => {
                    need(Instant::now() < queue_deadline, "APPLICATION_BUSY")?;
                    thread::sleep(Duration::from_millis(1));
                }
            }
        };
        need(!locked.poisoned, "STORE_POISONED")?;
        let digest = hash(&digest);
        let mut status;
        let answer;
        if kind == 3 {
            let old = &locked.value["peers"][&remote];
            let n = old["seq"].as_u64().unwrap_or(0);
            if seq > n {
                status = 5;
                answer = canonical(&json!({"error":"NOT_RECEIVED"}))?;
            } else if seq < n || old["digest"] != digest {
                status = if seq < n { 1 } else { 2 };
                answer = canonical(&json!({"error":"RETIRED_OR_CONFLICT"}))?;
            } else {
                let query = canonical(&json!({"native_reconcile":request}))?;
                match call_worker(c, &query) {
                    Ok(raw) => {
                        let v: Value = serde_json::from_slice(&raw)?;
                        status = if v["error"] == "STOP_RECONCILIATION_REQUIRED" {
                            3
                        } else {
                            0
                        };
                        answer = raw;
                    }
                    Err(_) => {
                        status = 3;
                        answer = canonical(&json!({"error":"APPLICATION_UNCERTAIN"}))?;
                    }
                }
            }
        } else {
            status = locked.reserve(&remote, seq, &digest)?;
            answer = if status == 0 {
                match call_worker(c, payload) {
                    Ok(raw) => raw,
                    Err(_) => {
                        status = 3;
                        canonical(&json!({"error":"APPLICATION_UNCERTAIN"}))?
                    }
                }
            } else {
                canonical(&json!({"error":if status==1{"ALREADY_SEEN"}else{"SEQUENCE_CONFLICT"}}))?
            };
        }
        drop(locked);
        stream.sock.phase(5);
        send(&mut stream, &envelope(c, peer, seq, 2, status, answer)?)?;
        log(
            "request",
            if status == 0 {
                "dispatched"
            } else {
                "rejected"
            },
        );
    }
    stream.conn.send_close_notify();
    let _ = stream.flush();
    Ok(())
}
fn serve(c: Config) -> R<()> {
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(c.state.join("receive.lock"))?;
    lock.try_lock()?;
    let sc = server_config(&c)?;
    let store = Arc::new(Mutex::new(Reservations::open(&c)?));
    let listener = TcpListener::bind(c.peers[c.who].address)?;
    listener.set_nonblocking(true)?;
    let limits = Limits::new();
    let count = Arc::new(AtomicUsize::new(0));
    let dialer = pool::start(c.clone(), limits.clone())?;
    let mut tasks = Vec::new();
    log("ready", "TESTNET ONLY");
    while !c.state.join("stop").exists() {
        match listener.accept() {
            Ok((socket, _)) => {
                if count.load(Ordering::SeqCst) >= 10 {
                    drop(socket);
                    log("busy", "connection limit");
                    continue;
                }
                let permit = match limits.pending() {
                    Ok(p) => p,
                    Err(e) => {
                        log("busy", &e.to_string());
                        drop(socket);
                        continue;
                    }
                };
                let ll = limits.clone();
                count.fetch_add(1, Ordering::SeqCst);
                let cc = c.clone();
                let sc = sc.clone();
                let ss = store.clone();
                let n = count.clone();
                tasks.push(thread::spawn(move || {
                    if let Err(e) = handle(&cc, sc, socket, &ss, &ll, permit) {
                        log("rejected", &e.to_string());
                    }
                    n.fetch_sub(1, Ordering::SeqCst);
                }));
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(10))
            }
            Err(e) => return Err(e.into()),
        }
        let mut i = 0;
        while i < tasks.len() {
            if tasks[i].is_finished() {
                let _ = tasks.swap_remove(i).join();
            } else {
                i += 1;
            }
        }
    }
    drop(listener);
    limits.stopping.store(true, Ordering::SeqCst);
    for t in tasks {
        let _ = t.join();
    }
    dialer.join().map_err(|_| "DIALER_THREAD")??;
    log("shutdown", "drained");
    Ok(())
}
type ClientStream = StreamOwned<ClientConnection, Deadline>;
fn connect(c: &Config, peer: usize) -> R<ClientStream> {
    let sock = TcpStream::connect_timeout(&c.peers[peer].address, Duration::from_secs(3))?;
    sock.set_nodelay(true)?;
    sock.set_nonblocking(true)?;
    let conn = ClientConnection::new(
        client_config(c)?,
        ServerName::try_from(c.peers[peer].name.clone())?,
    )?;
    let mut stream = StreamOwned::new(
        conn,
        Deadline {
            cancel: None,
            socket: sock,
            end: Instant::now() + Duration::from_secs(5),
        },
    );
    stream.conn.set_buffer_limit(Some(MAX));
    while stream.conn.is_handshaking() {
        stream
            .conn
            .complete_io(&mut stream.sock)
            .map_err(|e| format!("TLS_IO: {e}"))?;
    }
    need(stream.conn.alpn_protocol() == Some(ALPN), "ALPN")?;
    need(
        peer_id(stream.conn.peer_certificates().ok_or("CERTIFICATE")?[0].as_ref())?
            == c.peers[peer].id,
        "UNKNOWN_PEER",
    )?;
    stream.sock.phase(5);
    send(&mut stream, &hello(c, c.who)?)?;
    need(
        receive(&mut stream)? == hello(c, peer)?,
        "HELLO_NETWORK_IDENTITY",
    )?;
    log("session_open", &format!("peer {peer}"));
    Ok(stream)
}
fn exchange(
    c: &Config,
    peer: usize,
    seq: u64,
    request: &C,
    sent: &mut bool,
    stream: &mut ClientStream,
) -> R<Vec<u8>> {
    stream.sock.phase(5);
    *sent = true;
    send(stream, request)?;
    stream.sock.phase(15);
    let a = arr(receive(stream)?)?;
    let (n, status, payload) = check(c, peer, &a, 2)?;
    need(n == seq, "RESPONSE_SEQUENCE")?;
    if status != 0 {
        return Err(match status {
            5 => "NOT_RECEIVED".to_owned(),
            1 => "ALREADY_SEEN_RECONCILE".to_owned(),
            _ => format!("REMOTE_REJECTION: {}", String::from_utf8_lossy(payload)),
        }
        .into());
    }
    Ok(payload.to_vec())
}
fn attempt(
    c: &Config,
    peer: usize,
    seq: u64,
    frame: &C,
    sent: &mut bool,
    session: &mut Option<ClientStream>,
) -> R<Vec<u8>> {
    if session.is_none() {
        *session = Some(connect(c, peer)?);
    }
    let result = exchange(
        c,
        peer,
        seq,
        frame,
        sent,
        session.as_mut().ok_or("SESSION")?,
    );
    if result.is_err() {
        *session = None;
    }
    result
}
fn request(
    c: &Config,
    peer: usize,
    payload: Vec<u8>,
    recovery: bool,
    session: &mut Option<ClientStream>,
) -> R<Vec<u8>> {
    need(peer < 6 && peer != c.who, "PEER")?;
    let path = c.state.join(format!("send-{peer}.json"));
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(path.with_extension("lock"))?;
    lock.try_lock()?;
    let mut state = if path.exists() {
        read_json(&path)?
    } else {
        json!({"chain":c.chain,"genesis":c.genesis,"seq":0,"pending":null})
    };
    need(
        state["chain"] == c.chain && state["genesis"] == c.genesis,
        "STORE_NETWORK",
    )?;
    let had_pending = !state["pending"].is_null();
    if recovery && !had_pending {
        need(
            state["last"]["sequence"] == state["seq"] && !state["last"]["response"].is_null(),
            "NO_PENDING_OR_COMPLETED_REQUEST",
        )?;
        return canonical(&state["last"]["response"]);
    }

    let payload = if recovery {
        need(had_pending, "NO_PENDING_REQUEST")?;
        canonical(&state["pending"])?
    } else {
        payload
    };
    let value: Value = serde_json::from_slice(&payload)?;
    need(canonical(&value)? == payload, "APPLICATION_CANONICAL")?;
    need(
        value["signer"].as_u64() == Some(c.who as u64),
        "LOCAL_SIGNER",
    )?;
    let seq = if !state["pending"].is_null() {
        need(state["pending"] == value, "PENDING_REQUIRES_RECONCILIATION")?;
        state["seq"].as_u64().ok_or("SEQUENCE")?
    } else {
        let seq = state["seq"]
            .as_u64()
            .ok_or("SEQUENCE")?
            .checked_add(1)
            .ok_or("SEQUENCE_EXHAUSTED")?;
        state["seq"] = json!(seq);
        state["pending"] = value.clone();
        atomic(&path, &state)?;
        seq
    };
    let mut frame = envelope(
        c,
        peer,
        seq,
        if recovery { 3 } else { 1 },
        op(&value)?,
        payload,
    )?;
    let mut sent = false;
    let mut error: String = "CONNECT".into();
    for i in 0..5 {
        match attempt(c, peer, seq, &frame, &mut sent, session) {
            Ok(response) => {
                state["last"] = json!({"sequence":seq,"request_digest":hash(&canonical(&value)?),"response":serde_json::from_slice::<Value>(&response)?});
                state["pending"] = Value::Null;
                atomic(&path, &state)?;
                return Ok(response);
            }
            Err(e) => {
                error = e.to_string();
                log("retry", &error);
                if recovery && error == "NOT_RECEIVED" {
                    if let C::Array(a) = &mut frame {
                        a[1] = C::Unsigned(1);
                    }
                    continue;
                }
                if error.contains("RECONCILE") || error.contains("REJECTION") {
                    break;
                }
            }
        }
        if i < 4 {
            let mut byte = [0];
            File::open("/dev/urandom")?.read_exact(&mut byte)?;
            let base = 250_u64 * (1 << i);
            thread::sleep(Duration::from_millis(
                base + base * u64::from(byte[0]) / 1024,
            ));
        }
    }
    if !sent && !had_pending {
        state["pending"] = Value::Null;
        atomic(&path, &state)?;
    }
    Err(error.into())
}
fn run() -> R<()> {
    let _ = rustls::crypto::ring::default_provider().install_default();
    let args: Vec<String> = std::env::args().collect();
    need(args.len() >= 3, "usage: serve|request|stop CONFIG [PEER]")?;
    let c = load(Path::new(&args[2]))?;
    match args[1].as_str() {
        "serve" => serve(c),
        "pending" => {
            need(args.len() == 4, "peer argument")?;
            let peer: usize = args[3].parse()?;
            need(peer < 6 && peer != c.who, "PEER")?;
            let state = read_json(&c.state.join(format!("send-{peer}.json")))?;
            println!(
                "{}",
                json!({"sequence":state["seq"],"pending":!state["pending"].is_null(),"request_digest":if state["pending"].is_null(){state["last"]["request_digest"].clone()}else{json!(hash(&canonical(&state["pending"])?))},"kind":state["pending"]["body"]["kind"]})
            );
            Ok(())
        }
        "stop" => {
            File::create(c.state.join("stop"))?.sync_all()?;
            Ok(())
        }
        "request" | "recover" => {
            need(args.len() == 4, "peer argument")?;
            let peer = args[3].parse()?;
            let mut raw = Vec::new();
            if args[1] == "request" {
                std::io::stdin()
                    .take((MAX + 1) as u64)
                    .read_to_end(&mut raw)?;
            }
            need(raw.len() <= MAX, "FRAME_LIMIT")?;
            let response = if c.state.join("dialer.sock").exists() {
                pool::forward(&c, peer, raw, args[1] == "recover")?
            } else {
                request(&c, peer, raw, args[1] == "recover", &mut None)?
            };
            std::io::stdout().write_all(&response)?;
            Ok(())
        }
        _ => Err("operation".into()),
    }
}
fn main() {
    if let Err(e) = run() {
        log("error", &e.to_string());
        std::process::exit(1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frozen_native_vectors() {
        let vectors = include_str!("../../../vectors/testnet-native/v0/approved.tsv");
        for line in vectors.lines() {
            let (name, hexed) = line.split_once('\t').unwrap();
            let raw: Vec<u8> = (0..hexed.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&hexed[i..i + 2], 16).unwrap())
                .collect();
            let value = cbor::decode(&raw).unwrap();
            assert_eq!(cbor::encode(&value).unwrap(), raw);
            let a = arr(value).unwrap();
            assert_eq!(num(&a[0]).unwrap(), 1);
            assert_eq!(a.len(), if name == "hello" { 6 } else { 9 });
            assert_eq!(
                blob(&a[if name == "hello" { 3 } else { 2 }]).unwrap(),
                &(0..32).collect::<Vec<u8>>()
            );
        }
    }
    #[test]
    fn frame_bounds_and_truncation() {
        for raw in [vec![0, 1, 0, 1], vec![0, 0, 0, 2, 0x81]] {
            assert!(receive(&mut &raw[..]).is_err());
        }
    }
    #[test]
    fn rejects_noncanonical_cbor() {
        assert!(receive(&mut &[0, 0, 0, 2, 0x18, 1][..]).is_err());
    }
    #[test]
    fn private_endpoint_guard() {
        assert!(!allowed("8.8.8.8".parse().unwrap()));
        assert!(!allowed("0.0.0.0".parse().unwrap()));
        assert!(allowed("127.0.0.1".parse().unwrap()));
        assert!(allowed("10.2.3.4".parse().unwrap()));
        assert!(allowed("100.64.0.1".parse().unwrap()));
        assert!(!allowed("100.63.255.255".parse().unwrap()));
        assert!(!allowed("100.128.0.1".parse().unwrap()));
    }
}
