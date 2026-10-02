//! Explicit LOCALNET wallet and transaction tools; plaintext test custody only.
use crate::{Result, read_bounded, wire};
use q1_localnet::{genesis::Genesis, state::StateSnapshot};
use q1_primitives::{
    Address, Amount, Ed25519PrivateKey, Height, Network,
    cbor::{self, Value},
    ed25519,
};
use q1_protocol_types::{
    address::AddressEnvelope,
    transfer::{SignedTransferV1, TransferBodyV1, TransferFields},
    types::{FeeLimit, Nonce},
};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

pub fn help() -> &'static str {
    "Q1 LOCALNET v0 — TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS\n\
     q1-node --version\n\
     q1-node wallet-new KEY_FILE\n\
     q1-node wallet-address KEY_FILE\n\
     q1-node status GENESIS_FILE LOOPBACK:PORT\n\
     q1-node transfer-sign GENESIS_FILE KEY_FILE RECIPIENT AMOUNT NONCE FROM_HEIGHT UNTIL_HEIGHT OUTPUT_FILE\n\
     q1-node transfer-submit GENESIS_FILE LOOPBACK:PORT SIGNED_FILE\n\
     q1-node --genesis FILE --key-file RAW32 --participant HEX --data-dir DIR --listen LOOPBACK:PORT --producer LOOPBACK:PORT --voters ADDR,ADDR,ADDR\n\
     Wallet files are unencrypted local test seeds. Never reuse them elsewhere."
}
fn key(path: &str) -> Result<Ed25519PrivateKey> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if std::fs::metadata(path)?.permissions().mode() & 0o077 != 0 {
            return Err("key file must exclude group/other access (chmod 600)".into());
        }
    }
    Ok(Ed25519PrivateKey::from_seed(
        read_bounded(Path::new(path), 32)?
            .try_into()
            .map_err(|_| "expected 32-byte key file")?,
    ))
}
fn write_new(path: &str, bytes: &[u8]) -> Result<()> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}
fn genesis(path: &str) -> Result<Genesis> {
    Ok(Genesis::decode_canonical(&read_bounded(
        Path::new(path),
        cbor::MAX_OBJECT_SIZE,
    )?)?)
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn print_status(bytes: &[u8], genesis: &Genesis) -> Result<()> {
    let Value::Array(fields) = cbor::decode(bytes)? else {
        return Err("invalid status array".into());
    };
    let [
        Value::Unsigned(height),
        tip,
        Value::Bytes(root),
        Value::Bytes(snapshot),
    ] = fields.as_slice()
    else {
        return Err("invalid status fields".into());
    };
    let snapshot = StateSnapshot::decode_canonical(snapshot, genesis)?;
    if snapshot.root()?.as_bytes().as_slice() != root || snapshot.ledger().height().get() != *height
    {
        return Err("inconsistent status commitment".into());
    }
    let tip = match tip {
        Value::Null if *height == 0 => "genesis".to_string(),
        Value::Bytes(id) if id.len() == 32 && *height > 0 => hex(id),
        _ => return Err("invalid status tip".into()),
    };
    println!(
        "height={height}\nblock_id={tip}\nstate_root={}\nreward_pool={}\ntotal_supply={}",
        hex(root),
        snapshot.ledger().reward_pool().get(),
        snapshot.ledger().total_supply().get()
    );
    for (address, account) in snapshot.ledger().accounts() {
        println!(
            "account={} balance={} nonce={}",
            address.to_address(Network::Localnet)?.encode(),
            account.balance().get(),
            account.nonce().get()
        );
    }
    Ok(())
}
pub fn run(args: &[String]) -> Result<()> {
    let command = args.iter().map(String::as_str).collect::<Vec<_>>();
    match command.as_slice() {
        ["--help"] => println!("{}", help()),
        ["--version"] => println!("q1-node {} (LOCALNET_V0 ONLY)", env!("CARGO_PKG_VERSION")),
        ["wallet-new", path] => {
            // OS randomness on the supported macOS/Linux platforms. No fallback seed.
            let mut seed = [0; 32];
            File::open("/dev/urandom")?.read_exact(&mut seed)?;
            write_new(path, &seed)?;
            let address = Address::from_public_key(
                Network::Localnet,
                ed25519::public_key(&Ed25519PrivateKey::from_seed(seed))?,
            )?;
            eprintln!(
                "TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS; unencrypted wallet created"
            );
            println!("{}", address.encode());
        }
        ["wallet-address", path] => println!(
            "{}",
            Address::from_public_key(Network::Localnet, ed25519::public_key(&key(path)?)?)?
                .encode()
        ),
        ["status", manifest, endpoint] => {
            let genesis = genesis(manifest)?;
            print_status(
                &wire::request(
                    wire::address(endpoint)?,
                    genesis.id().as_bytes(),
                    wire::STATUS,
                    vec![],
                )?,
                &genesis,
            )?;
        }
        [
            "transfer-sign",
            manifest,
            path,
            recipient,
            amount,
            nonce,
            from,
            until,
            output,
        ] => {
            let genesis = genesis(manifest)?;
            let key = key(path)?;
            let amount = Amount::new(amount.parse()?);
            if amount.get() == 0 {
                return Err("transfer amount must be positive base units".into());
            }
            let transfer = SignedTransferV1::sign(
                TransferBodyV1::new(TransferFields {
                    chain_id: genesis.chain_id(),
                    sender_public_key: ed25519::public_key(&key)?,
                    recipient_address: AddressEnvelope::from_address(Address::parse_for_network(
                        Network::Localnet,
                        recipient,
                    )?),
                    amount,
                    fee_limit: FeeLimit::new(1),
                    nonce: Nonce::new(nonce.parse()?),
                    valid_from_height: Height::new(from.parse()?),
                    valid_until_height: Height::new(until.parse()?),
                })?,
                &key,
            )?;
            write_new(output, &transfer.encode_canonical()?)?;
            println!("transfer_id={}", hex(transfer.id()?.as_bytes()));
        }
        ["transfer-submit", manifest, endpoint, path] => {
            let genesis = genesis(manifest)?;
            let bytes = read_bounded(Path::new(path), cbor::MAX_OBJECT_SIZE)?;
            let transfer = SignedTransferV1::decode_canonical(&bytes)?;
            transfer.verify_for_chain(genesis.chain_id())?;
            let response = wire::request(
                wire::address(endpoint)?,
                genesis.id().as_bytes(),
                wire::SUBMIT,
                bytes,
            )?;
            print_status(&response, &genesis)?;
        }
        _ => return Err(help().into()),
    }
    Ok(())
}
