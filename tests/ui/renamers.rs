use designal::Designal;

#[derive(Designal)]
#[designal(rename)]
struct HumanBean();
#[derive(Designal)]
#[designal(rename = "")]
struct HumanBean1();

#[derive(Designal)]
#[designal(rename = 2)]
struct HumanBean2();

#[derive(Designal)]
#[designal(rename = HumanBean)]
struct HumanBean3();

#[derive(Designal)]
#[designal(rename = "HumanBean1")]
#[designal(rename = "HumanBean2")]
struct HumanBean4();

#[derive(Designal)]
#[designal(rename = "HumanBean1", rename = "HumanBean2")]
struct HumanBean5();

#[derive(Designal)]
#[designal(trim_start = "Human")]
struct HumanBean6(#[designal(rename = "flavour")] String);

#[derive(Designal)]
#[designal(rename = "HumanBean", add_start = "Snozz")]
struct HumanBean7();

#[derive(Designal)]
#[designal(rename = "HumanBean", add_start = "Snozz", add_end = "Snozz")]
struct HumanBean8();

#[derive(Designal)]
#[designal(
    rename = "HumanBean",
    add_start = "Snozz",
    add_end = "Snozz",
    trim_start = "Snozz"
)]
struct HumanBean9();

#[derive(Designal)]
#[designal(
    rename = "HumanBean",
    add_start = "Snozz",
    add_end = "Snozz",
    trim_start = "Snozz",
    trim_end = "Snozz"
)]
struct HumanBean10();

#[derive(Designal)]
#[designal(trim_start = "Snozz")]
struct HumanBean11();

#[derive(Designal)]
#[designal(trim_end = "Snozz")]
struct HumanBean12();

fn main() {}
