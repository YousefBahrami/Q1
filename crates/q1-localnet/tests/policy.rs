//! Local activation and fixed-membership quorum behavior.
use q1_localnet::{Error, LocalnetV0, quorum::LocalnetCommittee};
use q1_protocol_types::{chain::NetworkClass, types::ParticipantId};
fn id(n: u8) -> ParticipantId {
    ParticipantId::from_bytes([n; 32])
}
#[test]
fn two_of_three_remains_fixed_with_one_offline_voter() {
    let committee = LocalnetCommittee::new(
        LocalnetV0::new(NetworkClass::Localnet).unwrap(),
        id(1),
        [id(2), id(3), id(4)],
    )
    .unwrap();
    assert!(!committee.has_quorum(&[]).unwrap());
    assert!(!committee.has_quorum(&[id(2)]).unwrap());
    for pair in [[id(2), id(3)], [id(2), id(4)], [id(3), id(4)]] {
        assert!(committee.has_quorum(&pair).unwrap());
    }
    assert_eq!(committee.voters(), &[id(2), id(3), id(4)]);
    assert_eq!(committee.required_votes(), 2);
    assert_eq!(
        committee.has_quorum(&[id(2), id(2)]),
        Err(Error::DuplicateVote)
    );
    assert_eq!(committee.has_quorum(&[id(1), id(2)]), Err(Error::NonMember));
    assert_eq!(committee.has_quorum(&[id(2), id(5)]), Err(Error::NonMember));
}
#[test]
fn duplicate_members_and_voting_producer_are_rejected() {
    let mode = LocalnetV0::new(NetworkClass::Localnet).unwrap();
    assert_eq!(
        LocalnetCommittee::new(mode, id(1), [id(1), id(2), id(3)]),
        Err(Error::InvalidCommittee)
    );
    assert_eq!(
        LocalnetCommittee::new(mode, id(1), [id(2), id(2), id(3)]),
        Err(Error::InvalidCommittee)
    );
}
#[test]
fn no_implicit_nonlocal_activation() {
    for class in [NetworkClass::PrivateTestnet, NetworkClass::Research] {
        assert!(matches!(
            LocalnetV0::new(class),
            Err(Error::NonLocalNetwork)
        ));
    }
    assert!(NetworkClass::try_from(4).is_err()); // No production class can be enabled.
}
