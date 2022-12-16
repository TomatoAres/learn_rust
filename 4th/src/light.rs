#![allow(dead_code)]
use std::time::Duration;

pub enum Light {
    RED(u64),
    YELLOW(u64),
    BLUE(u64),
}

pub trait Blocker {
    fn block_time(&self) -> Duration;
}


impl Blocker for Light {
    fn block_time(&self) -> Duration {
        match &self {
            Light::RED(t) => Duration::from_secs(t.to_owned()),
            Light::YELLOW(t) => Duration::from_secs(t.to_owned()),
            Light::BLUE(t) => Duration::from_secs(t.to_owned()),
        }
    }
}

fn main() {
    let red = Light::RED(3);
    println!("red blocker : {:?}", red.block_time());
}
