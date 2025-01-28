use dominator::DomBuilder;
use futures_signals::signal::{self, Signal, SignalExt};
use web_sys::HtmlElement;

pub fn bar(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("gap-0")
        .style("background-color", super::BACKGROUND_COLOR)
}

pub fn body(dom: DomBuilder<HtmlElement>,
    is_active: impl Signal<Item = bool> + 'static,
    mouse_over: impl Signal<Item = bool> + 'static
) -> DomBuilder<HtmlElement> {
    dom.class("pt-1.5")
        .class("pl-1")
        .class("pr-2")
        .class("gap-1")
        .class("cursor-pointer")
        .style_signal("background-color", signal::or(is_active, mouse_over).map(|active| {
            if active {
                super::FOREGROUND_COLOR
            } else {
                "transparent"
            }
        }))
}

pub fn icon(
    dom: DomBuilder<HtmlElement>,
    mouse_over_close: impl Signal<Item = bool> + 'static,
    mouse_over: impl Signal<Item = bool> + 'static
) -> DomBuilder<HtmlElement> {
    dom.class_signal("invisible", signal::not(mouse_over))
        .style_signal("background-color", mouse_over_close.map(|flag| {
            if flag {
                super::BACKGROUND_COLOR
            } else {
                "transparent"
            }
        }))
        .apply(super::icon)
}