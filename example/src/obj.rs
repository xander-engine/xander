use xander_macros::namespace;

const N : &'static str = namespace!();

pub struct Object;

impl Object {
    pub fn say_hi(&self) {
        println!("Namespace From Say-hi: {}", namespace!());
    }
}

#[cfg(test)]
mod tests {
    use super::Object;

    #[test]
    fn macro_say_hi() {
        let o = Object;
        o.say_hi();
    }
}