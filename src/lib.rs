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
pub mod tests;

extern crate console_error_panic_hook;

const POINT_TO_PIXEL: (u32, u32) = (1, 4);

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

    let saito = Box::new(saito::SaitoFacade::new(saito_module));
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
        .size_pixels(POINT_TO_PIXEL.1*256, POINT_TO_PIXEL.1*256)
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
        WindowEvent::MouseExited => {
            model.last_pos = None;
        }
        MouseMoved(pos) => {
            let pos = Pixel::new(pos);
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
    pixels: Rc<RwLock<Vec<Pixel>>>,
    saito: Box<saito::SaitoFacade>,
    pressed: bool,
    last_pos: Option<Pixel>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Pixel {
    x: f32,
    y: f32,
    color: (u8, u8, u8),
}

impl Pixel {
    fn new(pos: Vec2) -> Self {
        Self {
            x: (pos.to_array()[0] / POINT_TO_PIXEL.1 as f32).round(),
            y: (pos.to_array()[1] / POINT_TO_PIXEL.1 as f32).round(),
            color: (0, 0, 0),
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
        let mut borrowed_pixels = pixels_clone.write().unwrap();
        txs.iter().for_each(|tx| {
            if let Ok(tup) = serde_yaml::from_str(tx) {
                borrowed_pixels.push(tup);
            }
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
    let draw = app.draw();
    draw.background().color(WHITE);

    m.pixels.read().unwrap().iter().for_each(|pixel| {
        // let x = pos.x + (POINT_TO_PIXEL.1 as f32)/2.0;
        // let y = pos.y + (POINT_TO_PIXEL.1 as f32)/2.0;
        let x = pixel.x.round() * POINT_TO_PIXEL.1 as f32;
        let y = pixel.y.round() * POINT_TO_PIXEL.1 as f32;
        draw.rect()
            .x_y(x, y)
            .w_h(POINT_TO_PIXEL.1 as f32, POINT_TO_PIXEL.1 as f32)
            .rgba(pixel.color.0 as f32 / 255.0, pixel.color.1 as f32 / 255.0, pixel.color.2 as f32 / 255.0, 1.0)
            .stroke(nannou::color::Rgb::new(pixel.color.0 as f32 / 255.0, pixel.color.1 as f32 / 255.0, pixel.color.2 as f32 / 255.0));
    });

    match &m.last_pos {
        Some(pos) => {
            let x = pos.x.round() * POINT_TO_PIXEL.1 as f32;
            let y = pos.y.round() * POINT_TO_PIXEL.1 as f32;
            draw.rect()
                .x_y(x, y)
                .w_h(POINT_TO_PIXEL.1 as f32, POINT_TO_PIXEL.1 as f32)
                .rgba(random::<f32>(), random::<f32>(), random::<f32>(), 1.0)
                .stroke(nannou::color::Rgb::new(random::<f32>(), random::<f32>(), random::<f32>()));
                // .stroke(random::<nannou::color::Rgb::new>());
        }
        None => {}
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
