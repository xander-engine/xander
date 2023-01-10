use crate::utils::identity::Identity;

pub trait Condition : Identity {
    // TODO: Add things
}

#[cfg(test)]
mod tests {
    use crate::{content::entity::Condition, utils::identity::Identity};

    #[test]
    fn exhaustion() {
        struct Exhaustion(u8);

        impl Condition for Exhaustion {}

        impl Identity for Exhaustion {
            fn id(&self) -> &'static str {
                "5E::CONDITION::EXHAUSTION"
            }
        }
        let s = Exhaustion(1);
    }
}