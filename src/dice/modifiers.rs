use std::ops::{Add, Sub, Mul, Div};

use super::{rolls::RollType, Roll, Die};

///
/// Represents an arithmetic operation. 
/// 
#[derive(Debug, PartialEq)]
pub enum Arithmetic {
    Add(i32),
    Sub(i32),
    Mul(i32),
    Div(i32)
}

///
/// Algebraic variable to allow for closures
/// to be auto-magically converted to [Modifier]s.
/// 
#[derive(Debug, PartialEq)]
pub struct Var {
    last_op : Option<Arithmetic>
}

impl const Add<i32> for Var {
    type Output = Self;

    fn add(mut self, rhs: i32) -> Self::Output {
        self.last_op.replace(Arithmetic::Add(rhs));
        self
    }
}
impl const Sub<i32> for Var {
    type Output = Self;

    fn sub(mut self, rhs: i32) -> Self::Output {
        self.last_op.replace(Arithmetic::Sub(rhs));
        self
    }
}
impl const Mul<i32> for Var {
    type Output = Self;

    fn mul(mut self, rhs: i32) -> Self::Output {
        self.last_op.replace(Arithmetic::Mul(rhs));
        self
    }
}
impl const Div<i32> for Var {
    type Output = Self;

    fn div(mut self, rhs: i32) -> Self::Output {
        self.last_op.replace(Arithmetic::Div(rhs));
        self
    }
}

impl const Default for Var {
    fn default() -> Self {
        Self { last_op: None }
    }
}

pub trait IntoModifier {
    fn into_modifier(&self) -> Box<dyn Modifier>;
}

impl<F : ~const Fn(Var) -> Var> IntoModifier for F
{
    fn into_modifier(&self) -> Box<dyn Modifier>
        where Self : Sized
    {
        Box::new({
            let x  = self(Var::default());
            x.last_op.unwrap()
        })
    }
} 

impl Modifier for Arithmetic {
    fn id(&self) -> &'static str {
        use Arithmetic::*;

        match self {
            Add(_) => "OPERATIONS::ADD",
            Sub(_) => "OPERATIONS::SUB",
            Mul(_) => "OPERATIONS::MUL",
            Div(_) => "OPERATIONS::DIV",
        }
    }

    fn symbol(&self) -> Option<&'static str> {
        use Arithmetic::*;

        match self {
            Add(_) => "+",
            Sub(_) => "-",
            Mul(_) => "*",
            Div(_) => "/",
        }.into() 
    }

    fn apply(
        &self, 
        raw_rolls : Vec<(& usize, &mut Vec<RollType>)>
    ) -> Option<i32> {
        use Arithmetic::*;

        let subtotal = raw_rolls.into_iter()
            .map(|(_, v)| 
                v.iter().map(RollType::value).sum::<i32>()
            ).sum::<i32>();

        match self {
            Add(p) => Some(subtotal + p),
            Sub(p) => Some(subtotal - p),
            Mul(p) => Some(subtotal * p),
            Div(p) => Some(subtotal / p),
        }
    }   
}


///
/// An operation on dice rolls,
/// after the result is known.
/// 
/// Can be:
/// * mathematical;
/// * arbitrary code
/// 
pub trait Modifier : std::fmt::Debug {
    ///
    /// Name for this operation.
    /// 
    fn id(&self) -> &'static str;

    ///
    /// If arithmetic, a symbol for this operation.
    /// 
    fn symbol(&self) -> Option<&'static str>;

    fn is_arithmetic(&self) -> bool {
        self.symbol().is_some()
    }

    ///
    /// Operation itself, returns Some(i32)
    /// to be a sum, or None if it modifies the Iterator
    /// in place.
    /// 
    fn apply(
        &self, 
        raw_rolls : Vec<(& usize, &mut Vec<RollType>)>
    ) -> Option<i32>;
}

#[derive(Debug, Clone, Copy)]
pub struct Advantage<D : Die + std::fmt::Debug + Copy>(pub D);

    impl<D : Die + std::fmt::Debug + Copy + 'static> IntoModifier for Advantage<D> {
        fn into_modifier(&self) -> Box<dyn Modifier> {
            Box::new(*self)
        }
    }

    impl<D : Die + std::fmt::Debug + Copy> Modifier for Advantage<D> {
        fn id(&self) -> &'static str {
            "5E::ADVANTAGE"
        }

        fn symbol(&self) -> Option<&'static str> {
            None
        }

        fn apply(
            &self, 
            raw_rolls : Vec<(& usize, &mut Vec<crate::dice::rolls::RollType>)>
        ) -> Option<i32> {
            raw_rolls
                .into_iter()
                .for_each(|(sides, rolls)| {
                    match *sides == self.0.sides() {
                        true  => {
                            rolls
                                .iter_mut()
                                .for_each(Roll::hide);

                            rolls
                                .iter_mut()
                                .max_by_key(|a| a.value())
                                .map(Roll::show)
                                .unwrap_or(());
                    
                        },
                        false => {
                            rolls
                                .iter_mut()
                                .for_each(Roll::hide)
                        }
                    }
                });

            None
        }
    }


#[cfg(test)]
mod tests {
    use crate::dice::{modifiers::{Modifier, IntoModifier}, Die, rolls::Roll, D20};

    use super::Advantage;

    
    #[test]
    fn advantage() {
        D20(2)
            .then(Advantage(D20))
            .inspect(|r| println!("{r:?}"))
            .apply()
            .inspect_err(|r| println!("{r:?}"))
            .unwrap_or_default();
    }
}