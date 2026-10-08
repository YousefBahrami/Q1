//! Private local dialer with one reusable outgoing session per enrolled peer.
use super::*;
use std::os::unix::net::UnixListener;
fn read(s: &mut UnixStream) -> R<Value> {
    let mut n = [0; 4];
    s.read_exact(&mut n)?;
    let n = u32::from_be_bytes(n) as usize;
    need(n > 0 && n <= MAX, "LOCAL_POOL_FRAME")?;
    let mut b = vec![0; n];
    s.read_exact(&mut b)?;
    let v: Value = serde_json::from_slice(&b)?;
    need(canonical(&v)? == b, "LOCAL_POOL_CANONICAL")?;
    Ok(v)
}
fn write(s: &mut UnixStream, v: &Value) -> R<()> {
    let b = canonical(v)?;
    need(b.len() <= MAX, "LOCAL_POOL_FRAME")?;
    s.write_all(&(b.len() as u32).to_be_bytes())?;
    s.write_all(&b)?;
    Ok(())
}
pub(super) fn forward(c: &Config, peer: usize, payload: Vec<u8>, recovery: bool) -> R<Vec<u8>> {
    let mut s = UnixStream::connect(c.state.join("dialer.sock"))?;
    s.set_read_timeout(Some(Duration::from_secs(120)))?;
    s.set_write_timeout(Some(Duration::from_secs(5)))?;
    let value = if recovery {
        Value::Null
    } else {
        serde_json::from_slice(&payload)?
    };
    write(
        &mut s,
        &json!({"peer":peer,"recovery":recovery,"request":value}),
    )?;
    let v = read(&mut s)?;
    if let Some(e) = v["pool_error"].as_str() {
        return Err(e.to_owned().into());
    }
    canonical(&v["response"])
}
pub(super) fn start(c: Config, limits: Arc<Limits>) -> R<thread::JoinHandle<R<()>>> {
    let path = c.state.join("dialer.sock");
    if path.exists() {
        fs::remove_file(&path)?;
    }
    let listener = UnixListener::bind(&path)?;
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
    listener.set_nonblocking(true)?;
    let sessions: Arc<[Mutex<Option<ClientStream>>; 6]> =
        Arc::new(std::array::from_fn(|_| Mutex::new(None)));
    Ok(thread::spawn(move || {
        let count = Arc::new(AtomicUsize::new(0));
        let mut tasks = Vec::new();
        while !c.state.join("stop").exists() {
            match listener.accept() {
                Ok((mut socket, _)) => {
                    if count.load(Ordering::SeqCst) >= 6 {
                        drop(socket);
                        log("busy", "local dialer cap");
                        continue;
                    }
                    let buffer = match limits.buffer() {
                        Ok(b) => b,
                        Err(_) => {
                            drop(socket);
                            continue;
                        }
                    };
                    count.fetch_add(1, Ordering::SeqCst);
                    let n = count.clone();
                    let cc = c.clone();
                    let pool = sessions.clone();
                    tasks.push(thread::spawn(move || {
                        let _buffer = buffer;
                        let result = (|| -> R<Value> {
                            // Darwin may inherit O_NONBLOCK from the Unix listener.
                            socket.set_nonblocking(false)?;
                            socket.set_read_timeout(Some(Duration::from_secs(5)))?;
                            socket.set_write_timeout(Some(Duration::from_secs(5)))?;
                            let v = read(&mut socket)?;
                            let peer = v["peer"].as_u64().ok_or("LOCAL_PEER")? as usize;
                            need(peer < 6 && peer != cc.who, "PEER")?;
                            let recovery = v["recovery"].as_bool().ok_or("LOCAL_RECOVERY")?;
                            let mut session =
                                pool[peer].try_lock().map_err(|_| "PEER_BACKPRESSURE")?;
                            let bytes = request(
                                &cc,
                                peer,
                                canonical(&v["request"])?,
                                recovery,
                                &mut session,
                            )?;
                            Ok(json!({"response":serde_json::from_slice::<Value>(&bytes)?}))
                        })();
                        let answer = match result {
                            Ok(v) => v,
                            Err(e) => {
                                log("pool_error", &e.to_string());
                                json!({"pool_error":e.to_string()})
                            }
                        };
                        if let Err(e) = write(&mut socket, &answer) {
                            log("pool_reply_error", &e.to_string());
                        }
                        n.fetch_sub(1, Ordering::SeqCst);
                    }));
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(5))
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
        for t in tasks {
            let _ = t.join();
        }
        fs::remove_file(path)?;
        Ok(())
    }))
}
