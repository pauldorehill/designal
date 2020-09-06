use designal::Designal;

#[derive(Designal)]
#[designal(remove)]
struct HumanBean();

#[derive(Designal)]
struct HumanBean1{
    #[designal(remove)]
    #[designal(remove)]
    taste: String
}

fn main() {}