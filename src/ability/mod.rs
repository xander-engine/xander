use xander_macros::abilities;

pub mod skills;

pub trait Checkable {}

pub trait Saveable {}

pub trait Ability : Checkable + Saveable {
    fn id(&self) -> &'static str;
}

abilities!(Strength, Dexterity, Constitution, Intelligence, Wisdom, Charisma);
