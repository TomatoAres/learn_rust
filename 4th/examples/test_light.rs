use homework2::light::{Blocker, Light};
fn main() {
    let red = Light::RED(3);
    println!("red blocker : {:?}", red.block_time());
}
