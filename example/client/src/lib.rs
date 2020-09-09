use dominator::{html, Dom};
use shared::HumanBeanSignal;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

struct App {
    human: Rc<HumanBeanSignal>,
}

impl App {
    fn new() -> Rc<Self> {
        let app = App {
            human: HumanBeanSignal::new("Chidler"),
        };
        Rc::new(app)
    }

    fn render(app: Rc<Self>) -> Dom {
        html! {"div", {
            .children(&mut [
                HumanBeanSignal::render(app.human.clone())
            ])
        }}
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let app = App::new();
    dominator::append_dom(&dominator::body(), App::render(app));
    Ok(())
}
