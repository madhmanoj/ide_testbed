use dominator::DomBuilder;
use futures_signals::signal::{Signal, SignalExt};
use web_sys::{HtmlElement, SvgElement};

const MENU_SIZE_PX: u32 = 48;
use super::TEXT_COLOR as MENU_BACKGROUND_COLOR;

pub fn body(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("block")
        .class("min-h-screen")
        .style("width", &format!("{MENU_SIZE_PX}px"))
        .style("background-color", MENU_BACKGROUND_COLOR)
}

pub fn button(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("px-2")
        .class("pt-4")
        .class("cursor-pointer")
}

pub fn button_toggle(
    dom: DomBuilder<SvgElement>,
    active: impl Signal<Item = bool> + 'static
) -> DomBuilder<SvgElement> {
    let active = active.broadcast();
    dom.attr_signal("fill", active.signal().map(|active| {
        if active {
            super::FOREGROUND_COLOR
        } else {
            super::TITLE_COLOR
        }
    }))
}