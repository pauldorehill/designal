use designal::Designal;

#[derive(Designal)]
#[designal(trim_start = "Human")]
#[designal(ignore)]
struct HumanBean();

#[derive(Designal)]
#[designal(trim_start = "Human")]
struct HumanBean1 {
    #[designal(ignore)]
    #[designal(ignore)]
    taste: String,
}
#[derive(Designal)]
#[designal(trim_start = "Human")]
struct HumanBean2 {
    #[designal(ignore)]
    #[designal(remove)]
    taste: String,
}

#[derive(Designal)]
#[designal(trim_start = "Human")]
struct HumanBean3 {
    #[designal(ignore)]
    #[designal(rename = "crunch")]
    taste: String,
}

#[derive(Designal)]
#[designal(trim_start = "Human")]
struct HumanBean4 {
    #[designal(ignore)]
    #[designal(keep_rc)]
    taste: String,
}

#[derive(Designal)]
#[designal(trim_start = "Human")]
struct HumanBean5 {
    #[designal(ignore)]
    #[designal(keep_arc)]
    taste: String,
}

fn main() {}
