//! Writes PUBLIC, deterministic test keys and objects for four-process acceptance.
//! These seeds are never generated or selected implicitly by the node daemon.
#[path = "../tests/common/mod.rs"]
mod common;
use q1_localnet::block::Chain;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

fn write(directory: &Path, name: &str, bytes: &[u8]) -> std::io::Result<()> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options.open(directory.join(name))?.write_all(bytes)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argument = std::env::args()
        .nth(1)
        .ok_or("provide a new fixture directory")?;
    let directory = Path::new(&argument);
    fs::create_dir(directory)?;
    write(directory, "TEST_ONLY.txt", "TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS\nAll fixture seeds are deterministic and publicly known.\n".as_bytes())?;
    write(directory, "sender.key", &[7; 32])?;
    write(directory, "recipient.key", &[8; 32])?;
    write(
        directory,
        "recipient.address",
        common::address(8)
            .to_address(q1_primitives::Network::Localnet)?
            .encode()
            .as_bytes(),
    )?;
    let genesis = common::genesis();
    write(directory, "genesis.cbor", &genesis.encode_canonical()?)?;
    write(
        directory,
        "genesis.id",
        common::hex(genesis.id().as_bytes()).as_bytes(),
    )?;
    write(
        directory,
        "producer.id",
        common::hex(genesis.producer().as_bytes()).as_bytes(),
    )?;
    write(directory, "producer.key", &[11; 32])?;
    for (index, seed) in [12, 13, 14].into_iter().enumerate() {
        write(
            directory,
            &format!("voter{index}.id"),
            common::hex(common::voter(seed).as_bytes()).as_bytes(),
        )?;
        write(directory, &format!("voter{index}.key"), &[seed; 32])?;
    }
    let mut chain = Chain::new(genesis.clone())?;
    for nonce in 0..4 {
        write(
            directory,
            &format!("tx{nonce}.cbor"),
            &common::transfer(&genesis, nonce, 10).encode_canonical()?,
        )?;
        if nonce == 3 {
            write(
                directory,
                "conflict3.proposal",
                &common::proposal(&chain, nonce, 11).encode_canonical()?,
            )?;
        }
        let proposal = common::proposal(&chain, nonce, 10);
        let block = common::certify(&genesis, proposal);
        chain.commit(&block)?;
        write(
            directory,
            &format!("state{}.cbor", nonce + 1),
            &chain.state().encode_canonical()?,
        )?;
        write(
            directory,
            &format!("root{}.hex", nonce + 1),
            common::hex(chain.state().root()?.as_bytes()).as_bytes(),
        )?;
    }
    println!(
        "TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS: {}",
        directory.display()
    );
    Ok(())
}
