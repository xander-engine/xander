pub mod proficiency;

use std::{collections::{HashMap, HashSet}, any::Any, hash::Hash, ops::Div};

use crate::{ability::{Check, Save, Ability, Saves, Checks}, identity::Identity, dice::{D20, Rolls}};

use self::proficiency::{Proficiency, Proficiencies};

#[derive(Debug)]
pub struct Creature(HashMap<&'static str, usize>, Proficiencies);

impl Creature {
    fn proficency_modifier(&self) -> i32 { 2 }

    fn score<A : Ability + ?Sized, T : Into<Box<A>>>(&self, ability : T) -> Option<usize> {
        let ability : Box<A> = ability.into();
        self.0.get(&ability.id())
            .copied()
    }

    fn modifier<A : Ability + ?Sized, T : Into<Box<A>>>(&self, ability :T) -> Option<i32> {
        self.score(ability)
            .map(|a| (a as i32 - 10).div(2))
    }

    fn check<C : Check + Hash + Eq>(&self, metric : C) -> Rolls 
    {
        D20() 
            + self.modifier::<dyn Ability, Box<dyn Ability>>(C::base()).unwrap()
            + self.proficient(Checks(metric))
                .then_some(self.proficency_modifier())
                .unwrap_or(0)
    }

    fn save<S : Save + Ability + Hash + Eq>(&self, metric : S) -> Rolls 
    {
        D20() + self.proficient(Saves(metric))
            .then_some(self.proficency_modifier())
            .unwrap_or(0)
    }

    fn proficient<I, P>(&self, prof : P) -> bool 
        where 
            I : Identity + Hash + Eq,
            P : Proficiency<I>
    {
        self.1
            .has(prof)
    }

    fn proficiencies(&mut self) -> &mut Proficiencies {
        &mut self.1
    }

    fn stats(&mut self) -> &mut HashMap<&'static str, usize> {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {

    use std::{collections::HashMap};

    use crate::{ability::{skills::{History, Skill, Performance}, Strength, Ability, Dexterity, Checks, Constitution, Intelligence, Wisdom, Charisma}, creature::proficiency::Proficiency, identity::Identity};

    use super::{Creature, proficiency::Proficiencies};

    #[test]
    fn check_and_save() {
        let mut ent = Creature(HashMap::default(), Proficiencies::default());

        ent.stats()
            .extend(
                vec![
                    (Strength.id(), 2), (Dexterity.id(), 5),
                    (Constitution.id(), 10), (Intelligence.id(), 12),
                    (Wisdom.id(), 17), (Charisma.id(), 13),
                ]
            );

        ent
            .proficiencies()
            .insert(Checks(Performance));

        println!("{ent:?}");

        println!("{:?}", ent.check(History));
    }

    #[test]
    fn proficiency() {
        let mut ent = Creature(HashMap::default(), Proficiencies::default());

        // ent.proficiencies()
        //     .insert(Dexterity)
        //     .insert(History);

        println!("{:?}", ent.proficiencies());

        // println!("Proficient in DEXTERITY, {}", Dexterity.proficient(&ent));
    }
}