use std::rc::Rc;
use futures_signals::signal::Mutable;

pub struct ContextMenu {
    pub show: Mutable<bool>,
    pub position: Mutable<(i32, i32)>,
}


impl ContextMenu {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            show: Mutable::new(false),
            position: Mutable::new((0, 0)),
        })
    }
}