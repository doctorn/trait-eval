//! # Compile-Time Evaluation
//!
//! We all know Rust's trait system is Turing complete, so tell me, why aren't we exploiting
//! this???
//!
//! Who needs `const-fn` when we've got a crate like this?!
//!
//! ## Example
//!
//! Here's an eminently readable example where we play FizzBuzz at compile-time!
//!
//! ```rust
//! # use trait_eval::*;
//! # use std::fmt::Display;
//! trait FizzBuzzType {
//!     fn show() -> String; // Don't worry about this -- it's just so we can print the result
//! }
//!
//! struct Fizz;
//!
//! impl FizzBuzzType for Fizz {
//!     fn show() -> String {
//!         "Fizz".to_string()
//!     }
//! }
//!
//! struct Buzz;
//!
//! impl FizzBuzzType for Buzz {
//!     fn show() -> String {
//!         "Buzz".to_string()
//!     }
//! }
//!
//! struct FizzBuzz;
//!
//! impl FizzBuzzType for FizzBuzz {
//!     fn show() -> String {
//!         "FizzBuzz".to_string()
//!     }
//! }
//!
//! impl<T: Nat> FizzBuzzType for T
//! where
//!     T: Eval,
//!     <T as Eval>::Output: Display,
//! {
//!     fn show() -> String {
//!         format!("{}", T::eval())
//!     }
//! }
//!
//! trait FizzBuzzEval: Nat {
//!     type Result: FizzBuzzType;
//! }
//!
//! impl<T: Nat,
//!     Mod3: Nat,
//!     Mod5: Nat,
//!     ShouldFizz: Bool,
//!     ShouldBuzz: Bool,
//!     ShouldFizzBuzz: Bool,
//!     DidBuzz: FizzBuzzType,
//!     DidFizz: FizzBuzzType,
//!     DidFizzBuzz: FizzBuzzType> FizzBuzzEval for T
//! where
//!     T: Mod<Three, Result = Mod3> + Mod<Five, Result = Mod5>,
//!     Mod3: Equals<Zero, Result = ShouldFizz>,
//!     Mod5: Equals<Zero, Result = ShouldBuzz>,
//!     ShouldFizz: AndAlso<ShouldBuzz, Result = ShouldFizzBuzz>,
//!     (Fizz, T): If<ShouldFizz, Result = DidFizz>,
//!     (Buzz, DidFizz): If<ShouldBuzz, Result = DidBuzz>,
//!     (FizzBuzz, DidBuzz): If<ShouldFizzBuzz, Result = DidFizzBuzz>,
//! {
//!     type Result = DidFizzBuzz;
//! }
//!
//! assert_eq!(<One as FizzBuzzEval>::Result::show(), "1");
//! assert_eq!(<Two as FizzBuzzEval>::Result::show(), "2");
//! assert_eq!(<Three as FizzBuzzEval>::Result::show(), "Fizz");
//! assert_eq!(<Four as FizzBuzzEval>::Result::show(), "4");
//! assert_eq!(<Five as FizzBuzzEval>::Result::show(), "Buzz");
//! assert_eq!(<Six as FizzBuzzEval>::Result::show(), "Fizz");
//! assert_eq!(<Seven as FizzBuzzEval>::Result::show(), "7");
//! assert_eq!(<Eight as FizzBuzzEval>::Result::show(), "8");
//! assert_eq!(<Nine as FizzBuzzEval>::Result::show(), "Fizz");
//! assert_eq!(<Ten as FizzBuzzEval>::Result::show(), "Buzz");
//!
//! type Fifteen = <Three as Times<Five>>::Result;
//! assert_eq!(<Fifteen as FizzBuzzEval>::Result::show(), "FizzBuzz"); // !!!
//! ```

use std::marker::PhantomData;

/// The type of natural numbers (`0..`)
pub trait Nat {}

/// Constant zero (`0`)
pub struct Zero {}

impl Nat for Zero {}

impl<T: Nat> Nat for Succ<T> {}

/// Constant one (`1`)
pub type One = Succ<Zero>;
/// Constant two (`2`)
pub type Two = Succ<One>;
/// Constant three (`3`)
pub type Three = Succ<Two>;
/// Constant four (`4`)
pub type Four = Succ<Three>;
/// Constant five (`5`)
pub type Five = Succ<Four>;
/// Constant six (`6`)
pub type Six = Succ<Five>;
/// Constant seven (`7`)
pub type Seven = Succ<Six>;
/// Constant eight (`8`)
pub type Eight = Succ<Seven>;
/// Constant nine (`9`)
pub type Nine = Succ<Eight>;
/// Constant ten (`10`)
pub type Ten = Succ<Nine>;

/// # Peano-style increment operator
///
/// ```rust
/// # use trait_eval::*;
/// assert_eq!(Succ::<Six>::eval(), 7);
/// ```
pub struct Succ<T> where T: Nat {
    _marker: PhantomData<T>,
}

/// The bool type (`true`, `false`)
pub trait Bool {}

/// True (`true`)
pub struct True {}

impl Bool for True {}

/// False (`false`)
pub struct False {}

impl Bool for False {}

/// # Conditional execution!
///
/// The syntax is kind of clunky, but write `<(Then, Else) as If<Cond>::Result`. The condition can
/// be any boolean expression built using [`True`](struct.True.html), [`False`](struct.False.html),
/// [`AndAlso`](trait.AndAlso.html), [`OrElse`](trait.OrElse.html) and [`Not`](trait.Not.html).
/// Talk about feature complete.
///
/// ```rust
/// # use trait_eval::*;
/// assert_eq!(<(Three, Four) as If<True>>::Result::eval(), 3);
/// assert_eq!(<(Three, Four) as If<False>>::Result::eval(), 4);
/// ```
pub trait If<T: Bool> {
    type Result;
}

impl<U, V> If<True> for (U, V) {
    type Result = U;
}

impl<U, V> If<False> for (U, V) {
    type Result = V;
}

/// # Addition
///
/// Addition at compile-time. That's right. Only limited by your imagination (and maybe your
/// `#[recursion_limit]`).
///
/// ```rust
/// # use trait_eval::*;
/// assert_eq!(<Four as Plus<Three>>::Result::eval(), 7);
/// assert_eq!(<Seven as Plus<One>>::Result::eval(), 8);
/// assert_eq!(<Three as Plus<Three>>::Result::eval(), 6);
/// assert_eq!(<Two as Plus<Three>>::Result::eval(), 5);
/// ```
pub trait Plus<T: Nat>: Nat {
    type Result: Nat;
}

impl<T: Nat> Plus<T> for Zero {
    type Result = T;
}

impl<T: Nat, U: Nat> Plus<T> for Succ<U>
where
    U: Plus<T>,
{
    type Result = Succ<U::Result>;
}

/// # Multiplication
///
/// It's time to get timesing at compile-time! That sounded way cooler in my head.
///
/// ```rust
/// # use trait_eval::*;
/// assert_eq!(<Four as Times<Three>>::Result::eval(), 12);
/// assert_eq!(<Seven as Times<One>>::Result::eval(), 7);
/// assert_eq!(<Three as Times<Three>>::Result::eval(), 9);
/// assert_eq!(<Two as Times<Three>>::Result::eval(), 6);
/// assert_eq!(<Six as Times<Seven>>::Result::eval(), 42);
/// ```
pub trait Times<T: Nat>: Nat {
    type Result: Nat;
}

impl<T: Nat> Times<T> for Zero {
    type Result = Zero;
}

impl<T: Nat, U: Nat, V: Nat> Times<T> for Succ<U>
where
    U: Times<T, Result = V>,
    V: Plus<T>,
{
    type Result = V::Result;
}

/// # Factorial!
///
/// Did I hear shouting? Or was that just this epic compile-time factorial operator?!
///
/// ```rust
/// # #![recursion_limit = "20000000"]
/// # use trait_eval::*;
/// assert_eq!(<Zero as Fact>::Result::eval(), 1);
/// assert_eq!(<One as Fact>::Result::eval(), 1);
/// assert_eq!(<Two as Fact>::Result::eval(), 2);
/// assert_eq!(<Three as Fact>::Result::eval(), 6);
/// assert_eq!(<Four as Fact>::Result::eval(), 24);
/// assert_eq!(<Five as Fact>::Result::eval(), 120);
/// // assert_eq!(<Six as Fact>::Result::eval(), 720); -- `rustc` gives up about here :((
/// ```
///
/// (Ok, fine, yes, the recursion limit is on like 20,000,000.)
pub trait Fact: Nat {
    type Result: Nat;
}

impl Fact for Zero {
    type Result = One;
}

impl<T: Nat, U: Nat> Fact for Succ<T>
where
    T: Fact<Result = U>,
    U: Times<Succ<T>>,
{
    type Result = U::Result;
}

/// # Saturating Decrement
///
/// See, this is engineering at its finest, compile-time execution and no undefined behaviour!
///
/// ```rust
/// # use trait_eval::*;
/// assert_eq!(<Zero as Pred>::Result::eval(), 0);
/// assert_eq!(<One as Pred>::Result::eval(), 0);
/// assert_eq!(<Two as Pred>::Result::eval(), 1);
/// assert_eq!(<Three as Pred>::Result::eval(), 2);
/// assert_eq!(<Four as Pred>::Result::eval(), 3);
/// assert_eq!(<Five as Pred>::Result::eval(), 4);
/// assert_eq!(<Six as Pred>::Result::eval(), 5);
/// assert_eq!(<Seven as Pred>::Result::eval(), 6);
/// assert_eq!(<Eight as Pred>::Result::eval(), 7);
/// assert_eq!(<Nine as Pred>::Result::eval(), 8);
/// assert_eq!(<Ten as Pred>::Result::eval(), 9);
/// ```
pub trait Pred: Nat {
    type Result: Nat;
}

impl Pred for Zero {
    type Result = Zero;
}

impl<T: Nat> Pred for Succ<T> {
    type Result = T;
}

/// # Saturating Subtraction
///
/// It's 2:30AM and I've completely run out of snappy sales pitches. It's subtraction - wow.
///
/// ```rust
/// # use trait_eval::*;
/// assert_eq!(<Ten as Minus<Three>>::Result::eval(), 7);
/// assert_eq!(<Seven as Minus<One>>::Result::eval(), 6);
/// assert_eq!(<Three as Minus<Three>>::Result::eval(), 0);
/// assert_eq!(<Two as Minus<Three>>::Result::eval(), 0);
/// ```
pub trait Minus<T: Nat>: Nat {
    type Result: Nat;
}

impl<T: Nat> Minus<Zero> for T {
    type Result = T;
}

impl<T: Nat, U: Nat, V: Nat> Minus<Succ<T>> for U
where
    U: Minus<T, Result = V>,
    V: Pred,
{
    type Result = V::Result;
}

/// # Remainders
///
/// GCD is left as an exercise for the reader...
/// 
/// ```rust
/// # use trait_eval::*;
/// assert_eq!(<Five as Mod<Three>>::Result::eval(), 2);
/// assert_eq!(<Ten as Mod<Two>>::Result::eval(), 0);
/// assert_eq!(<Seven as Mod<Nine>>::Result::eval(), 7);
/// assert_eq!(<Six as Mod<Six>>::Result::eval(), 0);
/// ```
pub trait Mod<T: Nat>: Nat {
    type Result: Nat;
}

impl<T: Nat> Mod<T> for Zero {
    type Result = Zero;
}

impl<T: Nat, U: Nat, V: Nat, W: Nat, C: Bool> Mod<T> for Succ<U>
where
    Self: Minus<T, Result = V> + LessThan<T, Result = C>,
    V: Mod<T>,
    (Self, <V as Mod<T>>::Result): If<C, Result = W>,
{
    type Result = W;
}

/// # Equality testing
///
/// ```rust
/// # use trait_eval::*;
/// type TwoPlusTwo = <Two as Plus<Two>>::Result;
/// type IsFour = <TwoPlusTwo as Equals<Four>>::Result;
/// assert_eq!(IsFour::eval(), true);
/// type MinusOne = <TwoPlusTwo as Minus<One>>::Result;
/// type ThatsThree = <MinusOne as Equals<Three>>::Result;
/// assert_eq!(ThatsThree::eval(), true); // quick maffs
///
/// assert_eq!(<Zero as Equals<One>>::Result::eval(), false);
/// ```
pub trait Equals<T: Nat> {
    type Result: Bool;
}

impl Equals<Zero> for Zero {
    type Result = True;
}

impl<T: Nat> Equals<Succ<T>> for Zero {
    type Result = False;
}

impl<T: Nat> Equals<Zero> for Succ<T> {
    type Result = False;
}

impl<T: Nat, U: Nat> Equals<Succ<T>> for Succ<U>
where
    T: Equals<U>,
{
    type Result = T::Result;
}

/// # Integer comparison
///
/// I honestly can't be bothered to implement `<=`, `>=` and `>`, so you're just going to have to make
/// do with this and [`Not`](trait.Not.html).
///
/// ```rust
/// # use trait_eval::*;
/// assert_eq!(<Three as LessThan<Five>>::Result::eval(), true);
/// assert_eq!(<Five as LessThan<Five>>::Result::eval(), false);
/// assert_eq!(<Ten as LessThan<Five>>::Result::eval(), false);
/// ```
pub trait LessThan<T: Nat> {
    type Result: Bool;
}

impl LessThan<Zero> for Zero {
    type Result = False;
}

impl<T: Nat> LessThan<Succ<T>> for Zero {
    type Result = True;
}

impl<T: Nat> LessThan<Zero> for Succ<T> {
    type Result = False;
}

impl<T: Nat, U: Nat> LessThan<Succ<T>> for Succ<U>
where
    U: LessThan<T>,
{
    type Result = U::Result;
}

/// # Built-in Fibonacci sequence
///
/// Of course, every programmer uses the Fibonacci sequence on the daily, so we've built it right
/// into the library. Now you don't need to worry about what the nth Fibonacci number is at
/// runtime. We've got you covered.
///
/// ```rust
/// # use trait_eval::*;
/// assert_eq!(<One as Fib>::Result::eval(), 1);
/// assert_eq!(<Two as Fib>::Result::eval(), 1);
/// assert_eq!(<Three as Fib>::Result::eval(), 2);
/// assert_eq!(<Four as Fib>::Result::eval(), 3);
/// assert_eq!(<Five as Fib>::Result::eval(), 5);
/// assert_eq!(<Six as Fib>::Result::eval(), 8);
/// assert_eq!(<Seven as Fib>::Result::eval(), 13);
/// assert_eq!(<Eight as Fib>::Result::eval(), 21);
/// assert_eq!(<Nine as Fib>::Result::eval(), 34);
/// assert_eq!(<Ten as Fib>::Result::eval(), 55);
/// ```
pub trait Fib: Nat {
    type Result: Nat;
}

impl Fib for Zero {
    type Result = Zero;
}

#[doc(hidden)]
pub trait FibRecurse: Nat {
    type Result: Nat;
}

impl<T: Nat, U: Nat, V: Nat, W: Nat, X: Nat, Y: Nat> FibRecurse for T
where
    T: Pred<Result = U> + Minus<Two, Result = V>,
    U: Fib<Result = W>,
    V: Fib<Result = X>,
    W: Plus<X, Result = Y>, 
{
    type Result = Y;
}

impl<T: Nat, U: Bool, V: Nat, W: Nat> Fib for Succ<T>
where
    T: Equals<Zero, Result = U>,
    Succ<T>: FibRecurse<Result = V>,
    (One, V): If<U, Result = W>,
{
    type Result = W;
}

/// # Logical Not
///
/// Negate any boolean you like at compile-time.
///
/// (`¬¬(P V ¬P)` - just saying.)
/// 
/// ```rust
/// # use trait_eval::*;
/// assert_eq!(<True as Not>::Result::eval(), false);
/// assert_eq!(<False as Not>::Result::eval(), true);
/// ```
pub trait Not: Bool {
    type Result: Bool;
}

impl Not for True {
    type Result = False;
}

impl Not for False {
    type Result = True;
}

/// # Logical And
///
/// A logical and operator for all you conjunctive needs. It would short circuit if that were
/// possible, but here we are.
/// 
/// ```rust
/// # use trait_eval::*;
/// assert_eq!(<False as AndAlso<False>>::Result::eval(), false);
/// assert_eq!(<False as AndAlso<True>>::Result::eval(), false);
/// assert_eq!(<True as AndAlso<False>>::Result::eval(), false);
/// assert_eq!(<True as AndAlso<True>>::Result::eval(), true);
/// ```
pub trait AndAlso<T: Bool> {
    type Result: Bool;
}

impl<T: Bool> AndAlso<T> for False {
    type Result = False;
}

impl AndAlso<False> for True {
    type Result = False;
}

impl AndAlso<True> for True {
    type Result = True;
}

/// # Logical Or
///
/// A logical or operator for all you disjunctive needs. It would short circuit if that were
/// possible, but here we are.
/// 
/// ```rust
/// # use trait_eval::*;
/// assert_eq!(<False as OrElse<False>>::Result::eval(), false);
/// assert_eq!(<False as OrElse<True>>::Result::eval(), true);
/// assert_eq!(<True as OrElse<False>>::Result::eval(), true);
/// assert_eq!(<True as OrElse<True>>::Result::eval(), true);
/// ```
pub trait OrElse<T: Bool> {
    type Result: Bool;
}

impl<T: Bool> OrElse<T> for True {
    type Result = True;
}

impl OrElse<False> for False {
    type Result = False;
}

impl OrElse<True> for False {
    type Result = True;
}

/// # `trait_eval` to Rust conversion
///
/// Here's a magic trait for getting your data out of `trait_eval` and into Rust. Don't worry - it
/// doesn't actually evaluate anything at all, I just couldn't think of a better name.
pub trait Eval {
    /// The Rust representation of our type.
    type Output;

    /// A static function to actually grab the data.
    ///
    /// ```rust
    /// # use trait_eval::*;
    /// assert_eq!(Three::eval(), 3);
    /// assert_eq!(<Six as Times<Seven>>::Result::eval(), 42);
    /// ```
    fn eval() -> Self::Output;
}

impl Eval for Zero {
    type Output = usize;

    #[inline]
    fn eval() -> Self::Output {
        0
    }
}

impl<T: Nat> Eval for Succ<T> where T: Eval<Output = usize> {
    type Output = usize;

    #[inline]
    fn eval() -> Self::Output {
        1 + T::eval()
    }
}

impl Eval for True {
    type Output = bool;

    #[inline]
    fn eval() -> Self::Output {
        true
    }
}

impl Eval for False where  {
    type Output = bool;

    #[inline]
    fn eval() -> Self::Output {
        false
    }
}
