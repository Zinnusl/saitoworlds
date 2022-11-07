use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use yew::html::{Component, ImplicitClone, Scope};

pub struct WeakComponentLink<COMP: Component>(Rc<RefCell<Option<Scope<COMP>>>>);

impl<COMP: Component> Clone for WeakComponentLink<COMP> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
impl<COMP: Component> ImplicitClone for WeakComponentLink<COMP> {}

impl<COMP: Component> Default for WeakComponentLink<COMP> {
    fn default() -> Self {
        Self(Rc::default())
    }
}

impl<COMP: Component> Deref for WeakComponentLink<COMP> {
    type Target = Rc<RefCell<Option<Scope<COMP>>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<COMP: Component> PartialEq for WeakComponentLink<COMP> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}


// let link: std::cell::RefMut<Option<yew::html::Scope<DisplayInfo>>> = (*self.displayinfo_l).borrow_mut();
// link.as_ref().unwrap().send_message(crate::displayinfo::Msg::SetDisplayInfo(info));
// impl<COMP: Component> WeakComponentLink<COMP> {
//     pub fn get_mut(&self) -> Option<&mut Scope<COMP>> {
//         self.0.borrow_mut().as_mut()
//     }
// }

