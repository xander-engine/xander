use std::{
    collections::HashMap,
    ops::{Add, Div, Index, Mul, Sub},
};

use crate::dice::Die;

use super::modifiers::{IntoModifier, Modifier};

///
/// Internal type for a die roll.
///
type RollInner = i32;

///
/// Represents the result of rolling a single die.
///
#[derive(Clone)]
pub struct Roll {
    value: RollInner,

    ///
    /// Hidden roles are not accounted for in totals.
    ///
    hidden: bool,
}

impl std::fmt::Debug for Roll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Roll({})",
            &self
                .hidden
                .then(|| "_".to_string())
                .unwrap_or(self.value.to_string())
        )
    }
}

impl core::cmp::PartialOrd for Roll {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl core::cmp::PartialEq for Roll {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl From<RollInner> for Roll {
    fn from(value: RollInner) -> Self {
        Self {
            value,
            hidden: false,
        }
    }
}

impl Roll {
    pub fn hide(&mut self) {
        self.hidden = true;
    }

    pub fn show(&mut self) {
        self.hidden = false;
    }

    pub fn hidden(&self) -> bool {
        self.hidden
    }

    pub fn value(&self) -> RollInner {
        self.hidden.then_some(self.value).unwrap_or(0)
    }
}

pub type RollType = Roll;

///
/// Container for a number of dice rolls.
///
#[derive(Debug, Default)]
pub struct Rolls {
    raw_rolls: HashMap<usize, Vec<RollType>>,
    modifiers: Vec<Box<dyn Modifier>>,
}

impl Rolls {
    ///
    /// Gets the rolls associated with a die.
    ///
    pub fn get(&self, die: &(impl Die + ?Sized)) -> impl Iterator<Item = &Roll> {
        self.raw_rolls
            .get(&die.sides())
            .map(|v| v.iter())
            .unwrap_or([].iter())
    }

    ///
    /// Adds rolls associated with a die.
    ///
    pub fn add(
        &mut self,
        die: &(impl Die + ?Sized),
        values: impl IntoIterator<Item = RollType>,
    ) -> &mut Self {
        self.raw_rolls
            .entry(die.sides())
            .or_default()
            .extend(values);

        self
    }

    ///
    /// Add a modifier.
    ///
    pub fn then(mut self, modifier: impl IntoModifier) -> Self {
        self.modifiers.push(modifier.into_modifier());
        self
    }

    ///
    /// Peek at what the total is,
    /// without needing ownership of `self`.
    ///
    pub fn peek(&self) -> i32 {
        let mut raw = self.raw_rolls.clone();
        for modifier in self.modifiers.iter() {
            let v = raw.iter_mut().collect();
            match modifier.apply(v) {
                Some(s) => return s,
                None => {}
            }
        }

        raw.into_iter()
            .map(|(_, v)| v.iter().map(RollType::value).sum::<i32>())
            .sum::<i32>()
    }

    ///
    /// Apply all modifiers,
    /// Returns Ok(i32), or Err(Self)
    ///     depending on the presence of arithmetic methods.
    ///
    pub fn apply(mut self) -> Result<i32, Self> {
        for modifier in self.modifiers.iter() {
            let v = self.raw_rolls.iter_mut().collect();
            match modifier.apply(v) {
                Some(s) => return Ok(s),
                None => {}
            }
        }

        Err(self)
    }

    ///
    /// 👀 Peek time.
    ///
    /// Allows for a function to use an immutable reference to this [Roll].
    ///
    pub fn inspect(self, func: impl Fn(&Self) -> ()) -> Self {
        func(&self);
        self
    }

    ///
    /// Calculate the total of these rolls,
    /// consuming this object.
    ///
    pub fn total(self) -> i32 {
        self.peek()
    }

    ///
    /// Adds all the rolls from the other [Rolls] collection.
    ///
    pub fn extend(mut self, other: Self) -> Self {
        self.raw_rolls.extend(other.raw_rolls.into_iter());

        self.modifiers.extend(other.modifiers);

        self
    }
}

impl Add<i32> for Rolls {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        self.then(|x| x + rhs)
    }
}

impl Sub<i32> for Rolls {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        self.then(|x| x - rhs)
    }
}

impl Mul<i32> for Rolls {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        self.then(|x| x * rhs)
    }
}

impl Div<i32> for Rolls {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        self.then(|x| x / rhs)
    }
}

// Utility for concat. two rolls.

impl Add<Rolls> for Rolls {
    type Output = Self;

    fn add(self, rhs: Rolls) -> Self::Output {
        self.extend(rhs)
    }
}

impl<D: Die> Index<D> for Rolls {
    type Output = [Roll];

    fn index(&self, index: D) -> &Self::Output {
        self.raw_rolls
            .get(&index.sides())
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
}

#[cfg(test)]
mod tests {
    use crate::dice::{
        rolls::{Roll, RollType},
        D20, D4,
    };

    use super::Rolls;

    #[test]
    fn get() {
        let collection = Rolls::default();
        let v = collection.get(&D20).collect::<Vec<_>>();
        assert_eq!(v, Vec::<&RollType>::new());
    }

    #[test]
    fn add() {
        let mut collection = Rolls::default();
        let v = vec![1, 2, 3, 4]
            .into_iter()
            .map(Roll::from)
            .collect::<Vec<_>>();

        collection.add(&D20, v.clone());

        assert_eq!(
            collection.get(&D20).map(|a| a.clone()).collect::<Vec<_>>(),
            v
        );
    }

    #[test]
    fn modifiers() {
        let results = D20(13).then(|s| s + 2);

        assert_eq!(results.modifiers.len(), 1);
    }

    #[test]
    fn indexing() {
        let results = D20(12) + D4(13);
        assert_eq!(13, results[D4].len())
    }
}
