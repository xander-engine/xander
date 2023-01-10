use super::identity::Identity;

pub struct Namespace(&'static str);

impl Namespace {
    pub fn lookup<'a, I : Identity + ?Sized>(&self, _id : &'a str) -> &'a I {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::content::{self, Language};

    use super::Namespace;

    #[test]
    fn apples() {
        let FIVE_E = Namespace("5E");
        let goblin : &dyn Language = FIVE_E.lookup("5E::LANGUAGE::GOBLIN");
    }
}