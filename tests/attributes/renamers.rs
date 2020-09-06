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
struct HumanBean6(#[designal(rename = "flavour")] String);

#[derive(Designal)]
#[designal(rename = "HumanBean", add_prefix = "Snozz")]
struct HumanBean7();

#[derive(Designal)]
#[designal(rename = "HumanBean", add_prefix = "Snozz", add_postfix = "Snozz")]
struct HumanBean8();

#[derive(Designal)]
#[designal(rename = "HumanBean", add_prefix = "Snozz", add_postfix = "Snozz", remove_start = "Snozz")]
struct HumanBean9();

#[derive(Designal)]
#[designal(rename = "HumanBean", add_prefix = "Snozz", add_postfix = "Snozz", remove_start = "Snozz", remove_end = "Snozz")]
struct HumanBean10();

#[derive(Designal)]
#[designal(remove_start = "Snozz")]
struct HumanBean11();

#[derive(Designal)]
#[designal(remove_end = "Snozz")]
struct HumanBean12();

fn main() {}
