# rollz 🎲

[![build badge](https://github.com/lmammino/rollz/workflows/Rust/badge.svg)](https://github.com/lmammino/rollz/actions?query=workflow%3ARust)
[![codecov](https://codecov.io/gh/lmammino/rollz/branch/main/graph/badge.svg?token=4CNbvgaDc1)](https://codecov.io/gh/lmammino/rollz)
[![crates.io badge](https://img.shields.io/crates/v/rollz.svg)](https://crates.io/crates/rollz)
[![Documentation](https://docs.rs/rollz/badge.svg)](https://docs.rs/rollz)

`rollz` is a super-simple Rust crate that allows you to... roll dice! What else would you need from a crate?!

In reality, this crate was born as an experiment for me to understand Rust return type polymorphism and while I was at it, why wouldn't I publish it as a crate for everyone else enjoyment!?

I don't expect this to be particularly useful for _real life projects™️_, except maybe if you are implementing something that has to do with D&D or similar types of board games.

Yes, this crate implements standard D&D dice set: `D4`, `D6`, `D8`, `D10`, `D12` and `D20`.


## Install

To install the library add the following lines to your `Cargo.toml`:

```toml
[dependencies]
rollz = "0"
```

Or, if you have [`cargo add`](https://github.com/killercup/cargo-edit), you can run the following command:

```bash
cargo add rollz@0
```


## Sample Usage

This is how you would roll a `D10` with `rollz`:

```rust
use rollz::prelude::*;
use rollz::D10;

fn main() {
    let d: D10 = roll();
    println!("You got a {}", d.val()); // You got a 2
}
```

Or you can also roll multiple dices together, because why not?

```rust
use rollz::prelude::*;
use rollz::D6;

fn main() {
    let d: (D6, D6) = (roll(), roll());
    println!("You got {:?}", d); // You got (D6(4), D6(6))
}
```

More usage examples are available in the [`examples`](/examples) folder.


### Create your own custom die

Here's an example for you, if you want to build a custom die:

```rust
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
```


## Contributing

Everyone is very welcome to contribute to this project.
You can contribute just by submitting bugs or suggesting improvements by
[opening an issue on GitHub](https://github.com/lmammino/rollz/issues).


## License

Licensed under [MIT License](LICENSE). © Luciano Mammino.