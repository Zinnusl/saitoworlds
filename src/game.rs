use yew::prelude::*;

use crate::displayinfo::*;
use crate::world::*;
use crate::Closure;
use crate::console::*;
use std::rc::Rc;
use std::borrow::Borrow;
use super::weakcomponentlink::*;


use crate::canvas::*;

pub enum Msg {
    SetDisplayInfo(DisplayInfo),
}

pub struct Game {
    pub world: WorldPtr,
    pub handles: Vec<Closure<dyn FnMut(web_sys::MouseEvent)>>,
    pub displayinfo_l: WeakComponentLink<DisplayInfo>,
}

impl Game {
    pub fn add_handle(&mut self, handle: Closure<dyn FnMut(web_sys::MouseEvent)>) {
        self.handles.push(handle);
    }
}
impl Component for Game {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            world: World::new(),
            handles: Vec::<Closure<dyn FnMut(web_sys::MouseEvent)>>::new(),
            displayinfo_l: WeakComponentLink::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetDisplayInfo(info) => {
                let link: std::cell::RefMut<Option<yew::html::Scope<DisplayInfo>>> = (*self.displayinfo_l).borrow_mut();
                link.as_ref().unwrap().send_message(crate::displayinfo::Msg::SetDisplayInfo(info));
                false
            }
        }

    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ondisplayinfo = ctx.link().callback(Msg::SetDisplayInfo);
        html! {
            <div>
                <Canvas {ondisplayinfo} world={self.world.clone()} />
                <DisplayInfo weak_link={&self.displayinfo_l} />
            </div>
        }
    }
}
