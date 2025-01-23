use std::rc::Rc;

use dominator::DomBuilder;
use futures_signals::signal::{self, Mutable, MutableSignalRef};
use web_sys::HtmlElement;

use crate::workspace::activity_panel::Activity;

pub fn bar_container(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("row-span-1")
        .class("bg-lightgray")
        .class("h-[35px]")
}

pub fn bar(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("inline-flex")
        .class("h-full")
        .class("gap-0")
}

pub fn container(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("h-full")
        .class("w-[110px]")
}

// There should be a better way to handle the last class_signal and its related variables
pub fn body(dom: DomBuilder<HtmlElement>,
    is_active: MutableSignalRef<Option<Rc<Activity>>, impl FnMut(&Option<Rc<Activity>>) -> bool + 'static>,
    mouse_over: &Mutable<bool>
) -> DomBuilder<HtmlElement> {
    dom.class("block")
        .class("h-full")
        .class("gap-1")
        .class("pt-1.5")
        .class("cursor-pointer")
        .class_signal("bg-white",signal::or(
            is_active,
            mouse_over.signal())
        )
}

pub fn content(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("icon_text")
        .class("inline-flex")
}

pub fn icon_default(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("icon")
}

pub fn icon(
    dom: DomBuilder<HtmlElement>,
    mouse_over_close: &Mutable<bool>,
    mouse_over: &Mutable<bool>
) -> DomBuilder<HtmlElement> {
    dom.class("icon")
        .class_signal("bg-lightgray", mouse_over_close.signal())
        .class_signal("invisible", signal::not(mouse_over.signal()))
}