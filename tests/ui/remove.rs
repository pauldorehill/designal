use designal::Designal;

#[derive(Designal)]
#[designal(trim_start = "Human")]
#[designal(remove)]
struct HumanBean();

#[derive(Designal)]
#[designal(trim_start = "Human")]
struct HumanBean1 {
    #[designal(remove)]
    #[designal(remove)]
    taste: String,
}

#[derive(Designal)]
#[designal(trim_start = "Human")]
struct HumanBean2 {
    #[designal(remove, rename = "Yum")]
    taste: String,
}

fn main() {}
