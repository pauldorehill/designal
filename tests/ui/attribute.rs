use designal::Designal;
use serde::{Deserialize, Serialize};

#[derive(Designal, Deserialize, Serialize)]
#[designal(trim_end = "Bean")]
#[designal(attribute =
    #[derive(Deserialize, Serialize)],
    #[serde(rename = "RenamedBean")]
)]
#[serde(rename = "RenamedBean")]
struct HumanBean {
    #[designal(attribute = #[serde(rename = "designal_new_name")])]
    #[serde(rename = "new_name")]
    taste: String,
}

fn main() {}