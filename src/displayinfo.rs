use yew::prelude::*;
use stylist::Style;
use std::rc::Rc;
use super::weakcomponentlink::*;

use crate::console::*;

pub trait ShowInfo {
    fn info() -> DisplayInfo;
}

pub enum Msg {
    SetDisplayInfo(DisplayInfo),
}

#[derive(PartialEq, Properties)]
pub struct DisplayInfoProps {
    pub weak_link: WeakComponentLink<DisplayInfo>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisplayInfo {
    pub name: String,
    pub attrs: Vec<String>
}

impl Component for DisplayInfo {
    type Message = Msg;
    type Properties = DisplayInfoProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props().weak_link.borrow_mut().replace(ctx.link().clone());

        let mut this = Self {
            name: "Click on a tile".to_string(),
            attrs: vec![]
        };
        this
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetDisplayInfo(info) => {
                self.name = info.name.clone();
                self.attrs = info.attrs.clone();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let css = Style::new("width: 20%; height: 100%; float: right; margin-left: 5%;").unwrap();
        html! {
            <div class={css}>
                <h1>{&self.name}</h1>
                <ul>
                    {for self.attrs.iter().map(|attr| html!{<li>{attr}</li>})}
                </ul>
            </div>
        }
    }
}
