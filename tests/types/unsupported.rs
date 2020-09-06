use designal::Designal;

#[derive(Designal)]
enum HumanBean {}

#[derive(Designal)]
union HumanBean1 {
    taste: u8
}

#[derive(Designal)]
struct HumanBean2;

fn main() {}