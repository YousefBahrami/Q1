//! TESTNET_FAILOVER_V0 fixture backend: actual Q1 ledger execution and certificate replay.
//! TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS. Legacy mode uses public fixtures;
//! native mode requires separately provisioned private TESTNET credentials.
use q1_localnet::{TestnetFailoverV0, ledger::Ledger, state::StateSnapshot};
use q1_primitives::{
    Address, Amount, Ed25519PrivateKey, Ed25519PublicKey, Ed25519Signature, Height, Network,
    cbor::{self, Value as Cbor},
    domain::Domain,
    ed25519, sha256,
};
use q1_protocol_types::{
    address::AddressEnvelope,
    block_body::BlockBodyV1,
    chain::{ChainIdentityPreimageV1, NetworkClass},
    parent::GenesisId,
    participant::ParticipantRecordV1,
    transfer::{SignedTransferV1, TransferBodyV1, TransferFields},
    types::FeeLimit,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::io::{self, Read};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const PROFILE: &str = "TESTNET_FAILOVER_V0";
const LIMIT: usize = 262_144;
static NATIVE_KEYS: std::sync::OnceLock<Value> = std::sync::OnceLock::new();
fn need(ok: bool, why: &str) -> Result<()> {
    if ok { Ok(()) } else { Err(why.into()) }
}
fn bytes(v: &Value) -> Result<Vec<u8>> {
    let s = v.as_str().ok_or("hex string")?;
    need(
        s.is_ascii() && s.len().is_multiple_of(2) && s.len() <= LIMIT * 2,
        "hex size",
    )?;
    let out: Vec<u8> = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect::<std::result::Result<_, _>>()?;
    need(hex(&out) == s, "canonical hex")?;
    Ok(out)
}
fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
fn number(v: &Value) -> Result<u64> {
    v.as_u64().ok_or_else(|| "integer".into())
}
fn canon(v: &Value) -> Result<Vec<u8>> {
    Ok(serde_json::to_vec(v)?)
}
fn id(v: &Value) -> Result<String> {
    Ok(hex(&Sha256::digest(canon(v)?)))
}
fn seed_key(id: u64) -> Result<Ed25519PrivateKey> {
    if let Some(keys) = NATIVE_KEYS.get() {
        let seed: [u8; 32] = bytes(&keys["seeds"][id.to_string()])?
            .try_into()
            .map_err(|_| "seed width")?;
        let key = Ed25519PrivateKey::from_seed(seed);
        need(
            hex(ed25519::public_key(&key)?.as_bytes()) == keys["public"][id.to_string()],
            "key mismatch",
        )?;
        return Ok(key);
    }
    let seed = if id <= 5 {
        101 + u8::try_from(id)?
    } else {
        u8::try_from(id)?
    };
    Ok(Ed25519PrivateKey::from_seed([seed; 32]))
}
fn key(id: u64) -> Result<Ed25519PrivateKey> {
    need(id <= 5, "signer")?;
    seed_key(id)
}
fn public(id: u64) -> Result<Ed25519PublicKey> {
    if let Some(keys) = NATIVE_KEYS.get() {
        let raw: [u8; 32] = bytes(&keys["public"][id.to_string()])?
            .try_into()
            .map_err(|_| "public width")?;
        return Ok(Ed25519PublicKey::from_bytes(raw)?);
    }
    ed25519::public_key(&seed_key(id)?).map_err(Into::into)
}
fn load_native_keys() -> Result<()> {
    let path = std::env::var_os("Q1_TESTNET_KEYS_FILE");
    if path.is_none() {
        need(
            std::env::var_os("Q1_TESTNET_NATIVE_REQUIRED").is_none(),
            "native credentials required",
        )?;
        return Ok(());
    }
    use std::os::unix::fs::PermissionsExt;
    let path = std::path::PathBuf::from(path.ok_or("key path")?);
    let meta = std::fs::symlink_metadata(&path)?;
    need(
        meta.is_file() && meta.len() <= 16384 && meta.permissions().mode() & 0o077 == 0,
        "key file permissions/size",
    )?;
    let raw = std::fs::read(path)?;
    let value: Value = serde_json::from_slice(&raw)?;
    need(
        canon(&value)? == raw
            && value["profile"] == "TESTNET_NATIVE_TRANSPORT_V0"
            && value["warning"] == "TESTNET ONLY",
        "native credentials profile",
    )?;
    need(
        value.as_object().is_some_and(|o| o.len() == 4),
        "credential fields",
    )?;
    need(
        value["public"].as_object().is_some_and(|o| o.len() == 8),
        "public registry fields",
    )?;
    let mut unique = std::collections::BTreeSet::new();
    for id in [0, 1, 2, 3, 4, 5, 201, 202] {
        need(
            unique.insert(value["public"][id.to_string()].clone().to_string()),
            "duplicate public key",
        )?;
        need(
            bytes(&value["public"][id.to_string()])?.len() == 32,
            "public registry",
        )?;
    }
    NATIVE_KEYS.set(value).map_err(|_| "keys already loaded")?;
    Ok(())
}
fn message(b: &Value) -> Result<Vec<u8>> {
    let raw = canon(b)?;
    let mut out = b"TESTNET_FAILOVER_V0_MESSAGE\0".to_vec();
    out.extend_from_slice(&(raw.len() as u64).to_be_bytes());
    out.extend(raw);
    Ok(out)
}
fn sign(who: u64, b: Value) -> Result<Value> {
    let signature = ed25519::sign(&key(who)?, &message(&b)?);
    Ok(json!({"signer":who,"body":b,"signature":hex(signature.as_bytes())}))
}
fn auth(v: &Value, nonce: &str) -> Result<Value> {
    need(
        v.as_object().is_some_and(|o| o.len() == 3),
        "envelope fields",
    )?;
    let who = number(&v["signer"])?;
    need(who <= 5, "signer")?;
    let sig: [u8; 64] = bytes(&v["signature"])?
        .try_into()
        .map_err(|_| "signature width")?;
    ed25519::verify_strict(
        public(who)?,
        &message(&v["body"])?,
        Ed25519Signature::from_bytes(sig),
    )?;
    need(
        v["body"]["profile"] == PROFILE && v["body"]["chain"] == nonce,
        "profile/chain",
    )?;
    Ok(v["body"].clone())
}
fn address(seed: u8) -> Result<AddressEnvelope> {
    Ok(AddressEnvelope::from_address(Address::from_public_key(
        Network::PrivateTestnet,
        public(u64::from(seed))?,
    )?))
}
fn initial(nonce: &str) -> Result<StateSnapshot> {
    let raw: [u8; 32] = bytes(&json!(nonce))?
        .try_into()
        .map_err(|_| "nonce width")?;
    let chain = ChainIdentityPreimageV1::new(NetworkClass::PrivateTestnet, raw);
    let mode = TestnetFailoverV0::new(chain.network_class())?;
    let mut registry = Vec::new();
    for i in 0..5 {
        registry.push(ParticipantRecordV1::new(
            if i >= 3 { Some(public(i)?) } else { None },
            if i < 3 { Some(public(i)?) } else { None },
            Height::ZERO,
            None,
        )?);
    }
    registry.sort_by_key(|r| r.participant_id());
    let mut allocations = vec![
        (address(201)?, Amount::new(1000)),
        (address(202)?, Amount::ZERO),
    ];
    allocations.sort_by_key(|r| r.0);
    let genesis = cbor::encode(&Cbor::Array(vec![
        Cbor::Unsigned(1),
        Cbor::Bytes(PROFILE.as_bytes().to_vec()),
        cbor::decode(&chain.encode_canonical()?)?,
        Cbor::Bytes(Amount::new(1000).to_be_bytes().to_vec()),
        Cbor::Array(
            allocations
                .iter()
                .map(|(a, n)| {
                    Cbor::Array(vec![
                        Cbor::Bytes(a.as_bytes().to_vec()),
                        Cbor::Bytes(n.to_be_bytes().to_vec()),
                    ])
                })
                .collect(),
        ),
        Cbor::Array(
            registry
                .iter()
                .map(|r| cbor::decode(&r.encode_canonical().expect("validated record")))
                .collect::<std::result::Result<_, _>>()?,
        ),
        Cbor::Bytes(b"2 producers;3 voters;quorum2;fee1;no issuance;no delay authority".to_vec()),
    ]))?;
    let genesis_id =
        GenesisId::from_bytes(sha256::hash_domain(Domain::Genesis, &genesis)?.into_bytes());
    let ledger =
        Ledger::from_testnet_allocations(mode, chain.chain_id()?, Amount::new(1000), allocations)?;
    Ok(StateSnapshot::for_testnet(
        mode, genesis_id, ledger, registry,
    )?)
}
fn value_next(v: &Value, state: &StateSnapshot, tip: &str, nonce: &str) -> Result<StateSnapshot> {
    let b = auth(v, nonce)?;
    let height = state.ledger().height().checked_increment()?.get();
    let ballot = number(&b["origin_ballot"])?;
    need(ballot <= u64::from(u32::MAX), "ballot bound")?;
    need(b.as_object().is_some_and(|o| o.len() == 7), "value fields")?;
    need(
        b["kind"] == "value" && number(&b["height"])? == height && b["parent"] == tip,
        "value context",
    )?;
    need(
        number(&v["signer"])? == 3 + (height - 1 + ballot) % 2,
        "producer",
    )?;
    let payload = b["payload"].as_str().ok_or("payload")?;
    let p: Value = serde_json::from_str(payload)?;
    need(
        canon(&p)? == payload.as_bytes() && p.as_object().is_some_and(|o| o.len() == 2),
        "payload canonical fields",
    )?;
    let block = BlockBodyV1::decode_canonical(&bytes(&p["block_body"])?)?;
    let next = state.apply_body(Height::new(height), &block)?;
    need(
        p["state_root"] == hex(next.root()?.as_bytes()),
        "StateRoot mismatch",
    )?;
    Ok(next)
}
fn replay(history: &Value, nonce: &str) -> Result<(StateSnapshot, String)> {
    let mut state = initial(nonce)?;
    let mut tip = "0".repeat(64);
    let history = history.as_array().ok_or("history")?;
    need(history.len() <= 16, "history bound")?;
    for c in history {
        need(
            c.as_object().is_some_and(|o| o.len() == 4),
            "certificate fields",
        )?;
        let next = value_next(&c["value"], &state, &tip, nonce)?;
        let height = next.ledger().height().get();
        let ballot = number(&c["ballot"])?;
        need(
            number(&c["height"])? == height
                && number(&c["value"]["body"]["origin_ballot"])? <= ballot,
            "certificate context",
        )?;
        let votes = c["votes"].as_array().ok_or("votes")?;
        need((2..=3).contains(&votes.len()), "quorum")?;
        let mut seen = [false; 3];
        let value_id = id(&c["value"])?;
        for vote in votes {
            let who = usize::try_from(number(&vote["signer"])?)?;
            need(who < 3, "unauthorized voter")?;
            need(!seen[who], "duplicate vote")?;
            seen[who] = true;
            let b = auth(vote, nonce)?;
            need(
                b == json!({"profile":PROFILE,"chain":nonce,"kind":"vote","height":height,"ballot":ballot,"value_id":value_id}),
                "vote context",
            )?;
        }
        tip = value_id;
        state = next;
    }
    Ok((state, tip))
}
fn status(state: &StateSnapshot, tip: &str) -> Result<Value> {
    Ok(
        json!({"profile":PROFILE,"height":state.ledger().height().get(),"tip":tip,"state_root":hex(state.root()?.as_bytes()),"chain_id":hex(state.ledger().chain_id().as_bytes()),"genesis_id":hex(state.genesis_id().as_bytes()),"sender":state.ledger().account(address(201)?).balance().get().to_string(),"recipient":state.ledger().account(address(202)?).balance().get().to_string(),"nonce":state.ledger().account(address(201)?).nonce().get(),"reward_pool":state.ledger().reward_pool().get().to_string(),"total_supply":state.ledger().total_supply().get().to_string()}),
    )
}
fn process(req: &Value) -> Result<Value> {
    let op = req["op"].as_str().ok_or("operation")?;
    let nonce = req["chain"].as_str().ok_or("chain")?;
    if op == "sign" {
        return sign(number(&req["who"])?, req["body"].clone());
    }
    if op == "verify" {
        return auth(&req["envelope"], nonce);
    }
    let (state, tip) = replay(&req["history"], nonce)?;
    match op {
        "status" => status(&state, &tip),
        "validate" => {
            let next = value_next(&req["value"], &state, &tip, nonce)?;
            status(&next, &id(&req["value"])?)
        }
        "propose" => {
            let height = state.ledger().height().checked_increment()?;
            let ballot = number(&req["ballot"])?;
            need(ballot <= u64::from(u32::MAX), "ballot bound")?;
            let who = number(&req["who"])?;
            need(who == 3 + (height.get() - 1 + ballot) % 2, "producer")?;
            let amount = number(&req["amount"])?;
            let private = seed_key(201)?;
            let tx = SignedTransferV1::sign(
                TransferBodyV1::new(TransferFields {
                    chain_id: state.ledger().chain_id(),
                    sender_public_key: ed25519::public_key(&private)?,
                    recipient_address: address(202)?,
                    amount: Amount::new(u128::from(amount)),
                    fee_limit: FeeLimit::new(1),
                    nonce: state.ledger().account(address(201)?).nonce(),
                    valid_from_height: height,
                    valid_until_height: Height::new(100),
                })?,
                &private,
            )?;
            let block = BlockBodyV1::new(vec![tx])?;
            let next = state.apply_body(height, &block)?;
            let payload = String::from_utf8(canon(
                &json!({"block_body":hex(&block.encode_canonical()?),"state_root":hex(next.root()?.as_bytes())}),
            )?)?;
            sign(
                who,
                json!({"profile":PROFILE,"chain":nonce,"kind":"value","height":height.get(),"origin_ballot":ballot,"parent":tip,"payload":payload}),
            )
        }
        _ => Err("operation".into()),
    }
}
fn main() {
    let result = (|| -> Result<Value> {
        load_native_keys()?;
        let mut raw = Vec::new();
        io::stdin().take((LIMIT + 1) as u64).read_to_end(&mut raw)?;
        need(raw.len() <= LIMIT, "input bound")?;
        let req: Value = serde_json::from_slice(&raw)?;
        need(canon(&req)? == raw, "canonical request")?;
        process(&req)
    })();
    match result {
        Ok(v) => println!("{v}"),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capability_rejects_other_networks() {
        assert!(TestnetFailoverV0::new(NetworkClass::Localnet).is_err());
        assert!(TestnetFailoverV0::new(NetworkClass::Research).is_err());
        assert!(q1_localnet::LocalnetV0::new(NetworkClass::PrivateTestnet).is_err());
        assert!(NetworkClass::try_from(4).is_err());
    }
    #[test]
    fn genesis_binds_instance_and_all_roles() {
        let a = initial(&"01".repeat(32)).unwrap();
        let b = initial(&"02".repeat(32)).unwrap();
        assert_ne!(a.genesis_id(), b.genesis_id());
        assert_ne!(a.root().unwrap(), b.root().unwrap());
        assert_eq!(a.ledger().network(), Network::PrivateTestnet);
        assert_eq!(a.ledger().total_supply(), Amount::new(1000));
        assert_eq!(
            a.root().unwrap(),
            initial(&"01".repeat(32)).unwrap().root().unwrap()
        );
    }
    #[test]
    fn overflowing_ballot_rejected_without_panic() {
        let request = json!({"op":"propose","chain":"01".repeat(32),"history":[],"who":3,"ballot":u64::MAX,"amount":10});
        assert!(process(&request).is_err());
    }
}
