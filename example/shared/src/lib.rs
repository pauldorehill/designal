#![allow(dead_code)]
use designal::Designal;
#[cfg(feature = "client")]
use dominator::{clone, events, html, Dom};
use futures_signals::signal::Mutable;
#[cfg(feature = "client")]
use futures_signals::signal::SignalExt;
use std::rc::Rc;

#[derive(Clone)]
pub struct Name(pub String);

impl Name {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }

    pub fn validate(&self) -> Option<String> {
        if &self.0 != "Chidler" {
            Some(format!("Name is not Chidler. You have entered: {}", &self.0))
        } else {
            None
        }
    }
}

#[derive(Designal)]
#[designal(trim_end = "Signal", cfg_feature = "server")]
pub struct HumanBeanSignal {
    pub name: Mutable<Name>,
}

impl HumanBeanSignal {
    pub fn new(name: &str) -> Rc<Self> {
        let human = Self {
            name: Mutable::new(Name::new(name)),
        };
        Rc::new(human)
    }

    #[cfg(feature = "client")]
    pub fn render(human: Rc<Self>) -> Dom {
        html! { "div", {
            .children(&mut [
                html! {"div", {
                    .text("Name: ")
                    .text_signal(human.name.signal_cloned().map(|name| name.0))
                }},
                html! { "div", {
                    .text_signal(human.name.signal_cloned().map(|name| Name::validate(&name).unwrap_or_default()))
                    .style("color", "red")
                }},
                html! {"input", {
                    .attribute("type", "text")
                    .event(clone! { human => move |i: events::Input| {
                        match i.value() {
                            Some(value) => human.name.set(Name::new(&value)),
                            None => {}
                        }
                    }})
                    .attribute("value", &human.name.get_cloned().0)
                }}
            ])
        }}
    }
}

#[cfg(feature = "server")]
impl HumanBean {
    pub fn new(name: &str) -> Self {
        Self {
            name: Name::new(name),
        }
    }
}
