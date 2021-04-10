//! `rollz` is a super-simple Rust crate that allows you to... roll dice! What else would you need from a crate?!
//!
//! This crate implements standard D&D dice set: `D4`, `D6`, `D8`, `D10`, `D12` and `D20`.
//!
//! Here's an example:
//!
//! ```rust
//! use rollz::prelude::*;
//! use rollz::D10;
//!
//! let d: D10 = roll();
//! assert!((1..=10).contains(&d.val()));
//! ```
//!
//! You can also implement your own custom die if you implement the `Rollable` trait:
//!
//! ```rust
//! use rollz::prelude::*;
//!
//! struct Fake100(u8);
//!
//! impl Rollable for Fake100 {
//!     fn roll() -> Fake100 {
//!         Fake100 { 0: 100 }
//!     }
//!     fn val(&self) -> u8 {
//!         self.0
//!     }
//! }
//!
//! let d: Fake100 = roll();
//! assert_eq!(100, d.val());
//!```

use rand::{thread_rng, Rng};

/// This is the trait that every die needs to implement to be... well... "rollable", right?
pub trait Rollable {
    /// Roll the die
    fn roll() -> Self;
    /// Get the value from the latest roll
    fn val(&self) -> u8;
}

/// A generic function to roll a given die.
/// It uses return type polymorphism so it will roll the die you actually want! Boom, like magic!
pub fn roll<T: Rollable>() -> T {
    Rollable::roll()
}

pub mod prelude;

/// A D4 die (4 faces): a roll will give you a `u8` in the `1..=4` range.
#[derive(Debug)]
pub struct D4(u8);
impl Rollable for D4 {
    fn roll() -> D4 {
        D4 {
            0: thread_rng().gen_range(1..=4),
        }
    }
    fn val(&self) -> u8 {
        self.0
    }
}

/// A D6 die (6 faces): a roll will give you a `u8` in the `1..=6` range.
#[derive(Debug)]
pub struct D6(u8);
impl Rollable for D6 {
    fn roll() -> D6 {
        D6 {
            0: thread_rng().gen_range(1..=6),
        }
    }
    fn val(&self) -> u8 {
        self.0
    }
}

/// A D8 die (8 faces): a roll will give you a `u8` in the `1..=8` range.
#[derive(Debug)]
pub struct D8(u8);
impl Rollable for D8 {
    fn roll() -> D8 {
        D8 {
            0: thread_rng().gen_range(1..=8),
        }
    }
    fn val(&self) -> u8 {
        self.0
    }
}

/// A D10 die (10 faces): a roll will give you a `u8` in the `1..=10` range.
#[derive(Debug)]
pub struct D10(u8);
impl Rollable for D10 {
    fn roll() -> D10 {
        D10 {
            0: thread_rng().gen_range(1..=10),
        }
    }
    fn val(&self) -> u8 {
        self.0
    }
}

/// A D12 die (12 faces): a roll will give you a `u8` in the `1..=12` range.
#[derive(Debug)]
pub struct D12(u8);
impl Rollable for D12 {
    fn roll() -> D12 {
        D12 {
            0: thread_rng().gen_range(1..=12),
        }
    }
    fn val(&self) -> u8 {
        self.0
    }
}

/// A D20 die (20 faces): a roll will give you a `u8` in the `1..=20` range.
#[derive(Debug)]
pub struct D20(u8);
impl Rollable for D20 {
    fn roll() -> D20 {
        D20 {
            0: thread_rng().gen_range(1..=20),
        }
    }
    fn val(&self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_rolls_a_d4() {
        let d: D4 = roll();
        assert!((1..=4).contains(&d.val()));
    }

    #[test]
    fn it_rolls_a_d6() {
        let d: D6 = roll();
        assert!((1..=6).contains(&d.val()));
    }

    #[test]
    fn it_rolls_a_d8() {
        let d: D8 = roll();
        assert!((1..=8).contains(&d.val()));
    }

    #[test]
    fn it_rolls_a_d10() {
        let d: D10 = roll();
        assert!((1..=10).contains(&d.val()));
    }

    #[test]
    fn it_rolls_a_d12() {
        let d: D12 = roll();
        assert!((1..=12).contains(&d.val()));
    }

    #[test]
    fn it_rolls_a_d20() {
        let d: D20 = roll();
        assert!((1..=20).contains(&d.val()));
    }

    #[test]
    fn it_rolls_multiple_d6() {
        let d: (D6, D6, D6) = (roll(), roll(), roll());
        assert!((1..=6).contains(&d.0.val()));
        assert!((1..=6).contains(&d.1.val()));
        assert!((1..=6).contains(&d.2.val()));
    }

    #[test]
    fn it_rolls_multiple_mixed_dice() {
        let d: (D6, D8, D10, D12) = (roll(), roll(), roll(), roll());
        assert!((1..=6).contains(&d.0.val()));
        assert!((1..=8).contains(&d.1.val()));
        assert!((1..=10).contains(&d.2.val()));
        assert!((1..=12).contains(&d.3.val()));
    }
}
