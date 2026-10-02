//! Emits reproducible approved-object vectors using public test fixtures only.
#[path = "../tests/support/vectors.rs"]
mod support;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    for (name, hex) in support::vectors()? {
        println!("{name}\t{hex}");
    }
    Ok(())
}
