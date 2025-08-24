use dominator::{clone, events, DomBuilder};
use futures_signals::signal::{Mutable, SignalExt};
use web_sys::HtmlElement;

pub fn body(dom_builder: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom_builder
        .class("p-1")
        .class("border-transparent")
        .class("rounded")
        .class("shadow-md") 
        .style("background-color", super::FOREGROUND_COLOR)
}

pub fn option(dom_builder: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    let is_hovered = Mutable::new(false);
    dom_builder
        .class("pl-5")
        .class("pt-0.5")
        .class("hover:rounded")
        .class("text-[0.9rem]")
        .class("cursor-pointer")
        .style_signal("background-color", is_hovered.signal().map(|hover| {
            if hover {
                super::FEATURE_COLOR
            } else {
                super::FOREGROUND_COLOR
            }
        }))
        .style_signal("color", is_hovered.signal().map(|hover| {
            if hover {
                super::FOREGROUND_COLOR
            } else {
                super::TEXT_COLOR
            }
        }))
        .event(clone!(is_hovered => move |_: events::MouseEnter| {
            is_hovered.set(true);
        }))
        .event(clone!(is_hovered => move |_: events::MouseLeave| {
            is_hovered.set(false);
        }))
}
