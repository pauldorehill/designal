#![allow(dead_code)]

use designal::Designal;
use futures_signals::signal::Mutable;
use futures_signals::signal_map::MutableBTreeMap;
use futures_signals::signal_vec::MutableVec;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::{rc::Rc, sync::Arc};

#[test]
fn test_should_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}

fn rename_struct() {
    #[derive(Designal, Debug)]
    #[designal(rename = "Giant")]
    struct HumanBean();
    let _ = Giant();
    println!("{:?}", HumanBean());
}

fn unit_struct() {
    #[derive(Designal)]
    #[designal(rename = "Giant")]
    struct HumanBean;
    let _ = Giant;
}

fn rename_struct_named_fields() {
    #[derive(Designal)]
    #[designal(rename = "NewBean")]
    struct HumanBean {
        taste: String,
    }
    let _ = NewBean {
        taste: String::new(),
    };
}

fn struct_option_field() {
    #[derive(Designal)]
    #[designal(rename = "NewBean")]
    struct HumanBean {
        taste: Option<String>,
    }
    let _ = NewBean { taste: None };
}

fn struct_option_field_trim_end_all() {
    #[derive(Designal)]
    #[designal(trim_end_all = "Signal")]
    struct HumanBeanSignal {
        id: Option<i32>,
    }
    let _ = HumanBean { id: None };
}

fn add_start_struct() {
    #[derive(Designal)]
    #[designal(add_start = "Snozz")]
    struct HumanBean();
    let _ = SnozzHumanBean();
}

fn add_end_struct() {
    #[derive(Designal)]
    #[designal(add_end = "Snozz")]
    struct HumanBean();
    let _ = HumanBeanSnozz();
}

fn trim_start_struct() {
    #[derive(Designal)]
    #[designal(trim_start = "Human")]
    struct HumanBean();
    let _ = Bean();
}

fn trim_end_struct() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean();
    let _ = Human();
}

fn trim_start_struct_all() {
    struct HumanTaste;
    struct Taste;
    struct HumanCrunch;
    struct Crunch;

    #[derive(Designal)]
    #[designal(trim_start_all = "Human")]
    struct HumanBean {
        taste: HumanTaste,
        crunch: HumanCrunch,
    }
    let _ = Bean {
        taste: Taste,
        crunch: Crunch,
    };
}

fn trim_end_struct_all() {
    struct TasteHuman;
    struct Taste;
    struct CrunchHuman;
    struct Crunch;

    #[derive(Designal)]
    #[designal(trim_end_all = "Human")]
    struct BeanHuman {
        taste: TasteHuman,
        crunch: CrunchHuman,
    }
    let _ = Bean {
        taste: Taste,
        crunch: Crunch,
    };
}

fn trim_end_struct_all_rename_field() {
    struct TasteHuman;
    struct Taste;
    struct CrunchBean;
    struct Crunch;

    #[derive(Designal)]
    #[designal(trim_end_all = "Human")]
    struct BeanHuman {
        taste: TasteHuman,
        #[designal(trim_end = "Bean")]
        crunch: CrunchBean,
    }
    let _ = Bean {
        taste: Taste,
        crunch: Crunch,
    };
}

fn trim_struct_named_field() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        #[designal(remove)]
        taste: Mutable<String>,
    }
    let _ = Human {};
}

fn trim_struct_unnamed_field() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean(#[designal(remove)] Mutable<String>);
    let _ = Human {};
}

fn ignore_struct_field() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        #[designal(ignore)]
        taste: Mutable<String>,
    }
    let _ = Human {
        taste: Mutable::new(String::new()),
    };
}

fn ignore_struct_unnamed_field() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean(#[designal(ignore)] Mutable<String>);
    let _ = HumanBean(Mutable::new(String::new()));
}

fn trim_mutable() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: Mutable<String>,
    }
    let _ = Human {
        taste: String::new(),
    };
}

fn trim_full_path_mutable() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: futures_signals::signal::Mutable<String>,
    }
    let _ = Human {
        taste: String::new(),
    };
}

fn trim_rc() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: Rc<String>,
    }
    let _ = Human {
        taste: String::new(),
    };
}

fn keep_rc_field() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        #[designal(keep_rc)]
        taste: Rc<String>,
    }
    let _ = Human {
        taste: Rc::new(String::new()),
    };
}

fn keep_rc_struct() {
    #[derive(Designal)]
    #[designal(keep_rc, trim_end = "Bean")]
    struct HumanBean {
        taste: Rc<String>,
        crunch: Rc<String>,
    }
    let _ = Human {
        taste: Rc::new(String::new()),
        crunch: Rc::new(String::new()),
    };
}

fn trim_mutable_rc() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: Mutable<Rc<String>>,
    }
    let _ = Human {
        taste: String::new(),
    };
}

fn trim_rc_mutable_rc() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: Rc<Mutable<Rc<String>>>,
    }
    let _ = Human {
        taste: String::new(),
    };
}

fn trim_mutable_mutable() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: Mutable<Mutable<String>>,
    }
    let _ = Human {
        taste: String::new(),
    };
}

fn trim_rc_rc() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        #[allow(clippy::clippy::redundant_allocation)]
        taste: Rc<Rc<String>>,
    }
    let _ = Human {
        taste: String::new(),
    };
}

fn trim_arc() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: Arc<String>,
    }
    let _ = Human {
        taste: String::new(),
    };
}

fn keep_arc_struct() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    #[designal(keep_arc)]
    struct HumanBean {
        taste: Arc<String>,
        crunch: Arc<String>,
    }
    let _ = Human {
        taste: Arc::new(String::new()),
        crunch: Arc::new(String::new()),
    };
}

fn keep_arc_field() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        #[designal(keep_arc)]
        taste: Arc<String>,
    }
    let _ = Human {
        taste: Arc::new(String::new()),
    };
}

fn trim_mutable_arc() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: Mutable<Arc<String>>,
    }
    let _ = Human {
        taste: String::new(),
    };
}

fn trim_arc_mutable() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: Arc<Mutable<String>>,
    }
    let _ = Human {
        taste: String::new(),
    };
}

fn trim_mutable_vec() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: MutableVec<String>,
    }
    let _ = Human { taste: Vec::new() };
}

fn trim_mutable_vec_rc() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: MutableVec<Rc<String>>,
    }
    let _ = Human {
        taste: vec![String::new()],
    };
}

fn trim_mutable_rc_vec() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: Rc<MutableVec<String>>,
    }
    let _ = Human { taste: Vec::new() };
}

fn trim_mutable_vec_arc() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: MutableVec<Arc<String>>,
    }
    let _ = Human { taste: Vec::new() };
}

fn trim_mutable_arc_vec() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: Arc<MutableVec<String>>,
    }
    let _ = Human { taste: Vec::new() };
}

fn trim_mutable_vec_full_path() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: futures_signals::signal_vec::MutableVec<String>,
    }
    let _ = Human { taste: Vec::new() };
}

fn trim_mutable_btreemap() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: MutableBTreeMap<String, u32>,
    }
    let taste: BTreeMap<String, u32> = BTreeMap::new();
    let _ = Human { taste };
}

fn trim_mutable_btreemap_to_btreeset() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: MutableBTreeMap<String, ()>,
    }
    let taste: BTreeSet<String> = BTreeSet::new();
    let _ = Human { taste };
}

fn trim_mutable_btreemap_to_hashset() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean", hashmap)]
    struct HumanBean {
        taste: MutableBTreeMap<String, ()>,
        crunch: MutableBTreeMap<u32, ()>,
    }
    let taste: HashSet<String> = HashSet::new();
    let crunch: HashSet<u32> = HashSet::new();
    let _ = Human { taste, crunch };
}

fn trim_mutable_btreemap_to_hashmap_struct() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean", hashmap)]
    struct HumanBean {
        taste: MutableBTreeMap<String, u32>,
        crunch: MutableBTreeMap<u32, String>,
    }
    let taste: HashMap<String, u32> = HashMap::new();
    let crunch: HashMap<u32, String> = HashMap::new();
    let _ = Human { taste, crunch };
}

fn trim_mutable_btreemap_to_hashmap_field() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        #[designal(hashmap)]
        taste: MutableBTreeMap<String, u32>,
    }
    let taste: HashMap<String, u32> = HashMap::new();
    let _ = Human { taste };
}

fn trim_mutable_btreemap_full_path() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: futures_signals::signal_map::MutableBTreeMap<String, u32>,
    }
    let taste: BTreeMap<String, u32> = BTreeMap::new();
    let _ = Human { taste };
}

fn trim_mutable_btreemap_rc() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: MutableBTreeMap<Rc<String>, Rc<u32>>,
    }
    let taste: BTreeMap<String, u32> = BTreeMap::new();
    let _ = Human { taste };
}

fn trim_mutable_rc_btreemap() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: Rc<MutableBTreeMap<String, u32>>,
    }
    let taste: BTreeMap<String, u32> = BTreeMap::new();
    let _ = Human { taste };
}

fn trim_mutable_btreemap_arc() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: MutableBTreeMap<Arc<String>, Arc<u32>>,
    }
    let taste: BTreeMap<String, u32> = BTreeMap::new();
    let _ = Human { taste };
}

fn trim_mutable_arc_btreemap() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean {
        taste: Arc<MutableBTreeMap<String, u32>>,
    }

    let taste: BTreeMap<String, u32> = BTreeMap::new();
    let _ = Human { taste };
}

mod upper {
    use inner::{Human, Human2};
    mod inner {
        use designal::Designal;
        use futures_signals::signal::Mutable;
        #[derive(Designal)]
        #[designal(trim_end = "Bean")]
        pub struct HumanBean {
            pub taste: Mutable<u8>,
        }
        #[derive(Designal)]
        #[designal(trim_end = "Bean")]
        pub struct Human2Bean(pub Mutable<u8>);
    }

    fn check_vis() {
        let _ = Human { taste: 0 };
        let _ = Human2(0);
    }
}

fn derive_single() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    #[designal(attribute = #[derive(Debug)])]
    struct HumanBean();

    let r = Human();

    println!("{:?}", r)
}

fn derive_vec_attributes() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    #[designal(attribute = #[derive(Debug)])]
    #[designal(attribute = #[derive(Eq)])]
    #[designal(attribute = #[derive(PartialEq)])]
    struct HumanBean();

    let r1 = Human();
    let r2 = Human();

    println!("{:?}", r2);
    let _ = r1 == r2;
}

fn derive_vec_attributes_inline() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    #[designal(attribute =
        #[derive(Debug)]
        #[derive(Eq)]
        #[derive(PartialEq)]
    )]
    pub struct HumanBean {
        taste: u8,
    }

    let r1 = Human { taste: 1 };
    let r2 = Human { taste: 2 };

    println!("{:?}", r2);
    let _ = r1 == r2;
}

fn derive_vec_attributes_csv() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    #[designal(attribute =
        #[derive(Debug)],
        #[derive(Eq)],
        #[derive(PartialEq)],
    )]
    pub struct HumanBean {
        taste: u8,
    }

    let r1 = Human { taste: 1 };
    let r2 = Human { taste: 2 };

    println!("{:?}", r2);
    let _ = r1 == r2;
}

fn derive_full_path() {
    // use find_me::FindMe;
    #[derive(designal::Designal)]
    #[designal(trim_end = "Bean")]
    #[designal(attribute = #[derive(std::default::Default)])]
    pub struct HumanBean {
        taste: u8,
    }

    let _ = Human::default();
}

fn generics() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    pub struct HumanBean<'a, T>
    where
        T: Copy,
    {
        taste: &'a T,
    }

    let _ = Human { taste: &5 };
}

fn generics_unnamed() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    struct HumanBean<'a, T>(&'a T)
    where
        T: Copy;

    let _ = Human(&5);
}

fn nested_types() {
    #[derive(Designal)]
    #[designal(trim_end = "Signal")]
    #[designal(attribute = #[derive(Debug)])]
    struct FlavoursSignal(MutableVec<String>);

    // TODO: Code to parse both types?
    #[derive(Designal)]
    #[designal(trim_end = "Signal")]
    #[designal(attribute = #[derive(Debug)])]
    struct TasteSignal {
        salt: Mutable<u32>,
        sweet: Mutable<bool>,
        sour: Mutable<Rc<i8>>,
        #[designal(trim_end = "Signal")]
        flavours: FlavoursSignal,
    }

    #[derive(Designal)]
    #[designal(trim_end = "Signal")]
    #[designal(attribute = #[derive(Debug)])]
    struct HumanSignal {
        #[designal(trim_end = "Signal")]
        taste: Rc<TasteSignal>,
        name: Mutable<(String, String)>,
        #[designal(remove)]
        editing: Mutable<bool>,
    }

    #[cfg(feature = "client")]
    let _ = Human {
        taste: Taste {
            salt: 0,
            sweet: true,
            sour: 5,
            flavours: Flavours(vec!["strawberry".to_string()]),
        },
        name: ("Sophie".to_string(), "Hopscotchy".to_string()),
    };
}

fn multiple_attributes() {
    #[derive(Designal)]
    #[designal(trim_end = "Bean")]
    #[designal(attribute =
        #[derive(Debug)],
        #[derive(Clone)],
        // #[cfg(feature = "client")]
    )]
    #[designal(attribute =
        #[derive(Default)],
    )]
    struct HumanBean {
        #[designal(attribute =
            #[cfg(feature = "client")]
        )]
        taste: String,
    };

    #[cfg(feature = "client")]
    let _: Human = Human {
        taste: String::new(),
    };
}

// TODO: Test the replacing
designal::start_write_to_file!();
#[test]
fn replace_atts() {
    #[derive(Designal, Deserialize, Serialize)]
    #[designal(trim_end = "Bean")]
    #[designal(attribute_replace =
        #[derive(Debug)],
        #[derive(Deserialize, Serialize)],
        #[serde(rename = "RenamedBean")]
    )]
    #[serde(rename = "RenamedBean")]
    struct HumanBean {
        #[designal(attribute_replace = #[serde(rename = "designal_new_name")])]
        #[serde(rename = "new_name")]
        taste: String,
    };

    let json = serde_json::json! {
        {
            "designal_new_name": "hello"
        }
    };
    assert!(serde_json::from_value::<Human>(json).is_ok());
}

// fn attributes_with_others() {
//     #[derive(Designal)]
//     #[designal(attribute = #[derive(Debug)], trim_end = "Bean")]
//     struct HumanBean {
//         taste: String,
//     };

//     let _: Human = Human {
//         taste: String::new(),
//     };
// }
designal::stop_write_to_file!();
fn basic_enum_num_testing() {
    #[derive(Designal)]
    #[designal(trim_end = "Signal")]
    enum GiantSignal {
        // These are 'syn::Fields::Unit`
        BoneCruncher,
        FleshLumpEater,
    }

    let _: Giant = Giant::BoneCruncher;
    let _: Giant = Giant::FleshLumpEater;
}

fn empty_enum() {
    #[derive(Designal)]
    #[designal(trim_start = "Human")]
    enum HumanBean {}
    let _: Bean = unreachable!();
}

// #[test]
// fn discrim_enum_num_testing() {
//     #[derive(Designal)]
//     #[designal(trim_end = "Signal")]
//     enum GiantSignal {
//         BoneCruncherSignal = 1,
//         FleshLumpEaterSignal = 2,
//     }

//     assert!(Giant::BoneCruncher as usize == 1);
//     // assert!(Giant::FleshLumpEater as usize == 2);
// }

fn basic_enum_with_struct_field() {
    #[derive(Designal)]
    #[designal(trim_end = "Signal")]
    struct MealSignal();

    #[derive(Designal)]
    #[designal(trim_end_all = "Signal")]
    enum GiantSignal {
        // This is a `syn::Fields::Named`
        GizzardGulper { name: i32 },
        TheButcherBoy { name: Mutable<String> },
        // This is a `syn::Fields::UnNamed`
        BoneCruncher(MealSignal),
        MeatDripper(MealSignal, String),
        FleshLumpEater(MealSignal, MealSignal),
    }

    let _: Giant = Giant::GizzardGulper { name: 3 };
    let _: Giant = Giant::TheButcherBoy {
        name: String::new(),
    };
    let _: Giant = Giant::BoneCruncher(Meal());
    let _: Giant = Giant::MeatDripper(Meal(), String::new());
    let _: Giant = Giant::FleshLumpEater(Meal(), Meal());
}
