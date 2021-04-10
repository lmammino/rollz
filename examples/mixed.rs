use rollz::prelude::*;
use rollz::{D10, D12, D20, D4, D6, D8};

fn main() {
    let d: (D4, D6, D8, D10, D12, D20) = (roll(), roll(), roll(), roll(), roll(), roll());
    println!("You got {:?}", d);
}
