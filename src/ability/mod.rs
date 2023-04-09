use core::hash::Hash;

use xander_macros::{abilities, proficiency};

use crate::{creature::{proficiency::{Proficiency}, Creature}, identity::Identity};

pub mod skills;

pub trait Check : Identity {
    fn base() -> Box<dyn Ability>
        where Self : Sized;
}
proficiency!(Checks(Check));

pub trait Save {}
proficiency!(Saves(Ability));

pub trait Ability : Check + Save + Identity {
    fn default() -> Self 
        where Self : Sized;
}

abilities!(Strength, Dexterity, Constitution, Intelligence, Wisdom, Charisma);
