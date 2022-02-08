use std::cell::Cell;
use std::rc::Rc;
use web_sys::ImageData;
use image::GenericImageView;
use js_sys::Uint8Array;
use wasm_bindgen::{prelude::*, JsCast, Clamped};
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
pub async fn fetch_url_binary(url: String) -> Result<Uint8Array, JsValue> {
    let window = web_sys::window().unwrap(); // Browser window
    let promise = JsFuture::from(window.fetch_with_str(&url)); // File fetch promise
    let result = promise.await?; // Await fulfillment of fetch
    let response: web_sys::Response = result.dyn_into().unwrap(); // Type casting
    let image_data = JsFuture::from(response.array_buffer()?).await?; // Get text
    Ok(Uint8Array::new(&image_data))
}


#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {

    let binary = fetch_url_binary("data/terrain/iceberg.png".to_owned()).await?;
    let altbuf = binary.to_vec();

    // Convert the png encoded bytes to an rgba pixel buffer (given the PNG is actually in
    // 8byte RGBA format).
    let image = image::load_from_memory_with_format(&altbuf, image::ImageFormat::Png).unwrap();
    let mut rgba_image = image.to_rgba8();

    // I suppose this is what you tried to do in your original loop
    // judging by the function name:
    for (_, _, pixel) in rgba_image.enumerate_pixels_mut() {
        if pixel[0] > 0 {
            *pixel = image::Rgba([0, pixel[1], pixel[2],
                pixel[3]]);
        }
    }


    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(640);
    canvas.set_height(480);
    canvas.style().set_property("border", "solid")?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;



    let clamped_buf: Clamped<&[u8]> = Clamped(rgba_image.as_raw());
    let image_data_temp = ImageData::new_with_u8_clamped_array_and_sh(clamped_buf, image.width(), image.height())?;
    context.put_image_data(&image_data_temp, 0.0, 0.0)?;

    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));
    let image_data_temp_rc = Rc::new(image_data_temp);
    {
        let context = context.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {

            context.put_image_data(&image_data_temp_rc, event.offset_x() as f64, event.offset_y() as f64);

        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
