//! Loopback-only Q1 LOCALNET v0 daemon. No production activation exists.

mod cli;
mod wire;

use q1_localnet::{
    block::{Attestation, Certificate, CertifiedBlock, Proposal},
    genesis::Genesis,
    store::PersistentChain,
};
use q1_primitives::{
    Ed25519PrivateKey,
    cbor::{self, Value},
    ed25519,
};
use q1_protocol_types::{
    block_body::BlockBodyV1, transfer::SignedTransferV1, types::ParticipantId,
};
use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    net::{SocketAddr, TcpListener},
    path::Path,
    sync::{
        Arc, Mutex, MutexGuard,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
    time::Duration,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
const MAX_WORKERS: usize = 16;

struct Node {
    genesis: Genesis,
    participant: ParticipantId,
    key: Ed25519PrivateKey,
    store: Mutex<PersistentChain>,
    submissions: Mutex<()>,
    producer: SocketAddr,
    voters: Vec<SocketAddr>,
}
impl Node {
    fn store(&self) -> Result<MutexGuard<'_, PersistentChain>> {
        self.store.lock().map_err(|_| "store lock poisoned".into())
    }
    fn status(&self) -> Result<Vec<u8>> {
        let store = self.store()?;
        let chain = store.chain();
        let state = chain.state();
        // Exact full snapshot permits observers to compare all balances/nonces.
        Ok(cbor::encode(&Value::Array(vec![
            Value::Unsigned(state.ledger().height().get()),
            chain
                .tip()
                .map_or(Value::Null, |id| Value::Bytes(id.as_bytes().to_vec())),
            Value::Bytes(state.root()?.as_bytes().to_vec()),
            Value::Bytes(state.encode_canonical()?),
        ]))?)
    }
    fn commit(&self, block: &CertifiedBlock) -> Result<()> {
        let mut store = self.store()?;
        let height = block.proposal().height().get();
        let current = store.chain().state().ledger().height().get();
        if height <= current {
            let index = usize::try_from(height.checked_sub(1).ok_or("height zero")?)?;
            let previous = store
                .chain()
                .history()
                .get(index)
                .ok_or("missing history")?;
            if previous.proposal().id()? != block.proposal().id()? {
                return Err("conflicting committed proposal".into());
            }
            block
                .certificate()
                .verify(&self.genesis, block.proposal())?;
            return Ok(());
        }
        store.commit(block)?;
        Ok(())
    }
    fn submit(&self, bytes: Vec<u8>) -> Result<Vec<u8>> {
        if self.participant != self.genesis.producer() {
            return wire::request(
                self.producer,
                self.genesis.id().as_bytes(),
                wire::SUBMIT,
                bytes,
            );
        }
        // Reject concurrent work rather than allowing unbounded waiting workers.
        let _submission = self
            .submissions
            .try_lock()
            .map_err(|_| "producer busy; retry submission")?;
        let transfer = SignedTransferV1::decode_canonical(&bytes)?;
        let proposal = self
            .store()?
            .chain()
            .propose(BlockBodyV1::new(vec![transfer])?, &self.key)?;
        let encoded = proposal.encode_canonical()?;
        let mut votes = BTreeMap::new();
        for peer in &self.voters {
            let response = wire::request(
                *peer,
                self.genesis.id().as_bytes(),
                wire::PROPOSE,
                encoded.clone(),
            );
            if let Ok(response) = response
                && let Ok(vote) = Attestation::decode_canonical(&response)
                && vote.verify(&self.genesis, &proposal).is_ok()
            {
                votes.insert(vote.voter(), vote);
            }
        }
        let certificate =
            Certificate::new(&self.genesis, &proposal, votes.into_values().collect())?;
        let block = CertifiedBlock::new(proposal, certificate);
        self.commit(&block)?;
        let encoded = block.encode_canonical()?;
        // Durably finalized locally before broadcast. Disconnected voters recover by SYNC.
        for peer in &self.voters {
            let _ = wire::request(
                *peer,
                self.genesis.id().as_bytes(),
                wire::COMMIT,
                encoded.clone(),
            );
        }
        self.status()
    }
    fn handle(&self, op: u64, payload: Vec<u8>) -> Result<Vec<u8>> {
        match op {
            wire::STATUS if payload.is_empty() => self.status(),
            wire::SUBMIT => self.submit(payload),
            wire::PROPOSE => {
                let proposal = Proposal::decode_canonical(&payload)?;
                self.store()?.reserve_vote(self.participant, &proposal)?;
                Ok(
                    Attestation::sign(&self.genesis, &proposal, self.participant, &self.key)?
                        .encode_canonical()?,
                )
            }
            wire::COMMIT => {
                self.commit(&CertifiedBlock::decode_canonical(&payload)?)?;
                Ok(vec![])
            }
            wire::SYNC => {
                let Value::Unsigned(height) = cbor::decode(&payload)? else {
                    return Err("expected requested height".into());
                };
                let index = usize::try_from(height.checked_sub(1).ok_or("height zero")?)?;
                let store = self.store()?;
                match store.chain().history().get(index) {
                    Some(block) => Ok(block.encode_canonical()?),
                    None => Ok(vec![]),
                }
            }
            _ => Err("unknown opcode or malformed request".into()),
        }
    }
    fn catch_up(&self) -> Result<()> {
        let next = self
            .store()?
            .chain()
            .state()
            .ledger()
            .height()
            .checked_increment()?
            .get();
        let bytes = wire::request(
            self.producer,
            self.genesis.id().as_bytes(),
            wire::SYNC,
            cbor::encode(&Value::Unsigned(next))?,
        )?;
        if !bytes.is_empty() {
            self.commit(&CertifiedBlock::decode_canonical(&bytes)?)?;
        }
        Ok(())
    }
}

fn read_bounded(path: &Path, max: usize) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    fs::File::open(path)?
        .take((max + 1) as u64)
        .read_to_end(&mut bytes)?;
    if bytes.len() > max {
        return Err("file exceeds local size limit".into());
    }
    Ok(bytes)
}
fn participant(text: &str) -> Result<ParticipantId> {
    if text.len() != 64 || !text.is_ascii() {
        return Err("participant must be 32-byte hex".into());
    }
    let mut bytes = [0; 32];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&text[index * 2..index * 2 + 2], 16)?;
    }
    Ok(ParticipantId::from_bytes(bytes))
}
fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().is_none_or(|arg| {
        !matches!(
            arg.as_str(),
            "--genesis"
                | "--key-file"
                | "--participant"
                | "--data-dir"
                | "--listen"
                | "--producer"
                | "--voters"
        )
    }) {
        return cli::run(&args);
    }
    let names = [
        "--genesis",
        "--key-file",
        "--participant",
        "--data-dir",
        "--listen",
        "--producer",
        "--voters",
    ];
    if args.len() != names.len() * 2 {
        return Err("usage: q1-node --genesis FILE --key-file RAW32 --participant HEX --data-dir DIR --listen LOOPBACK:PORT --producer LOOPBACK:PORT --voters ADDR,ADDR,ADDR".into());
    }
    let mut config = BTreeMap::new();
    for pair in args.chunks_exact(2) {
        if !names.contains(&pair[0].as_str())
            || config.insert(pair[0].as_str(), pair[1].as_str()).is_some()
        {
            return Err("unknown or repeated argument".into());
        }
    }
    let genesis = Genesis::decode_canonical(&read_bounded(
        Path::new(config["--genesis"]),
        cbor::MAX_OBJECT_SIZE,
    )?)?;
    let participant = participant(config["--participant"])?;
    let key_path = Path::new(config["--key-file"]);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if fs::metadata(key_path)?.permissions().mode() & 0o077 != 0 {
            return Err("key file must have no group/other permissions (chmod 600)".into());
        }
    }
    let key = Ed25519PrivateKey::from_seed(
        read_bounded(key_path, 32)?
            .try_into()
            .map_err(|_| "key file must contain exactly 32 bytes")?,
    );
    let public = if participant == genesis.producer() {
        genesis.producer_public_key()
    } else {
        genesis.voter_public_key(participant)?
    };
    if ed25519::public_key(&key)? != public {
        return Err("participant key does not match genesis role".into());
    }
    let listen = wire::address(config["--listen"])?;
    let producer = wire::address(config["--producer"])?;
    let voters = config["--voters"]
        .split(',')
        .map(wire::address)
        .collect::<Result<Vec<_>>>()?;
    let mut unique = voters.clone();
    unique.sort();
    unique.dedup();
    if unique.len() != 3
        || voters.len() != 3
        || voters.contains(&producer)
        || ((participant == genesis.producer()) != (listen == producer))
        || (participant != genesis.producer() && !voters.contains(&listen))
    {
        return Err("expected distinct fixed producer and three loopback voter endpoints".into());
    }
    let listener = TcpListener::bind(listen)?;
    let directory = Path::new(config["--data-dir"]);
    let store = if directory.try_exists()? {
        PersistentChain::open(directory, &genesis)?
    } else {
        PersistentChain::create(directory, &genesis)?
    };
    let node = Arc::new(Node {
        genesis,
        participant,
        key,
        store: Mutex::new(store),
        submissions: Mutex::new(()),
        producer,
        voters,
    });
    if node.participant != node.genesis.producer() {
        let sync_node = Arc::clone(&node);
        thread::spawn(move || {
            loop {
                if let Err(error) = sync_node.catch_up() {
                    eprintln!("catch-up pending: {error}");
                }
                thread::sleep(Duration::from_millis(250));
            }
        });
    }
    println!("LOCALNET_V0 listening {listen}");
    let workers = Arc::new(AtomicUsize::new(0));
    for stream in listener.incoming() {
        let mut stream = stream?;
        if workers
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| {
                (n < MAX_WORKERS).then_some(n + 1)
            })
            .is_err()
        {
            continue;
        }
        let node = Arc::clone(&node);
        let workers = Arc::clone(&workers);
        thread::spawn(move || {
            struct Worker(Arc<AtomicUsize>);
            impl Drop for Worker {
                fn drop(&mut self) {
                    self.0.fetch_sub(1, Ordering::Relaxed);
                }
            }
            let _worker = Worker(workers);
            let result = wire::configure(&stream)
                .and_then(|()| wire::read(&mut stream, node.genesis.id().as_bytes()))
                .and_then(|(op, payload)| node.handle(op, payload).map(|response| (op, response)));
            let (op, payload) = match result {
                Ok(response) => response,
                Err(error) => (wire::ERROR, error.to_string().into_bytes()),
            };
            let _ = wire::write(&mut stream, node.genesis.id().as_bytes(), op, payload);
        });
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("q1-node: {error}");
        std::process::exit(1);
    }
}
