use crate::utils::identity::Identity;

use super::ability::Ability;

pub trait Skill : Identity {
    fn id() -> &'static str
        where Self : Sized;

    fn basis(&self) -> &'static dyn Ability;
}