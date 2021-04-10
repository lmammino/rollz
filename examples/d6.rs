use rollz::prelude::*;
use rollz::D6;

fn main() {
    let d: D6 = roll();
    println!("You got a {}", d.val());
}
