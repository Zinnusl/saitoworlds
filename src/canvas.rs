use crate::displayinfo::*;
use crate::world::*;
use crate::clickable::*;
use yew::prelude::*;
use crate::pos::Offset;
use crate::renderable::*;
use gloo::timers::callback::Interval;
use std::rc::Rc;
use crate::Closure;
use wasm_bindgen::JsCast;
use std::cell::Cell;

use stylist::Style;                   

use crate::console::*;

pub enum Msg {
    Render,
    MoveCamera(Offset),
    Click(Offset),
}

#[derive(PartialEq, Properties)]
pub struct CanvasProps {
    pub world: WorldPtr,
    pub ondisplayinfo: Callback<DisplayInfo>,
}

pub struct Canvas {
    canvas_node_ref: NodeRef,
    clock_handle: Option<Interval>,
}
impl Canvas {
    pub fn get_canvas(&self) -> web_sys::HtmlCanvasElement {
        self.canvas_node_ref.cast::<web_sys::HtmlCanvasElement>().unwrap()
    }
}
impl Clickable for Canvas {
    fn was_clicked(&self, _world: Option<&World>, _x: f64, _y: f64) -> bool {
        false
    }
    fn clicked_mut(&mut self, _game: Option<&World>, _info: &mut Option<DisplayInfo>, _x: f64, _y: f64) {
    }
}
impl Component for Canvas {
    type Message = Msg;
    type Properties = CanvasProps;

    fn create(ctx: &Context<Self>) -> Self {
        let clock_handle = {
            let link = ctx.link().clone();
            Interval::new(1, move || link.send_message(Msg::Render))
        };
        let retval = Self {
            canvas_node_ref: NodeRef::default(),
            clock_handle: Some(clock_handle),
        };
        retval
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let canvas = self.get_canvas();
        let context = canvas
            .get_context("2d").unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        (*ctx.props().world).borrow().render(&context, Offset::new(0, 0));

        match msg {
            Msg::MoveCamera(offset) => {
                (*ctx.props().world).borrow_mut().move_camera_offset_mut(offset);
            }
            Msg::Click(offset) => {
                match offset {
                    Offset::ScreenSpace((x, y)) => {
                        let mut info = None;
                        (*ctx.props().world).borrow_mut().clicked_mut(None, &mut info, x, y);
                        if let Some(info) = info {
                            console_log!("emitting {:?}", &info);
                            ctx.props().ondisplayinfo.emit(info);
                        }
                    }
                    _ => {}
                }
            }
            Msg::Render => {
                (*ctx.props().world).borrow_mut().render(&context, Offset::new(0, 0));
            }
        }
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        // let link = ctx.link();
        let css = Style::new("width: 75%; height: 100%; float: left; clear: both;").unwrap();
        html! {
            <canvas ref={self.canvas_node_ref.clone()} class={css}></canvas>
        }
    }

    // TODO: In die Tonne treten und durch eine bessere Lösung ersetzen
    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let canvas = self.get_canvas();
        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);

        console_log!("Canvas Size: {} {}", canvas.offset_width(), canvas.offset_height());

        let screen_offset = Offset::ScreenSpace((canvas.width() as f64 / 2.0, canvas.height() as f64 / 2.0));
        (*ctx.props().world).borrow_mut().move_camera_to_mut(screen_offset);

        // let link = Rc::new(RefCell::new(ctx.link().clone()));
        let link = ctx.link();
        let pressed = Rc::new(Cell::new(false));
        let moved_camera = Rc::new(Cell::new(false));
        {
            let pressed = pressed.clone();
            let moved_camera = moved_camera.clone();
            let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {

                pressed.set(true);
                moved_camera.set(false);

            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
            closure.forget();
            // self.add_handle(closure);
        }
        {
            let pressed = pressed.clone();
            let moved_camera = moved_camera.clone();
            let link = link.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                if pressed.get() {
                    link.send_message(Msg::MoveCamera(Offset::ScreenSpace((event.movement_x() as f64, event.movement_y() as f64))));
                    // game.borrow_mut().world.move_camera_offset_mut(Offset::ScreenSpace((event.movement_x() as f64, event.movement_y() as f64)));
                    // game.borrow().render(&context, Offset::new(0, 0));
                    moved_camera.set(true);
                }

            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).unwrap();
            closure.forget();
            // self.add_handle(closure);
        }
        {
            let pressed = pressed.clone();
            let moved_camera = moved_camera.clone();
            let link = link.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                if !moved_camera.get() {
                    link.send_message(Msg::Click(Offset::ScreenSpace((event.offset_x() as f64, event.offset_y() as f64))));
                    // game.borrow_mut().clicked_mut(None, &mut None, event.offset_x() as f64, event.offset_y() as f64);
                }

                link.send_message(Msg::MoveCamera(Offset::ScreenSpace((event.movement_x() as f64, event.movement_y() as f64))));
                // game.borrow_mut().world.move_camera_offset_mut(Offset::ScreenSpace((event.movement_x() as f64, event.movement_y() as f64)));
                // game.borrow().render(&context, Offset::new(0, 0));
                pressed.set(false);

            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref()).unwrap();
            closure.forget();
            // self.add_handle(closure);
        }
    }
}
