use dominator::DomBuilder;
use web_sys::HtmlElement;

pub fn body(dom_builder: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom_builder
        .class("h-screen")
        .style("background-color", super::BACKGROUND_COLOR)
}

pub fn title(dom_builder: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom_builder
        .class("pl-[1.625rem]")
        .class("pt-2.5")
        .class("m-0")
        .apply(super::icon_text)
}

pub fn title_text(dom_builder: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom_builder
        .class("text-[0.70em]")
        .class("tracking-tight")
        .class("uppercase")
        .style("color", super::TITLE_COLOR)
}