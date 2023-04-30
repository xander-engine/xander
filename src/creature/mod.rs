pub mod proficiency;

use std::{
    collections::HashMap,
    hash::Hash,
};

use crate::{
    ability::{Ability, Check, Checks, Save, Saves},
    dice::{Rolls, D20},
    identity::Identity,
};

use self::proficiency::{Proficiencies, Proficiency, ProficiencyType};

#[derive(Debug)]
pub struct Creature(HashMap<&'static str, usize>, Proficiencies);

#[allow(unused)]
impl Creature {
    fn proficency_modifier(&self) -> i32 {
        2
    }

    fn score<A>(&self, ability: impl Into<Box<A>>) -> Option<usize>
    where 
        A : Ability + ?Sized,
    {
        let ability: Box<A> = ability.into();
        self.0.get(&ability.id()).copied()
    }

    fn modifier<A>(&self, ability: impl Into<Box<A>>) -> Option<i32>
    where
        A : Ability + ?Sized,
    {
        self.score(ability).map(|a| (a as i32 - 10).div_floor(2))
    }

    fn check<C>(&self, metric: C) -> Rolls 
    where 
        C :  Check + Hash + Eq,
    {
        let mut r = D20()
            + self
                .modifier::<dyn Ability>(C::base())
                .unwrap();

        if let Some(prof) = self
            .proficient(Checks(metric))
            .map(|t| t.bonus(self, self.proficency_modifier()))
        {
            r = r + prof;
        }
        
        r
    }

    fn save<S>(&self, metric: S) -> Rolls 
    where 
        S : Save + Ability + Hash + Eq,
    {
        D20()
            + self
                .proficient(Saves(metric))
                .map(|t| t.bonus(self, self.proficency_modifier()))
                .unwrap_or(0)
    }

    pub fn proficient<I, P>(&self, prof: P) -> Option<&dyn ProficiencyType>
    where
        I: Identity + Hash + Eq,
        P: Proficiency<I>,
    {
        self.1.has(prof)
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

    use std::collections::HashMap;

    use crate::{
        ability::{
            skills::{History, Performance},
            Charisma, Checks, Constitution, Dexterity, Intelligence, Strength, Wisdom, Saves,
        },
        identity::Identity,
    };

    use super::{proficiency::Proficiencies, Creature};

    #[test]
    fn check_and_save() {
        let mut ent = Creature(HashMap::default(), Proficiencies::default());

        ent.stats().extend(vec![
            (Strength.id(), 2),
            (Dexterity.id(), 5),
            (Constitution.id(), 10),
            (Intelligence.id(), 12),
            (Wisdom.id(), 17),
            (Charisma.id(), 13),
        ]);

        ent
            .proficiencies()
                .insert(Checks(Performance));

        println!("{ent:?}");

        println!("{:?}", ent.check(History));
    }

    #[test]
    fn proficiency() {
        let mut ent = Creature(HashMap::default(), Proficiencies::default());

        ent.proficiencies()
            .insert(Saves(Dexterity))
            .insert(Checks(History));

        println!("{:?}", ent.proficiencies());

        // println!("Proficient in DEXTERITY, {}", Dexterity::proficient(&ent));
    }
}
