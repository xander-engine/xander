use core::hash::Hash;
use std::{collections::{HashMap, HashSet}};

use crate::identity::Identity;

pub trait Proficiency<P> : Identity + Hash + Eq
    where 
        P : Identity + Hash + Eq + ?Sized
{
    fn value(&self) -> &P
        where P : Sized;
}


#[derive(Debug)]
pub struct Proficiencies(HashMap<&'static str, HashSet<&'static str>>);

impl Default for Proficiencies {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Proficiencies {
    pub fn insert<I, P>(&mut self, prof : P) -> &mut Self
    where 
        I : Identity + Hash + Eq,
        P : Proficiency<I>
    {
        let cat = self.0
            .entry(P::__id())
            .or_insert(HashSet::new());

        cat.insert(prof.value().id());
        self
    }

    pub fn has<I, P>(& self, prof : P) -> bool
        where 
            I : Identity + Hash + Eq,
            P : Proficiency<I>
    {
        self.0.get(P::__id())
            .map(|cat| cat.contains(prof.value().id()))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use core::hash::Hash;

    use xander_macros::proficiency;

    use crate::{ability::{skills::{Skill, History}, Ability, Charisma, Checks}, identity::Identity};

    use super::{Proficiency, Proficiencies};


    #[test]
    fn skill_proficiencies() {
        let mut profs = Proficiencies::default();
        profs.insert(Checks(History));
    }
}