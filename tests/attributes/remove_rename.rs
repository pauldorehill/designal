use designal::Designal;

#[derive(Designal)]
struct HumanBean {
    #[designal(remove, rename = "Yum")]
    taste: String,
}

fn main() {}
