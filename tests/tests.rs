#![allow(dead_code)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]

use designal::Designal;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use std::{rc::Rc, sync::Arc};
use trybuild;
// TODO: test derive::derive etc

#[test]
fn test_should_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/attributes/*.rs");
    t.compile_fail("tests/types/*.rs");
}

fn basic() {
    #[derive(Designal)]
    struct HumanBean();

    let _ = HumanBeanDesig();
}

fn named() {
    #[derive(Designal)]
    struct HumanBean {
        taste: String,
    }

    let _ = HumanBeanDesig {
        taste: String::new(),
    };
}

fn unnamed_struct_fields() {
    #[derive(Designal)]
    struct HumanBean(String, i8);
    let _ = HumanBeanDesig(String::new(), 8);
}

fn rename_struct() {
    #[derive(Designal)]
    #[designal(rename = "NewBean")]
    struct HumanBean();
    let _ = NewBean();
}

fn add_prefix_struct() {
    #[derive(Designal)]
    #[designal(add_prefix = "Snozz")]
    struct HumanBean();
    let _ = SnozzHumanBean();
}

fn add_postfix_struct() {
    #[derive(Designal)]
    #[designal(add_postfix = "Snozz")]
    struct HumanBean();
    let _ = HumanBeanSnozz();
}

fn remove_start_struct() {
    #[derive(Designal)]
    #[designal(remove_start = "Human")]
    struct HumanBean();
    let _ = Bean();
}

fn remove_end_struct() {
    #[derive(Designal)]
    #[designal(remove_end = "Bean")]
    struct HumanBean();
    let _ = Human();
}

fn auto_rename_mutable_struct_prefix() {
    #[derive(Designal)]
    struct MutableHumanBean();
    let _ = HumanBean();
}

fn auto_rename_mutable_struct_postfix() {
    #[derive(Designal)]
    struct HumanBeanMutable();
    let _ = HumanBean();
}

fn auto_rename_signal_struct_prefix() {
    #[derive(Designal)]
    struct SignalHumanBean();
    let _ = HumanBean();
}

fn auto_rename_signal_struct_postfix() {
    #[derive(Designal)]
    struct HumanBeanSignal();
    let _ = HumanBean();
}

fn auto_rename_struct_called_mutable() {
    #[derive(Designal)]
    struct Mutable();
    let _ = MutableDesignal();
}

fn auto_rename_struct_called_signal() {
    #[derive(Designal)]
    struct Signal();
    let _ = SignalDesignal();
}

fn rename_struct_named_field() {
    #[derive(Designal)]
    struct HumanBean {
        #[designal(rename = "flavour")]
        taste: String,
    }
    let _ = HumanBeanDesig {
        flavour: String::new(),
    };
}

fn remove_struct_field() {
    #[derive(Designal)]
    struct HumanBean {
        #[designal(remove)]
        taste: Mutable<String>,
    }
    let _ = HumanBeanDesig {};
}

fn ignore_struct_field() {
    #[derive(Designal)]
    struct HumanBean {
        #[designal(ignore)]
        taste: Mutable<String>,
    }
    let _ = HumanBeanDesig {
        taste: Mutable::new(String::new()),
    };
}

fn ignore_struct_unnamed_field() {
    #[derive(Designal)]
    struct HumanBean(#[designal(ignore)] Mutable<String>);
    let _ = HumanBeanDesig(Mutable::new(String::new()));
}

fn remove_mutable() {
    #[derive(Designal)]
    struct HumanBean {
        taste: Mutable<String>,
    }
    let _ = HumanBeanDesig {
        taste: String::new(),
    };
}
fn remove_full_path_mutable() {
    #[derive(Designal)]
    struct HumanBean {
        taste: futures_signals::signal::Mutable<String>,
    }
    let _ = HumanBeanDesig {
        taste: String::new(),
    };
}

fn remove_rc() {
    #[derive(Designal)]
    struct HumanBean {
        taste: Rc<String>,
    }
    let _ = HumanBeanDesig {
        taste: String::new(),
    };
}

fn keep_rc_field() {
    #[derive(Designal)]
    struct HumanBean {
        #[designal(keep_rc)]
        taste: Rc<String>,
    }
    let _ = HumanBeanDesig {
        taste: Rc::new(String::new()),
    };
}

fn keep_rc_struct() {
    #[derive(Designal)]
    #[designal(keep_rc)]
    struct HumanBean {
        taste: Rc<String>,
        crunch: Rc<String>,
    }
    let _ = HumanBeanDesig {
        taste: Rc::new(String::new()),
        crunch: Rc::new(String::new()),
    };
}

fn remove_mutable_rc() {
    #[derive(Designal)]
    struct HumanBean {
        taste: Mutable<Rc<String>>,
    }
    let _ = HumanBeanDesig {
        taste: String::new(),
    };
}

fn remove_rc_mutable_rc() {
    #[derive(Designal)]
    struct HumanBean {
        taste: Rc<Mutable<Rc<String>>>,
    }
    let _ = HumanBeanDesig {
        taste: String::new(),
    };
}

fn remove_mutable_mutable() {
    #[derive(Designal)]
    struct HumanBean {
        taste: Mutable<Mutable<String>>,
    }
    let _ = HumanBeanDesig {
        taste: String::new(),
    };
}

fn remove_rc_rc() {
    #[derive(Designal)]
    struct HumanBean {
        taste: Rc<Rc<String>>,
    }
    let _ = HumanBeanDesig {
        taste: String::new(),
    };
}

fn remove_arc() {
    #[derive(Designal)]
    struct HumanBean {
        taste: Arc<String>,
    }
    let _ = HumanBeanDesig {
        taste: String::new(),
    };
}

fn keep_arc_struct() {
    #[derive(Designal)]
    #[designal(keep_arc)]
    struct HumanBean {
        taste: Arc<String>,
        crunch: Arc<String>,
    }
    let _ = HumanBeanDesig {
        taste: Arc::new(String::new()),
        crunch: Arc::new(String::new()),
    };
}

fn keep_arc_field() {
    #[derive(Designal)]
    struct HumanBean {
        #[designal(keep_arc)]
        taste: Arc<String>,
    }
    let _ = HumanBeanDesig {
        taste: Arc::new(String::new()),
    };
}

fn remove_mutable_arc() {
    #[derive(Designal)]
    struct HumanBean {
        taste: Mutable<Arc<String>>,
    }
    let _ = HumanBeanDesig {
        taste: String::new(),
    };
}

fn remove_arc_mutable() {
    #[derive(Designal)]
    struct HumanBean {
        taste: Arc<Mutable<String>>,
    }
    let _ = HumanBeanDesig {
        taste: String::new(),
    };
}

fn remove_mutable_vec() {
    #[derive(Designal)]
    struct HumanBean {
        taste: MutableVec<String>,
    }
    let _ = HumanBeanDesig { taste: Vec::new() };
}

fn remove_mutable_vec_rc() {
    #[derive(Designal)]
    struct HumanBean {
        taste: MutableVec<Rc<String>>,
    }
    let _ = HumanBeanDesig {
        taste: vec![String::new()],
    };
}

fn remove_mutable_rc_vec() {
    #[derive(Designal)]
    struct HumanBean {
        taste: Rc<MutableVec<String>>,
    }
    let _ = HumanBeanDesig { taste: Vec::new() };
}

fn remove_mutable_vec_arc() {
    #[derive(Designal)]
    struct HumanBean {
        taste: MutableVec<Arc<String>>,
    }
    let _ = HumanBeanDesig { taste: Vec::new() };
}

fn remove_mutable_arc_vec() {
    #[derive(Designal)]
    struct HumanBean {
        taste: Arc<MutableVec<String>>,
    }
    let _ = HumanBeanDesig { taste: Vec::new() };
}

fn remove_mutable_vec_full_path() {
    #[derive(Designal)]
    struct HumanBean {
        taste: futures_signals::signal_vec::MutableVec<String>,
    }
    let _ = HumanBeanDesig { taste: Vec::new() };
}

mod upper {
    use inner::{HumanBean, HumanBean2};
    mod inner {
        use designal::Designal;
        #[derive(Designal)]
        pub struct HumanBeanMutable {
            pub taste: u8,
        }
        #[derive(Designal)]
        pub struct HumanBean2Mutable(pub u8);
    }

    fn check_vis() {
        let _ = HumanBean { taste: 0 };
        let _ = HumanBean2(0);
    }
}

fn derive_single() {
    #[derive(Designal)]
    #[designal(derive = "Debug")]
    struct HumanBean();

    let r = HumanBeanDesig();

    println!("{:?}", r)
}

fn derive_vec_attributes() {
    #[derive(Designal)]
    #[designal(derive = "Debug")]
    #[designal(derive = "Eq")]
    #[designal(derive = "PartialEq")]
    struct HumanBean();

    let r1 = HumanBeanDesig();
    let r2 = HumanBeanDesig();

    println!("{:?}", r2);
    let _ = r1 == r2;
}

fn derive_vec_attributes_inline() {
    #[derive(Designal)]
    #[designal(derive = "Debug", derive = "PartialEq")]
    pub struct HumanBeanMutable {
        taste: u8,
    }
}

fn derive_vec_attributes_csv() {
    #[derive(Designal)]
    #[designal(derive = "Debug, PartialEq")]
    pub struct HumanBeanMutable {
        taste: u8,
    }
}

fn generics() {
    #[derive(Designal)]
    pub struct HumanBeanMutable<'a, T>
    where
        T: Copy,
    {
        taste: &'a T,
    }

    let _ = HumanBean { taste: &5 };
}

fn generics_unnamed() {
    #[derive(Designal)]
    struct HumanBeanMutable<'a, T>(&'a T)
    where
        T: Copy;

    let _ = HumanBean(&5);
}
