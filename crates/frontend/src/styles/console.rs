use dominator::DomBuilder;
use futures_signals::signal::{Mutable, SignalExt};
use web_sys::HtmlElement;

pub fn container(
    dom: DomBuilder<HtmlElement>,
    height: &Mutable<u32>
) -> DomBuilder<HtmlElement> {
    dom.class("h-full")
        .class("bg-lightgray")
        .style_signal("height", height.signal()
            .map(|height| format!("{height}px")))
}

pub fn body(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("grid")
        .class("grid-rows-[auto_1fr]")
        .class("m-0")
        .class("h-full")
}

pub fn title(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("icon-text")
        .class("p-3")
        .class("m-0")
}

pub fn title_text(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("text-xs")
        .class("uppercase")
        .class("tracking-widest")
}

pub fn message_area(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("p-2")
        .class("block")
        .style("overflow-y", "scroll")
        .class("bg-white")
}

pub fn message(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("tag")
        .class("bg-white")
        .class("text-[#090a0c]")
}

pub fn node(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("tag")
        .class("bg-white")
        .class("text-[#090a0c]")
}

pub fn timestamp(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("tag")
        .class("bg-white")
        .class("text-[#090a0c]")
}

pub fn category(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("tag")
        .class("bg-[#f3f4f6]")
        .class("text-[#2e333d]")
        .class("uppercase")
        .style("min-width", "65px")
        .style("letter-spacing", ".1em")
}

pub fn warning(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("bg-[#ffdd57]")
}

pub fn error(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("bg-[#ff3860]")
}

pub fn success(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("bg-[#48c774]")
}