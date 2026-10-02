//! Emits the frozen LOCALNET_V0 reference objects and commitments.
#[path = "../tests/common/mod.rs"]
mod common;
#[path = "../tests/common/vectors.rs"]
mod vectors;
use common::*;
fn main() {
    for (name, bytes) in vectors::vectors() {
        println!("{name}\t{}", hex(&bytes));
    }
}
