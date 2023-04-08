//!
//! Collection of dice helper structs.
//! 
//! ### Examples
//! ***
//! Using preset dice:
//! ```
//! use fifth_types::dice::*;
//! 
//! fn main() {
//!     let d6 = D6;        // Make a new D6.
//!     let damage = d6(2); // Roll twice.
//!     println!("You shall take {} damage!", damage.iter().sum::<i32>())
//! }
//! ```
//! ***
//! Using the generic, `N`-sided die:
//! ```
//! use fifth_types::dice::*;
//! 
//! fn main() {
//!     let sides = 123;
//!     let die = D(sides);    // 123-sided dice.
//!     let results = die(10); // Roll 10 times.
//!     println!("Results : {results:?}")
//! }
//! ```

use rand::Rng;
use xander_macros::dice;

///
/// Supertrait for all dice.
/// 
/// Common implementation of roll.
/// 
pub trait Die : Fn<()> + Fn<(usize,)> + Fn<(Option<usize>,)> {
    ///
    /// How many sides are on this die?
    /// 
    fn sides(&self) -> usize;

    ///
    /// Roll this die `n` times.
    /// Accepts `None`, and `Some(n)` too!
    /// 
    fn roll(&self, times : impl Into<Option<usize>>) -> Vec<i32> {
        let times : usize = times.into().unwrap_or(1);
        let mut rng = rand::thread_rng();

        (0..times)
            .map(move |_| rng.gen_range(1..=self.sides()) as i32)
            .collect()
    }
}

dice!(4, 6, 8, 10, 12, 20, 100);

///
/// A generic `n`-sided die.
/// 
/// ### Example
/// ```
/// use fifth_types::dice::*;
/// 
/// fn main() {
///     let sides = 123;
///     let die = D(sides);    // 123-sided dice.
///     let results = die(10); // Roll 10 times.
///     println!("Results : {results:?}")
/// }
/// ```
/// 
pub struct D(pub usize);

impl Die for D {
    fn sides(&self) -> usize {
        self.0
    }
}

impl FnOnce<()> for D {
    type Output = Vec<i32>;

    extern "rust-call" fn call_once(self, _: ()) -> Self::Output {
        self.roll(None)
    }
}

impl FnMut<()> for D {
    extern "rust-call" fn call_mut(&mut self, _: ()) -> Self::Output {
        self.roll(None)
    }
}
 
impl Fn<()> for D {
    extern "rust-call" fn call(&self, _: ()) -> Self::Output {
        self.roll(None)
    }
}

impl<I : Into<Option<usize>>> FnOnce<(I,)> for D {
    type Output = Vec<i32>;

    extern "rust-call" fn call_once(self, args: (I,)) -> Self::Output {
        self.roll(args.0)
    }
} 

impl<I : Into<Option<usize>>> FnMut<(I,)> for D {
    extern "rust-call" fn call_mut(&mut self, args: (I,)) -> Self::Output {
        self.roll(args.0)
    }
}
impl<I : Into<Option<usize>>> Fn<(I,)> for D {
    extern "rust-call" fn call(&self, args: (I,)) -> Self::Output {
        self.roll(args.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dice() {
        let d20 = D20;
        let result = d20();
        println!("{result:?}");
    }
}