use rollz::prelude::*;

/// A roll of this guy will always give you 100!
/// Shush ... Don't tell anyone! 🤫
#[derive(Debug)]
struct Fake100(u8);

impl Rollable for Fake100 {
    fn roll() -> Fake100 {
        Fake100 { 0: 100 }
    }
    fn val(&self) -> u8 {
        self.0
    }
}

fn main() {
    println!("I bet I'll get a 100 this time!");
    let d: Fake100 = roll();
    println!("Look what I got: {}!", d.val())
}
