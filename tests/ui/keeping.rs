use designal::Designal;

#[derive(Designal)]
#[designal(trim_start = "Human")]
struct HumanBean {
    #[forbid(dead_code)]
    taste: String,
}

#[derive(Designal)]
#[designal(trim_start = "Human")]
#[forbid(dead_code)]
struct HumanBean1 {
    crunch: String,
}

fn main() {
    let _ = Bean1 {
        crunch: String::new(),
    };
}
