use std::rc::Rc;
use std::sync::RwLock;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Event;
// use crate::console;
//

use crate::console;

pub struct HtmlFacade {
    color_cb: Rc<RwLock<Vec<Box<dyn Fn(&String)>>>>,
}

fn set_js_property(obj: &js_sys::Object, key: &str, value: &JsValue) {
    let key = JsValue::from_str(key);
    let value = JsValue::from(value);
    js_sys::Reflect::set(&obj, &key, &value).unwrap();
}
fn get_js_property(obj: &js_sys::Object, key: &str) -> Result<JsValue, JsValue> {
    let key = JsValue::from_str(key);
    js_sys::Reflect::get(&obj, &key)
}

#[derive(Debug, Clone)]
pub enum Err {
    JsError,
    OtherError,
}

impl From<JsValue> for Err {
    fn from(_: JsValue) -> Self {
        Err::JsError
    }
}

macro_rules! event {
    ($text:literal) => {
        Event::new($text).unwrap()
    }
}

impl HtmlFacade {
    pub fn new(palette: web_sys::Element) -> Self {
        let color_cb = Rc::new(RwLock::new(Vec::<Box<dyn Fn(&String)>>::new()));
        let color_cb_clone = color_cb.clone();
        let closure = Closure::wrap(Box::new(move |event: Event| {
            match event
                .dyn_into::<web_sys::MouseEvent>()
                .and_then(|e| e.target().ok_or(event!("no target")))
                .and_then(|target| target.dyn_into::<web_sys::HtmlElement>().map_err(|_| event!("not html element")))
                .and_then(|htmlelement| htmlelement.dataset().get("color").ok_or(event!("could not find data-color attribute")))
            {
                Ok(color) => {
                    color_cb_clone
                        .read()
                        .unwrap()
                        .iter()
                        .for_each(|cb| cb(&color));
                }
                Err(e) => console::console_log!("error with color: {:?}", e),
            }
        }) as Box<dyn FnMut(_)>);
        palette
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
        HtmlFacade { color_cb }
    }

    pub fn register_callbacks(&self) {}

    pub fn on_color_select_mut(&mut self, cb: Box<dyn Fn(&String)>) {
        self.color_cb.write().unwrap().push(cb);
    }
}
