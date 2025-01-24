use dominator::DomBuilder;
use futures_signals::signal::{Mutable, SignalExt};
use web_sys::{HtmlElement, HtmlInputElement};

pub fn container(dom: DomBuilder<HtmlElement>, size: &Mutable<u32>) -> DomBuilder<HtmlElement> {
    dom.style_signal("width", size.signal_ref(|s| format!("{s}px")))
        .class_signal("hidden", size.signal().eq(0))
}

pub fn body(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("block")
        .class("bg-lightgray")
        .class("h-screen")
}

pub fn title(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("block")
        .class("m-0")
        .class("h-[35px]")
        .class("icon_text")
        .class("pl-[1.625rem]")
        .class("pt-2")
}

pub fn title_text(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("text-darkgray")
        .class("text-[0.70em]")
        .class("tracking-tight")
        .class("uppercase")
}

pub fn input(dom: DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> {
    dom.class("w-full")
        .class("focus:outline-none")
        .class("focus:ring-2")
        .class("ring-coreblue")
}