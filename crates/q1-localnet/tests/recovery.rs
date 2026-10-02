//! Durable commit, exclusive directory ownership, recovery and anti-equivocation.
mod common;
use common::*;
use q1_localnet::{Error, store::PersistentChain};
use q1_primitives::cbor::{self, Value};

#[test]
fn durable_commit_reopens_with_same_root_and_continues() {
    let directory = tempdir();
    let genesis = genesis();
    let mut store = PersistentChain::create(&directory, &genesis).unwrap();
    assert!(matches!(
        PersistentChain::open(&directory, &genesis),
        Err(Error::StorageLocked)
    ));
    let p = proposal(store.chain(), 0, 20);
    store.reserve_vote(voter(12), &p).unwrap();
    store.commit(&certify(&genesis, p)).unwrap();
    let saved = store.chain().clone();
    drop(store);
    std::fs::write(
        directory.join("state.pending"),
        b"interrupted temporary file",
    )
    .unwrap();
    let mut reopened = PersistentChain::open(&directory, &genesis).unwrap();
    assert_eq!(reopened.chain(), &saved);
    assert_eq!(reopened.chain().state().ledger().height().get(), 1);
    let next = proposal(reopened.chain(), 1, 30);
    reopened.commit(&certify(&genesis, next)).unwrap();
    assert_eq!(reopened.chain().state().ledger().height().get(), 2);
    assert_eq!(
        reopened
            .chain()
            .state()
            .ledger()
            .account(address(7))
            .balance()
            .get(),
        948
    );
    assert_eq!(reopened.chain().state().ledger().reward_pool().get(), 2);
    let root = reopened.chain().state().root().unwrap();
    drop(reopened);
    assert_eq!(
        PersistentChain::open(&directory, &genesis)
            .unwrap()
            .chain()
            .state()
            .root()
            .unwrap(),
        root
    );
    std::fs::remove_dir_all(directory).unwrap();
}
#[test]
fn reservations_survive_restart_and_reject_equivocation() {
    let directory = tempdir();
    let genesis = genesis();
    let mut store = PersistentChain::create(&directory, &genesis).unwrap();
    let first = proposal(store.chain(), 0, 20);
    let different = proposal(store.chain(), 0, 21);
    store.reserve_vote(voter(12), &first).unwrap();
    store.reserve_vote(voter(12), &first).unwrap();
    drop(store);
    let mut store = PersistentChain::open(&directory, &genesis).unwrap();
    let before = std::fs::read(directory.join("state.cbor")).unwrap();
    assert_eq!(
        store.reserve_vote(voter(12), &different),
        Err(Error::Equivocation)
    );
    assert_eq!(
        store.reserve_vote(genesis.producer(), &first),
        Err(Error::NonMember)
    );
    assert_eq!(before, std::fs::read(directory.join("state.cbor")).unwrap());
    drop(store);
    std::fs::remove_dir_all(directory).unwrap();
}
#[test]
fn corruption_and_snapshot_tampering_are_not_silently_trusted() {
    let directory = tempdir();
    let genesis = genesis();
    let mut store = PersistentChain::create(&directory, &genesis).unwrap();
    let block = certify(&genesis, proposal(store.chain(), 0, 20));
    store.commit(&block).unwrap();
    drop(store);
    let path = directory.join("state.cbor");
    let original = std::fs::read(&path).unwrap();
    let Value::Array(mut archive) = cbor::decode(&original).unwrap() else {
        panic!()
    };
    archive[4] = Value::Unsigned(2);
    std::fs::write(&path, cbor::encode(&Value::Array(archive)).unwrap()).unwrap();
    assert!(PersistentChain::open(&directory, &genesis).is_err());
    std::fs::write(&path, &original[..original.len() / 2]).unwrap();
    assert!(PersistentChain::open(&directory, &genesis).is_err());
    std::fs::write(&path, &original).unwrap();
    assert!(PersistentChain::create(&directory, &genesis).is_err());
    let reopened = PersistentChain::open(&directory, &genesis).unwrap();
    assert_eq!(reopened.chain().history().len(), 1);
    drop(reopened);
    std::fs::remove_dir_all(directory).unwrap();
}
#[test]
fn invalid_commit_and_failed_pre_rename_write_preserve_disk_and_memory() {
    let directory = tempdir();
    let genesis = genesis();
    let mut store = PersistentChain::create(&directory, &genesis).unwrap();
    let block = certify(&genesis, proposal(store.chain(), 0, 20));
    store.commit(&block).unwrap();
    let before = store.chain().clone();
    let disk = std::fs::read(directory.join("state.cbor")).unwrap();
    assert!(store.commit(&block).is_err());
    assert_eq!(store.chain(), &before);
    assert_eq!(std::fs::read(directory.join("state.cbor")).unwrap(), disk);
    std::fs::create_dir(directory.join("state.pending")).unwrap();
    let next = certify(&genesis, proposal(store.chain(), 1, 30));
    assert!(store.commit(&next).is_err());
    assert_eq!(store.chain(), &before);
    assert_eq!(std::fs::read(directory.join("state.cbor")).unwrap(), disk);
    std::fs::remove_dir(directory.join("state.pending")).unwrap();
    store.commit(&next).unwrap();
    drop(store);
    std::fs::remove_dir_all(directory).unwrap();
}
