//! NONE evidence activation, collision, and independent fixed-vector checks.
use q1_primitives::{
    cbor::{self, Value},
    domain::Domain,
};
use q1_protocol_types::{chain::NetworkClass, delay::LocalnetNoneEvidence};
#[test]
fn none_is_canonical_and_local_only() {
    let local = NetworkClass::Localnet;
    let evidence = LocalnetNoneEvidence::new(local).unwrap();
    assert_eq!(
        evidence.encode_canonical().unwrap(),
        [0x85, 1, 0, 0, 0x40, 0x40]
    );
    assert_eq!(
        LocalnetNoneEvidence::decode_for_network(local, &[0x85, 1, 0, 0, 0x40, 0x40]).unwrap(),
        evidence
    );
    for network in [NetworkClass::PrivateTestnet, NetworkClass::Research] {
        assert!(LocalnetNoneEvidence::new(network).is_err());
        assert!(evidence.verify_for_network(network).is_err());
        assert!(
            LocalnetNoneEvidence::decode_for_network(
                network,
                &evidence.encode_canonical().unwrap()
            )
            .is_err()
        );
    }
    for index in 0..5 {
        let Value::Array(mut fields) = cbor::decode(&evidence.encode_canonical().unwrap()).unwrap()
        else {
            panic!()
        };
        fields[index] = if index < 3 {
            Value::Unsigned(2)
        } else {
            Value::Bytes(vec![0])
        };
        assert!(
            LocalnetNoneEvidence::decode_for_network(
                local,
                &cbor::encode(&Value::Array(fields)).unwrap()
            )
            .is_err()
        );
    }
}
#[test]
fn registry_has_no_aliases_and_delay_evidence_does_not_reuse_output() {
    let mut seen = std::collections::BTreeSet::new();
    for id in 0..=u16::MAX {
        if let Ok(domain) = Domain::try_from(id) {
            assert_eq!(domain.id(), id);
            assert!(seen.insert(domain.id()));
        }
    }
    assert_eq!(seen.len(), 20);
    assert_eq!(Domain::try_from(0x0014), Ok(Domain::DelayEvidence));
    assert_eq!(
        Domain::try_from(0x0011).unwrap(),
        Domain::LocalnetProposalId
    );
    assert_eq!(Domain::try_from(0x0015).unwrap(), Domain::LocalnetState);
    assert!(Domain::try_from(0x0012).is_err());
    let witness = LocalnetNoneEvidence::new(NetworkClass::Localnet).unwrap();
    assert_ne!(
        witness.commitment().unwrap().as_bytes(),
        q1_primitives::sha256::hash_domain(
            Domain::DelayOutput,
            &witness.encode_canonical().unwrap()
        )
        .unwrap()
        .as_bytes()
    );
}
