use designal::Designal;

#[derive(Designal)]
#[designal(trim_end = "Bean")]
pub struct HumanBean {
    #[designal(derive = "Debug")]
    taste: u8,
}

fn main() {}
