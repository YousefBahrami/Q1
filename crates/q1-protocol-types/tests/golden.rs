//! Rust objects must match independently produced Node/OpenSSL reference bytes.
mod support {
    include!("support/vectors.rs");
}
#[test]
fn approved_objects_match_cross_language_vectors() {
    let expected: std::collections::BTreeMap<_, _> =
        include_str!("../../../vectors/protocol_objects/v1/approved.tsv")
            .lines()
            .map(|line| {
                let (name, value) = line.split_once('\t').unwrap();
                (name, value.to_owned())
            })
            .collect();
    assert_eq!(support::vectors().unwrap(), expected);
}
