use designal::Designal;

#[derive(Designal)]
#[designal(ignore)]
struct HumanBean();

#[derive(Designal)]
struct HumanBean1 {
    #[designal(ignore)]
    #[designal(ignore)]
    taste: String,
}
#[derive(Designal)]
struct HumanBean2 {
    #[designal(ignore)]
    #[designal(remove)]
    taste: String,
}

#[derive(Designal)]
struct HumanBean3 {
    #[designal(ignore)]
    #[designal(rename = "crunch")]
    taste: String,
}

#[derive(Designal)]
struct HumanBean4 {
    #[designal(ignore)]
    #[designal(keep_rc)]
    taste: String,
}

#[derive(Designal)]
struct HumanBean5 {
    #[designal(ignore)]
    #[designal(keep_arc)]
    taste: String,
}

fn main() {}
