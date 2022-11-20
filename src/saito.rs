use std::rc::Rc;
use std::sync::RwLock;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// use crate::console;

pub struct SaitoFacade {
    saitomod: js_sys::Object,
    on_update: Rc<RwLock<Vec<Box<dyn Fn(&String)>>>>,
    on_load: Rc<RwLock<Vec<Box<dyn Fn(&Vec<String>)>>>>,
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

impl SaitoFacade {
    pub fn register_callbacks(&self) {
        {
            let weak = Rc::downgrade(&self.on_update);
            let closure = Closure::wrap(Box::new(move |str: JsValue| {
                let str = str.as_string().unwrap();
                for f in weak.upgrade().unwrap().read().unwrap().iter() {
                    f(&str);
                }
            }) as Box<dyn Fn(JsValue)>);
            let closure_ref = closure.as_ref().unchecked_ref();
            set_js_property(&self.saitomod, "wasm_onConfirmationCallback", closure_ref);
            closure.forget();
        }
        {
            let weak = Rc::downgrade(&self.on_load);
            let closure = Closure::wrap(Box::new(move |txs: JsValue| {
                let txs = txs
                    .dyn_into::<js_sys::Array>()
                    .unwrap()
                    .iter()
                    .map(|tx| tx.as_string())
                    .collect::<Vec<Option<String>>>();
                let txs = txs.into_iter().filter_map(|tx| tx).collect::<Vec<String>>();
                for f in weak.upgrade().unwrap().read().unwrap().iter() {
                    f(&txs);
                }
            }) as Box<dyn Fn(JsValue)>);
            let closure_ref = closure.as_ref().unchecked_ref();
            set_js_property(
                &self.saitomod,
                "wasm_onLoadTransactionsCallback",
                closure_ref,
            );
            closure.forget();
        }
    }

    pub fn new(saitomod: js_sys::Object) -> Self {
        Self {
            saitomod,
            on_update: Rc::new(RwLock::new(vec![])),
            on_load: Rc::new(RwLock::new(vec![])),
        }
    }

    pub fn testerino(&self, msg: &str) -> Result<i32, Err> {
        let saitomod = &self.saitomod;
        let func = get_js_property(&saitomod, "testerino")?.dyn_into::<js_sys::Function>()?;
        let retval = func.call1(&saitomod, &JsValue::from(msg))?;
        Ok(retval.as_f64().unwrap() as i32)
    }

    pub fn on_tx_confirmation_mut(&mut self, func: Box<dyn Fn(&String)>) {
        self.on_update.write().unwrap().push(func);
    }
    pub fn on_load_all_txs_mut(&mut self, func: Box<dyn Fn(&Vec<String>)>) {
        self.on_load.write().unwrap().push(func);
    }
}
