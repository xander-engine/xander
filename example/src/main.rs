use xander_macros::namespace;

use obj::Object;

namespace!(apples);

mod obj;

fn main() {
    let s = namespace!();
    println!("Currently in namespace, {}", namespace!());
}
