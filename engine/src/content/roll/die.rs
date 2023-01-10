use rand::{thread_rng, Rng};

pub type RollValue = i32;

///
/// A standard die.
/// 
/// This engine comes with the standard 7-set of dice:
/// `D4`, `D6`, `D8`, `D10`
/// `D12`, `D20`, `D100`
/// 
/// Along with an n-sided die: `D(n)`
/// 
/// ## Example
/// 
/// Making your own die.
/// ```
/// use xander::content::Die;
/// 
/// pub struct Coin;
/// impl Die for Coin {
///     fn sides(&self) -> usize { 2 }
/// }
/// 
/// fn main() {
///     let coin = Coin;
///     println!("{}", coin.roll());
/// }
/// ```
/// 
pub trait Die {
    fn sides(&self) -> usize;
    fn roll(&self)   -> RollValue {
        thread_rng().gen_range(1..=self.sides()) as RollValue
    }
}

///
/// Generic `n`-sided die.
/// 
/// ## Example
/// ```
/// use xander::content::{D, Die};
/// 
/// fn main() {
///     let coin = D(2);
///     println!("{}", 
///         match coin.roll() {
///             1 => "Tails",
///             2 => "Heads",
///             _ => unreachable!("Roll can only be 1 or 2!")
///         }
///     );
/// }
/// ```
/// 
pub struct D(pub usize);
// Some people may hate the public access above,
//  but I believe that the `D(n)` syntax is more
// elegant than the alternate `D::new(n)`.


impl Die for D {
    fn sides(&self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{D, Die};

    #[test]
    fn test_roll() {
        let die = D(10);
        println!("Rolled a custom die, result: {}", die.roll());
    }
}