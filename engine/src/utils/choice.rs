use std::slice::Iter;

use super::identity::Identity;

pub struct Choice<I: Identity + ?Sized + 'static, const N : usize>([&'static I ; N]);

impl<I: Identity + ?Sized + 'static, const N : usize> Choice<I, N> {
    pub fn choose(&self, option : usize) -> Option<&'static I> {
        match option {
            o if o < N  => Some(self.0[option]),
            _ => None,
        }
    }

    pub const fn len(&self) -> usize {
        N
    } 

    pub fn iter<'a>(&'a self) -> Iter<'a, &I> {
        self.0.iter()
    }
}

pub struct Choices;
impl Choices {
    pub fn of<
        I: Identity + ?Sized + 'static,
        const N : usize
    >(arr : [&'static I ; N]) -> Choice<I, N> {
        Choice(arr)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::{Display, Debug};

    use crate::utils::identity::{Identity, ToAny};

    use super::Choice;

    trait Ability : Identity {}

    impl<'a> Display for &'a dyn Ability {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Ability [{}]", self.id()
            )
        }
    }

    impl<'a> Debug for &'a dyn Ability {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f, "{}", self.id()
            )
        }
    }

    struct Charisma;

    impl Ability for Charisma {}
    impl Identity for Charisma {
        fn id(&self) -> &'static str {
            "5E::ABILITY::CHARISMA"
        }
    }
    struct Strength;

    impl Ability for Strength {}
    impl Identity for Strength {
        fn id(&self) -> &'static str {
            "5E::ABILITY::STRENGTH"
        }
    }

    #[test]
    fn choice_ability() {
        let c : Choice<dyn Ability, _> = Choice([&Charisma, &Strength]);
        let v : Vec<_> = c.iter()
            .map(|a| a.id())
            .collect();

        assert_eq!(v, vec!["5E::ABILITY::CHARISMA", "5E::ABILITY::STRENGTH"])
    }
}