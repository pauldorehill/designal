use designal::Designal;

#[derive(Designal)]
#[designal(trim_start = "Human")]
struct HumanBean {
    #[designal(cfg_feature = "client")]
    taste: String,
}

fn main() {}