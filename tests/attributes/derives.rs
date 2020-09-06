use designal::Designal;

#[derive(Designal)]
pub struct HumanBeanMutable {
    #[designal(derive = "Debug")]
    taste: u8,
}

fn main() {}
