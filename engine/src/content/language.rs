use crate::utils::identity::Identity;

pub trait Language : Identity {
    fn description(&self) -> &'static str;
}

#[cfg(test)]
mod tests {
    use crate::utils::{identity::Identity, choice::{Choice, Choices}};

    use super::Language;

    struct Goblin;

    impl Language for Goblin {
        fn description(&self) -> &'static str {
            "Spoken by goblins."    
        }
    }

    impl Identity for Goblin {
        fn id(&self) -> &'static str {
            "5E::LANGUAGE::GOBLIN"    
        }
    }

    struct Occidural;
    impl Language for Occidural {
        fn description(&self) -> &'static str {
            "Homebrew language example."
        }
    }

    impl Identity for Occidural {
        fn id(&self) -> &'static str {
            "DEUS::LANGUAGE::OCCIDURAL"
        }
    }

    #[test]
    fn langs() {
        let choice : Choice<dyn Language, _> = Choices::of([
            &Goblin, &Occidural
        ]);

        choice
            .iter()
            .map(|f| format!("{} => {}", f.id(), f.description()))
            .for_each(|f| println!("{}", f));
    }
}