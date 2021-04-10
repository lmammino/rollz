use rollz::prelude::*;
use rollz::D6;

fn main() {
    let d: (D6, D6) = (roll(), roll());
    println!("You got {:?}", d);
}
