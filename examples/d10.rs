use rollz::prelude::*;
use rollz::D10;

fn main() {
    let d: D10 = roll();
    println!("You got a {}", d.val());
}
