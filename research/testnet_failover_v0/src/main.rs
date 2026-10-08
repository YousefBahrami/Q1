//! Isolated TESTNET experiment fixture signer, never a wallet or node authority.
//! TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS.
use q1_primitives::{
    ed25519,
    key::{Ed25519PrivateKey, Ed25519Signature},
};

fn decode(text: &str) -> Result<Vec<u8>, String> {
    if !text.len().is_multiple_of(2) || text.len() > 131_072 || !text.is_ascii() {
        return Err("hex size".into());
    }
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).map_err(|_| "hex".into()))
        .collect()
}
fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 4 {
        return Err("sign|verify fixture_id message_hex [signature_hex]".into());
    }
    let id: u8 = args[2].parse().map_err(|_| "id")?;
    if id > 5 {
        return Err("fixture id".into());
    }
    let private = Ed25519PrivateKey::from_seed([101 + id; 32]);
    let message = decode(&args[3])?;
    let mut framed = b"Q1_TESTNET_FAILOVER_LAB_ONLY\0".to_vec();
    framed.extend_from_slice(&(message.len() as u64).to_be_bytes());
    framed.extend_from_slice(&message);
    match args[1].as_str() {
        "sign" if args.len() == 4 => {
            for b in ed25519::sign(&private, &framed).as_bytes() {
                print!("{b:02x}");
            }
            println!();
        }
        "verify" if args.len() == 5 => {
            let bytes: [u8; 64] = decode(&args[4])?
                .try_into()
                .map_err(|_| "signature length")?;
            ed25519::verify_strict(
                ed25519::public_key(&private).map_err(|_| "key")?,
                &framed,
                Ed25519Signature::from_bytes(bytes),
            )
            .map_err(|_| "invalid signature")?;
            println!("valid");
        }
        _ => return Err("operation".into()),
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
