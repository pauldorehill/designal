use designal::Designal;

#[derive(Designal)]
struct HumanBean {
    #[forbid(dead_code)]
    taste: String,
}

#[derive(Designal)]
#[forbid(dead_code)]
struct HumanBean1 {
    crunch: String,
}

fn main() {}
