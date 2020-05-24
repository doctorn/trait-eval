# `trait_eval`

[![Crates.io](https://img.shields.io/crates/v/trait_eval.svg?style=plastic)](https://crates.io/crates/trait_eval)
[![Build Status](https://travis-ci.org/doctorn/trait-eval.svg?branch=master)](https://travis-ci.org/doctorn/trait-eval)
[![lines of code](https://tokei.rs/b1/github/doctorn/trait-eval)](https://github.com/Aaronepower/tokei)
![Minimum rustc 1.43](https://img.shields.io/badge/rustc-1.43+-brightgreen.svg)

We all know Rust's trait system is Turing complete, so tell me, why aren't we exploiting this??? Who needs const-fn when we've got a crate like this?!

Honestly, I was too preoccupied with the fact that I could to stop to think whether I actually should.

Believe it or not, [I even wrote docs for this](https://crates.io/crates/trait_eval).

## Example

Here's an eminently readable example where we play FizzBuzz at compile-time!

```rust
trait FizzBuzzType {
    fn show() -> String; // Don't worry about this -- it's just so we can print the result
}

struct Fizz;

impl FizzBuzzType for Fizz {
    fn show() -> String {
        "Fizz".to_string()
    }
}

struct Buzz;

impl FizzBuzzType for Buzz {
    fn show() -> String {
        "Buzz".to_string()
    }
}

struct FizzBuzz;

impl FizzBuzzType for FizzBuzz {
    fn show() -> String {
        "FizzBuzz".to_string()
    }
}

impl<T: Nat> FizzBuzzType for T
where
    T: Eval,
    <T as Eval>::Output: Display,
{
    fn show() -> String {
        format!("{}", T::eval())
    }
}

trait FizzBuzzEval: Nat {
    type Result: FizzBuzzType;
}

impl<T: Nat,
    Mod3: Nat,
    Mod5: Nat,
    ShouldFizz: Bool,
    ShouldBuzz: Bool,
    ShouldFizzBuzz: Bool,
    DidBuzz: FizzBuzzType,
    DidFizz: FizzBuzzType,
    DidFizzBuzz: FizzBuzzType> FizzBuzzEval for T
where
    T: Mod<Three, Result = Mod3> + Mod<Five, Result = Mod5>,
    Mod3: Equals<Zero, Result = ShouldFizz>,
    Mod5: Equals<Zero, Result = ShouldBuzz>,
    ShouldFizz: AndAlso<ShouldBuzz, Result = ShouldFizzBuzz>,
    (Fizz, T): If<ShouldFizz, Result = DidFizz>,
    (Buzz, DidFizz): If<ShouldBuzz, Result = DidBuzz>,
    (FizzBuzz, DidBuzz): If<ShouldFizzBuzz, Result = DidFizzBuzz>,
{
    type Result = DidFizzBuzz;
}

assert_eq!(<One as FizzBuzzEval>::Result::show(), "1");
assert_eq!(<Two as FizzBuzzEval>::Result::show(), "2");
assert_eq!(<Three as FizzBuzzEval>::Result::show(), "Fizz");
assert_eq!(<Four as FizzBuzzEval>::Result::show(), "4");
assert_eq!(<Five as FizzBuzzEval>::Result::show(), "Buzz");
assert_eq!(<Six as FizzBuzzEval>::Result::show(), "Fizz");
assert_eq!(<Seven as FizzBuzzEval>::Result::show(), "7");
assert_eq!(<Eight as FizzBuzzEval>::Result::show(), "8");
assert_eq!(<Nine as FizzBuzzEval>::Result::show(), "Fizz");
assert_eq!(<Ten as FizzBuzzEval>::Result::show(), "Buzz");

type Fifteen = <Three as Times<Five>>::Result;
assert_eq!(<Fifteen as FizzBuzzEval>::Result::show(), "FizzBuzz"); // !!!
```

## Contributing

Please, for the love of God, don't use this crate. If you must contribute, open a PR.
