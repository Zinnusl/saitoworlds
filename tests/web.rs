//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

// #[path = "../src/lib.rs"] mod lib;

extern crate wasm_bindgen_test;
use saitoworlds::{World, Tile, TileImg, Clickable, Pos};
use wasm_bindgen_test::*;
use wasm_bindgen::{prelude::*, JsCast, Clamped};
use wasm_bindgen_futures::JsFuture;
use js_sys::Uint8Array;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen]
pub async fn fetch_url_binary(url: String) -> Result<Uint8Array, JsValue> {
    let window = web_sys::window().unwrap(); // Browser window
    let promise = JsFuture::from(window.fetch_with_str(&url)); // File fetch promise
    let result = promise.await?; // Await fulfillment of fetch
    let response: web_sys::Response = result.dyn_into().unwrap(); // Type casting
    let image_data = JsFuture::from(response.array_buffer()?).await?; // Get text
    Ok(Uint8Array::new(&image_data))
}


// #[wasm_bindgen_test]
// async fn print_texture() {
//     let s = lib::fetch_url_binary("../www/data/terrain/volcano.png".to_owned()).await;
//     console_log!("{:?}", s.unwrap().to_string());
// }


#[wasm_bindgen_test]
fn click_tile() {
    {
        let mut fake_world = World::new();
        let t = Tile::new_with_xy(TileImg::VOLCANO, 0, 0);
        assert_eq!(true, t.was_clicked(Some(&mut fake_world), 0.0, 0.0));
        assert_eq!(true, t.was_clicked(Some(&mut fake_world), 31.0, 31.0));
        assert_eq!(true, t.was_clicked(Some(&mut fake_world), -31.0, -31.0));

        assert_eq!(false, t.was_clicked(Some(&mut fake_world), -33.0, -33.0));
        assert_eq!(false, t.was_clicked(Some(&mut fake_world), 33.0, 33.0));
    }
    {
        let mut fake_world = World::new();
        let t = Tile::new_with_xy(TileImg::VOLCANO, 99, 99);
        let d = 99.0;
        assert_eq!(true, t.was_clicked(Some(&mut fake_world), 0.0 + d, 0.0 + d));
        assert_eq!(true, t.was_clicked(Some(&mut fake_world), 31.0 + d, 31.0 + d));
        assert_eq!(true, t.was_clicked(Some(&mut fake_world), -31.0 + d, -31.0 + d));

        assert_eq!(false, t.was_clicked(Some(&mut fake_world), -33.0 + d, -33.0 + d));
        assert_eq!(false, t.was_clicked(Some(&mut fake_world), 33.0 + d, 33.0 + d));
    }
    {
        let mut fake_world = World::new();
        fake_world.move_camera_to_mut(Pos::new(99, 99));
        let t = Tile::new_with_xy(TileImg::VOLCANO, 0, 0);
        let d = 99.0;
        assert_eq!(true, t.was_clicked(Some(&mut fake_world), 0.0 + d, 0.0 + d));
        assert_eq!(true, t.was_clicked(Some(&mut fake_world), 31.0 + d, 31.0 + d));
        assert_eq!(true, t.was_clicked(Some(&mut fake_world), -31.0 + d, -31.0 + d));

        assert_eq!(false, t.was_clicked(Some(&mut fake_world), -33.0 + d, -33.0 + d));
        assert_eq!(false, t.was_clicked(Some(&mut fake_world), 33.0 + d, 33.0 + d));
    }
}
