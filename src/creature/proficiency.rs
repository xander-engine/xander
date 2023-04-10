use core::hash::Hash;
use std::{collections::HashMap, marker::PhantomData};

use xander_macros::ProficiencyType;

use crate::identity::Identity;

use super::Creature;

///
/// ## Proficiency
/// Container for any category of proficiencies.
///
/// ***
///
/// Use this trait, in co-ordination with the
/// `proficiency!` macro to make a new proficiency:
///
/// ```
/// use xander_macros::proficiency;
/// use xander::{
///     creature::proficiency::Proficiency,
///     identity::Identity,
///     ability::Ability
/// };
///
///
/// proficiency!(SavingThrows(Ability));
/// ```
///
pub trait Proficiency<P>: Identity + Hash + Eq
where
    P: Identity + Hash + Eq + ?Sized,
{
    fn value(&self) -> &P
    where
        P: Sized;
}

///
/// Allows for exotic types of proficiency,
/// (à la Half-Proficiency [Bard], Expertise [Rogue]).
///
/// This allows you to output a custom proficiency
/// bonus for different types of proficiencies.
///
/// ***
///
/// Along with
///
/// ```
/// use xander::{
///     creature::{
///         proficiency::{
///             Proficiency, ProficiencyType,
///             ProficiencyTyped,
///         },
///         Creature,
///     },
///     identity::Identity,
/// };
/// #[derive(Debug)]
/// pub struct Triple;
///
/// impl Identity for Triple {
///     fn id(&self) -> &'static str { "HOMEBREW::PROFICIENCY_TYPE::TRIPLE" }
///     fn __id() -> &'static str
///         where Self : Sized {
///         "HOMEBREW::PROFICIENCY_TYPE::TRIPLE"
///     }
/// }
///
/// impl ProficiencyType for Triple {
///     fn bonus(_ : &Creature, bonus : i32) -> i32 {
///         bonus * 3
///     }
/// }
/// ```
///
pub trait ProficiencyType: Identity {
    ///
    /// Modify the original proficiency bonus,
    /// in some manner.
    ///
    fn bonus(&self, ent: &Creature, prof_bonus: i32) -> i32;

    fn boxed(self) -> Box<dyn ProficiencyType>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

pub struct ProficiencyTyped<T, I, P>(T, P, PhantomData<I>)
where
    I: Identity + Hash + Eq,
    P: Proficiency<I>,
    T: ProficiencyType;

impl<T, I, P> ProficiencyTyped<T, I, P>
where
    I: Identity + Hash + Eq,
    P: Proficiency<I>,
    T: ProficiencyType,
{
    pub fn prof_type(&self) -> &T {
        &self.0
    }

    pub fn prof(&self) -> &P {
        &self.1
    }
}

pub trait IntoProficiencyTyped<T, I, P>
where
    T: ProficiencyType,
    I: Identity + Hash + Eq,
    P: Proficiency<I>,
{
    fn into_proficiency_typed(self) -> ProficiencyTyped<T, I, P>;
}

impl<T, I, P> IntoProficiencyTyped<T, I, P> for ProficiencyTyped<T, I, P>
where
    T: ProficiencyType,
    I: Identity + Hash + Eq,
    P: Proficiency<I>,
{
    fn into_proficiency_typed(self) -> ProficiencyTyped<T, I, P> {
        self
    }
}

#[derive(Debug)]
pub struct Proficiencies(HashMap<&'static str, HashMap<&'static str, Box<dyn ProficiencyType>>>);

impl Default for Proficiencies {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Proficiencies {
    pub fn insert<T, I, P>(&mut self, prof: impl IntoProficiencyTyped<T, I, P>) -> &mut Self
    where
        T: ProficiencyType + 'static,
        I: Identity + Hash + Eq,
        P: Proficiency<I>,
    {
        let ProficiencyTyped(prof_type, prof, _) = prof.into_proficiency_typed();
        let cat = self.0.entry(P::__id()).or_insert(HashMap::new());

        cat.insert(prof.value().id(), prof_type.boxed());
        self
    }

    pub fn has<I, P>(&self, prof: P) -> Option<&dyn ProficiencyType>
    where
        I: Identity + Hash + Eq,
        P: Proficiency<I>,
    {
        self.0
            .get(P::__id())
            .and_then(|cat| cat.get(prof.value().id()).map(Box::as_ref))
    }
}

#[ProficiencyType("5E::PROFICIENCY_TYPE::FULL")]
pub fn Full(&self, _: &Creature, bonus: i32) -> i32 {
    bonus
}

// Default implementation of Proficiency Type - Full Proficiency

impl<I, P> IntoProficiencyTyped<Full, I, P> for P
where
    I: Identity + Hash + Eq,
    P: Proficiency<I>,
{
    fn into_proficiency_typed(self) -> ProficiencyTyped<Full, I, P> {
        ProficiencyTyped(Full, self, PhantomData::<I>)
    }
}

///
/// Half-proficiency: from the "Jack of All Trades" feat.
///
#[ProficiencyType("5E::PROFICIENCY_TYPE::Half")]
pub fn Half(&self, _: &Creature, bonus: i32) -> i32 {
    bonus.div_floor(2)
}

///
/// Expertise from the Rogue features.
///
#[ProficiencyType("5E::PROFICIENCY_TYPE::EXPERTISE")]
pub fn Expertise(&self, _: &Creature, bonus: i32) -> i32 {
    bonus * 2
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        ability::{
            skills::{History, Persuasion, Stealth},
            Charisma, Checks, Constitution, Dexterity, Intelligence, Strength, Wisdom,
        },
        creature::{
            proficiency::{IntoExpertise, IntoHalf},
            Creature,
        },
        dice::{modifiers::Advantage, D20},
        identity::Identity,
    };

    use super::Proficiencies;

    #[test]
    fn skill_proficiencies() {
        let mut rogue = Creature(HashMap::new(), Proficiencies::default());

        rogue.stats().extend(vec![
            (Strength.id(), 2),
            (Dexterity.id(), 20),
            (Constitution.id(), 10),
            (Intelligence.id(), 12),
            (Wisdom.id(), 17),
            (Charisma.id(), 13),
        ]);

        rogue
            .proficiencies()
            .insert(Checks(Persuasion))
            .insert(Checks(Stealth).expertise())
            .insert(Checks(History).half());

        println!("{:?}", rogue.check(Stealth).then(Advantage(D20)));

        println!("{:?}", rogue.check(Persuasion));

        println!("{:?}", rogue.check(History));

        // println!("{rogue:?}")
    }
}
