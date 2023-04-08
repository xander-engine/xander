use xander_macros::skills;

use super::{Ability, Checkable, Strength, Intelligence, Dexterity, Wisdom, Charisma};

pub trait Skill : Checkable {
    fn id(&self) -> &'static str;
    fn base(&self) -> Box<dyn Ability>;
}

skills!(
    Strength(Athletics),
    Dexterity(Acrobatics, SleightOfHand),
    Intelligence(Arcana, History, Investigation, Nature, Religion),
    Wisdom(AnimalHandling, Insight, Medicine, Perception, Survival),
    Charisma(Deception, Intimidation, Performance, Persuasion)
);