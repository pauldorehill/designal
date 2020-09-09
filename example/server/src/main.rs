#![allow(dead_code)]
use shared::HumanBean;

fn main() {
    let human = HumanBean::new("Sophie");
    if let Some(err) = human.name.validate() {
        eprintln!("Invalid: {}", err)
    }
}
