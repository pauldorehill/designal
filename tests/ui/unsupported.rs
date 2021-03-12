use designal::Designal;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;

#[derive(Designal)]
#[designal(trim_start = "Human")]
union HumanBean1 {
    taste: u8,
}

#[derive(Designal)]
#[designal(trim_start = "Human")]
struct HumanBean3 {
    #[designal(hashmap)]
    taste: Mutable<String>,
}

#[derive(Designal)]
#[designal(trim_start = "Human")]
struct HumanBean4 {
    #[designal(hashmap)]
    taste: MutableVec<String>,
}

fn main() {}
