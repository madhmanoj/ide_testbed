use dominator::DomBuilder;
use futures_signals::signal::{self, Broadcaster, Signal};
use web_sys::{HtmlElement, SvgElement};

const MENU_SIZE_PX: u32 = 48;

pub fn container(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.style("width", &format!("{MENU_SIZE_PX}px"))
}

pub fn body(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("block")
        .class("bg-mineshaft")
        .class("min-h-screen")
}

pub fn button(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("px-2")
        .class("pt-4")
        .class("cursor-pointer")
}

pub fn button_explorer(dom: DomBuilder<SvgElement>, active: &Broadcaster<impl Signal<Item = bool> + 'static>) -> DomBuilder<SvgElement> {
    dom.class_signal("fill-white", active.signal())
        .class_signal("fill-darkgray", signal::not(active.signal()))
}

pub fn button_search(dom: DomBuilder<SvgElement>, active: &Broadcaster<impl Signal<Item = bool> + 'static>) -> DomBuilder<SvgElement> {
    dom.class_signal("fill-white", active.signal())
        .class_signal("fill-darkgray", signal::not(active.signal()))
}