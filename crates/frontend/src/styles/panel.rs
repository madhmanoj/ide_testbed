use dominator::DomBuilder;
use web_sys::HtmlElement;

pub fn body(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("block")
        .class("h-screen")
        .style("background-color", super::BACKGROUND_COLOR)
}

pub fn title(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.apply(super::icon_text)
        .class("h-[35px]")
        .class("pl-[1.625rem]")
        .class("pt-2.5")
        .class("m-0")
}

pub fn title_text(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("text-[0.70em]")
        .class("tracking-tight")
        .class("uppercase")
        .style("color", super::TITLE_COLOR)
}