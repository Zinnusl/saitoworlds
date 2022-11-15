use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::sync::RwLock;
use crate::console;

pub struct SaitoFacade
{
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

impl SaitoFacade
{
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
                let txs = txs.dyn_into::<js_sys::Array>().unwrap().iter().map(|tx| {
                    tx.as_string().unwrap()
                }).collect::<Vec<String>>();
                for f in weak.upgrade().unwrap().read().unwrap().iter() {
                    f(&txs);
                }
            }) as Box<dyn Fn(JsValue)>);
            let closure_ref = closure.as_ref().unchecked_ref();
            set_js_property(&self.saitomod, "wasm_onLoadTransactionsCallback", closure_ref);
            closure.forget();
        }
    }

    pub fn new(saitomod: js_sys::Object) -> Self
    {
        Self { 
            saitomod,
            on_update: Rc::new(RwLock::new(vec![])),
            on_load: Rc::new(RwLock::new(vec![])),
        }
    }

    pub fn testerino(&self, msg: &str) -> Result<i32, Err>
    {
        let saitomod = &self.saitomod;
        let func = get_js_property(&saitomod, "testerino")?.dyn_into::<js_sys::Function>()?;
        let retval = func.call1(&saitomod, &JsValue::from(msg))?;
        Ok(retval.as_f64().unwrap() as i32)
    }

    // pub fn loadTransactions(&self, how_many: i32) -> Result<(), Err>
    // {
    //     let saitomod = &self.saitomod;
    //     let app = get_js_property(&saitomod, "app")?.dyn_into::<js_sys::Object>()?;
    //     let storage = get_js_property(&app, "storage")?.dyn_into::<js_sys::Object>()?;
    //     let func = get_js_property(&storage, "LoadTransactions")?.dyn_into::<js_sys::Function>()?;
    //     let retval = func.call2(&saitomod, &JsValue::from(how_many), callback)?;
    //     Ok(())
    // }
    
    pub fn block_on_confirmation_mut(&mut self, func: Box<dyn Fn(&String)>)
    {
        self.on_update.write().unwrap().push(func);
    }
    pub fn block_on_load_all_mut(&mut self, func: Box<dyn Fn(&Vec<String>)>)
    {
        self.on_load.write().unwrap().push(func);
        console::console_log!("block_on_load_all_mut");
    }
}

    // let app = js_sys::Reflect::get(&saitomod, &JsValue::from("app"))?;
    // let wallet = js_sys::Reflect::get(&app, &JsValue::from("wallet"))?;
    // let network = js_sys::Reflect::get(&app, &JsValue::from("network"))?;
    // let newtx_func = js_sys::Reflect::get(&wallet, &JsValue::from("createUnsignedTransactionWithDefaultFee"))?.dyn_into::<js_sys::Function>()?;
    // let newtx = newtx_func.call0(&wallet)?;

    // newtx.msg.module  = "Email";
    // newtx.msg.title   = "Congratulations - testerino button clicked!";
    // newtx.msg.message = "Your computer attached this email to a transaction and broadcast it. Your message is now on the blockchain.";
    // newtx = this.app.wallet.signTransaction(newtx);
    // this.app.network.propagateTransaction(newtx);

    // console::console_log!("newtx: {:?}", newtx);
    // js_sys::Object::entries(&wallet.into()).iter().for_each(|entry| {
    //     console::console_log!("entry: {:?}", entry);
    // });
    // js_sys::Object::entries(&network.into()).iter().for_each(|entry| {
    //     console::console_log!("entry: {:?}", entry);
    // });
