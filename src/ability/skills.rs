use core::hash::Hash;

use xander_macros::skills;

use crate::identity::Identity;

use super::{Ability, Charisma, Check, Dexterity, Intelligence, Strength, Wisdom};

pub trait Skill: Check + Identity {
    fn base(&self) -> Box<dyn Ability>;
}

skills!(
    Strength(
        ///
        /// Test 1
        ///
        Athletics
    ),
    Dexterity(
        ///
        /// Test 2
        ///
        Acrobatics,
        ///
        /// Test 3
        ///  
        SleightOfHand,
        ///
        /// Test 4
        ///
        Stealth
    ),
    Intelligence(
        ///
        /// Test 5
        ///
        Arcana,
        ///
        /// Test 6
        ///
        History,
        ///
        /// Test 7
        ///
        Investigation,
        ///
        /// Test 8
        ///  
        Nature,
        ///
        /// Test 9
        ///
        Religion
    ),
    Wisdom(
        ///
        /// Test 10
        ///
        AnimalHandling,
        ///
        /// Test 11
        ///
        Insight,
        ///
        /// Test 12
        ///
        Medicine,
        ///
        /// Test 13
        ///
        Perception,
        ///
        /// Test 14
        ///  
        Survival
    ),
    Charisma(
        ///
        /// Test 15
        ///
        Deception,
        ///
        /// Test 16
        ///
        Intimidation,
        ///
        /// Test 17
        ///  
        Performance,
        ///
        /// Test 18
        ///  
        Persuasion
    )
);
