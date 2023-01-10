use std::slice::Iter;

pub mod identity;
pub mod choice;
pub mod registry;

pub trait Proxy<T, Context> {
    fn affectors(&self) -> Iter<&dyn Fn(&Context, T) -> T>;
    fn initial(&self) -> T;
    fn calculate(&self, ctx : Context) -> T {
        self.affectors()
            .fold(self.initial(), |value, affector| affector(&ctx, value))
    }
}

#[cfg(test)]
mod tests {
    use std::slice::Iter;

    use crate::utils::Proxy;

    
    #[test]
    fn test_accumulation() {
        const FUNCS : [&dyn Fn(&Nonsense, i16) -> i16; 4] = [
            &|_, v| v + 1,
            &|_, v| v + 2, 
            &|_, v| v + 3, 
            &|_, v| v + 4 
        ];
        struct Nonsense;
        struct AbilityValue(i16);

        impl Proxy<i16, Nonsense> for AbilityValue {
            fn affectors(&self) -> Iter<&dyn Fn(&Nonsense, i16) -> i16> {
                FUNCS.iter()
            }

            fn initial(&self) -> i16 {
                self.0
            }  
        }

        let a = AbilityValue(0);
        println!("Value after +1 +2 +3 +4: {}", a.calculate(Nonsense));
        
    }
}