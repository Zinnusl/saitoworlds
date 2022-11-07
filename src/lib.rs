#![cfg_attr(not(doc), no_main)]
#![feature(stmt_expr_attributes)]
#![feature(try_trait_v2)]
#![allow(dead_code)]
#[allow(unused_imports)]


#[macro_use]
extern crate num_derive;

// use web_sys::ImageData;
// use wasm_bindgen::{prelude::*, JsCast, Clamped};
use wasm_bindgen::prelude::*;

mod info;
mod game;
mod displayinfo;
mod pos;
mod world;
mod renderable;
mod clickable;
mod console;
mod canvas;
mod weakcomponentlink;

#[wasm_bindgen(module = "/mods/saitoworlds/saitoworlds.js")]
extern "C" {
    type SaitoworldsGame;

    #[wasm_bindgen(method, catch)]
    fn testerino(this: &SaitoworldsGame) -> Result<i32, JsValue>;
}

#[wasm_bindgen]
pub async fn greet() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().unwrap();

    yew::start_app_in_element::<game::Game>(document.query_selector(".container").unwrap().unwrap());

    Ok(())
}
