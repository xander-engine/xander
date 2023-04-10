//!
//! Collection of dice helper structs.
//!
//! ### Examples
//! ***
//! Using preset dice:
//! ```
//! use xander::dice::*;
//!
//! fn main() {
//!     let d6 = D6;        // Make a new D6.
//!     let damage = d6(2); // Roll twice.
//!     println!("You shall take {} damage!", damage.total())
//! }
//! ```
//! ***
//! Using the generic, `N`-sided die:
//! ```
//! use xander::dice::*;
//!
//! fn main() {
//!     let sides = 123;
//!     let die = D(sides);    // 123-sided dice.
//!     let results = die(10); // Roll 10 times.
//!     println!("Results : {results:?}")
//! }
//! ```

pub mod modifiers;
mod rolls;

use std::ops::{Add, Div, Mul, Sub};

use rand::Rng;
use xander_macros::dice;

pub use rolls::{Roll, Rolls};

///
/// Supertrait for all dice.
///
/// Common implementation of roll.
///
pub trait Die :
    // Fn Traits for the D20(n) syntax.
    FnOnce<(), Output = Rolls>       +
    FnOnce<(usize,), Output = Rolls> +
    Fn<()> + Fn<(usize,)>               +

    // Arithmetic
    Add<i32, Output = Rolls> +
    Sub<i32, Output = Rolls> +
    Mul<i32, Output = Rolls> +
    Div<i32, Output = Rolls>
{
    ///
    /// How many sides are on this die?
    /// 
    fn sides(&self) -> usize;

    ///
    /// Roll this die `n` times.
    /// 
    fn roll(&self, times : usize) -> Rolls {
        let mut rng = rand::thread_rng();

        let mut r = Rolls::default();
        Rolls::add(
            &mut r,
            self,
            (0..times)
                .map(move |_| rng.gen_range(1..=self.sides()) as i32)
                .map(Roll::from)
        );

        r
    }
}

dice!(4, 6, 8, 10, 12, 20, 100);

///
/// A generic `n`-sided die.
///
/// ### Example
/// ```
/// use xander::dice::*;
///
/// fn main() {
///     let sides = 123;
///     let die = D(sides);    // 123-sided dice.
///     let results = die(10); // Roll 10 times.
///     println!("Results : {results:?}")
/// }
/// ```
///
#[derive(Debug, Clone, Copy)]
pub struct D(pub usize);

impl Die for D {
    fn sides(&self) -> usize {
        self.0
    }
}

impl FnOnce<()> for D {
    type Output = Rolls;

    extern "rust-call" fn call_once(self, _: ()) -> Self::Output {
        self.roll(0)
    }
}

impl FnMut<()> for D {
    extern "rust-call" fn call_mut(&mut self, _: ()) -> Self::Output {
        self.roll(0)
    }
}

impl Fn<()> for D {
    extern "rust-call" fn call(&self, _: ()) -> Self::Output {
        self.roll(0)
    }
}

impl FnOnce<(usize,)> for D {
    type Output = Rolls;

    extern "rust-call" fn call_once(self, args: (usize,)) -> Self::Output {
        self.roll(args.0)
    }
}

impl FnMut<(usize,)> for D {
    extern "rust-call" fn call_mut(&mut self, args: (usize,)) -> Self::Output {
        self.roll(args.0)
    }
}

impl Fn<(usize,)> for D {
    extern "rust-call" fn call(&self, args: (usize,)) -> Self::Output {
        self.roll(args.0)
    }
}

impl Add<i32> for D {
    type Output = Rolls;

    fn add(self, rhs: i32) -> Self::Output {
        self.roll(1).then(|x| x + rhs)
    }
}
impl Sub<i32> for D {
    type Output = Rolls;

    fn sub(self, rhs: i32) -> Self::Output {
        self.roll(1).then(|x| x - rhs)
    }
}
impl Mul<i32> for D {
    type Output = Rolls;

    fn mul(self, rhs: i32) -> Self::Output {
        self.roll(1).then(|x| x * rhs)
    }
}
impl Div<i32> for D {
    type Output = Rolls;

    fn div(self, rhs: i32) -> Self::Output {
        self.roll(1).then(|x| x / rhs)
    }
}

impl Into<Rolls> for D {
    fn into(self) -> Rolls {
        self.roll(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dyn_dispatch() {
        let v: Vec<Box<dyn Die>> = vec![Box::new(D4), Box::new(D6)];
        let result = v.iter().map(|d| d.roll(1)).collect::<Vec<_>>();
        println!("{result:?}")
    }

    #[test]
    fn dice_arithmetic() {
        let results = D20() + 23;
        println!("{results:?}")
    }
}
