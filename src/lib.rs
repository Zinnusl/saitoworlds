#![cfg_attr(not(doc), no_main)]
#![feature(stmt_expr_attributes)]
#![feature(try_trait_v2)]
#![allow(dead_code)]
#[allow(unused_imports)]

use nannou::prelude::*;
use nannou::{
    app::{self, App},
    wgpu::{Backends, DeviceDescriptor, Limits},
};
use std::sync::RwLock;
use wasm_bindgen::prelude::*;
use std::rc::Rc;

mod console;
mod task;
mod saito;

extern crate console_error_panic_hook;

// #[wasm_bindgen(module = "/mods/saitoworlds/saitoworlds.js")]
// #[wasm_bindgen]
// extern "C" {
//     type SaitoworldsGame;

//     #[wasm_bindgen(method, catch)]
//     fn testerino(this: &SaitoworldsGame) -> Result<i32, JsValue>;
// }

#[wasm_bindgen]
pub async fn greet() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let window = web_sys::window().unwrap();
    let saito_module = window
        .get("saitoworlds_module")
        .unwrap();
    console::console_log!("saitomod: {:?}", saito_module);

    let mut saito = Box::new(saito::SaitoFacade::new(saito_module));
    saito.register_callbacks();

    thread_local!(static MODEL: RwLock<Option<Model>> = Default::default());
    let model = model(saito);

    MODEL.with(|m| m.write().unwrap().replace(model));

    task::block_on(async { app::Builder::new_async(|app| {
            Box::new(async {
                create_window(app).await;
                MODEL.with(|m| m.write().unwrap().take().unwrap())
            })
        })
        .backends(Backends::PRIMARY | Backends::GL)
        .update(update)
        .run_async()
        .await;
    });

    Ok(())
}

async fn create_window(app: &App) {
    let device_desc = DeviceDescriptor {
        limits: Limits {
            max_texture_dimension_2d: 8192,
            ..Limits::downlevel_webgl2_defaults()
        },
        ..Default::default()
    };

    app.new_window()             
        .size(1024, 1024)
        .device_descriptor(device_desc)
        .title("Saitoworlds")
        .view(view)
        // .mouse_pressed(mouse_pressed)
        // .mouse_released(mouse_released)
        .event(event)
        .build_async()
        .await
        .unwrap();
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MouseMoved(pos) => {
            let pos = Pos::new(pos);
            if model.pressed {
                console::console_log!("MouseMoved: {:?} {:?}", pos, model.saito.testerino(serde_yaml::to_string(&pos).unwrap().as_str()).unwrap());
            }
            model.last_pos = Some(pos);
        }

        MousePressed(button) => {
            if button == MouseButton::Left {
                model.pressed = true;
                match &model.last_pos {
                    Some(pos) => {
                        model.saito.testerino(serde_yaml::to_string(&pos).unwrap().as_str()).unwrap();
                    }
                    None => {}
                }
            }
        }

        MouseReleased(button) => {
            if button == MouseButton::Left {
                model.pressed = false;
            }
        }

        _other => (),
    }
}

struct Model {
    pixels: Rc<RwLock<Vec<Pos>>>,
    saito: Box<saito::SaitoFacade>,
    pressed: bool,
    last_pos: Option<Pos>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct Pos {
    x: f32,
    y: f32,
}

impl Pos {
    fn new(pos: Vec2) -> Self {
        Self {
            x: pos.to_array()[0],
            y: pos.to_array()[1],
        }
    }
}


fn model(mut saito: Box<saito::SaitoFacade>) -> Model {
    let pixels = Rc::new(RwLock::new(Vec::new()));

    let pixels_clone = pixels.clone();
    saito.block_on_confirmation_mut(Box::new(move |msg| {
        // console::console_log!("msg: {:?}", msg);
        let tup = serde_yaml::from_str(msg).unwrap();
        pixels_clone.write().unwrap().push(tup);
    }));
    let pixels_clone = pixels.clone();
    saito.block_on_load_all_mut(Box::new(move |txs| {
        console::console_log!("block_on_load_all_mut: {:?}", txs.len());
        let mut borrowed_pixels = pixels_clone.write().unwrap();
        txs.iter().for_each(|tx| {
            let tup = serde_yaml::from_str(tx).unwrap();
            borrowed_pixels.push(tup);
        });
    }));

    Model {
        pixels,
        saito,
        pressed: false,
        last_pos: None,
    }
}

fn update(_app: &App, _m: &mut Model, _update: Update) {
}

fn view(app: &App, m: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    // A simple way to draw the wave with an ellipse at each position
    m.pixels.read().unwrap().iter().for_each(|pos| {
        let x = pos.x;
        let y = pos.y;
        draw.ellipse()
            .x_y(x, y)
            .w_h(1.0, 1.0)
            .rgba(0.0, 0.0, 0.0, 1.0)
            .stroke(BLACK);
    });

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
