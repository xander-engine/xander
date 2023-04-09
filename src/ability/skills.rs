use core::hash::Hash;

use xander_macros::skills;

use crate::{creature::proficiency::{Proficiency}, identity::Identity};

use super::{Ability, Check, Strength, Intelligence, Dexterity, Wisdom, Charisma};

pub trait Skill : Check + Identity {
    fn base(&self) -> Box<dyn Ability>;
}

skills!(
    Strength(Athletics),
    Dexterity(Acrobatics, SleightOfHand),
    Intelligence(Arcana, History, Investigation, Nature, Religion),
    Wisdom(AnimalHandling, Insight, Medicine, Perception, Survival),
    Charisma(Deception, Intimidation, Performance, Persuasion)
);