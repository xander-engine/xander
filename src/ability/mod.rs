use xander_macros::{abilities, Proficiency};

use crate::{creature::proficiency::Proficiency, identity::Identity};

pub mod skills;

///
/// Metrics that checks can be asked for.
///
/// Under normal 5E rules, these are
/// abilities ([Ability]), and skills ([skills::Skill]).  
///
/// ***
/// A custom metric can be made, as long as it has a *valid*
/// base ability.
///
pub trait Check: Identity {
    ///
    /// Return the base ability
    /// concerning this check.
    /// ***
    /// **Example**: [skills::Stealth] => [Dexterity]
    ///
    fn base() -> Box<dyn Ability>
    where
        Self: Sized;
}

pub trait Save {}

pub trait Ability: Check + Save + Identity {
    fn default() -> Self
    where
        Self: Sized;
}

///
/// Proficiency in (skill) checks.
///
#[Proficiency("5E::PROFICIENCY::CHECKS")]
pub struct Checks(Check);

///
/// Proficiency in saving throws.
///
#[Proficiency("5E::PROFICIENCY::SAVES")]
pub struct Saves(Ability);

abilities!(
    ///
    /// Test 1
    ///
    Strength,
    ///
    /// Test 2
    ///
    Dexterity,
    ///
    /// Test 3
    ///
    Constitution,
    ///
    /// Test 4
    ///  
    Intelligence,
    ///
    /// Test 5
    ///  
    Wisdom,
    ///
    /// Test 6
    ///
    Charisma
);
