use dominator::DomBuilder;
use web_sys::HtmlElement;

pub fn list(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("pl-5")
        .class("pt-0")
}

pub fn body(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.apply(super::icon_text)
        .class("grid")
        .class("grid-cols-[auto_1fr]")
        .class("p-[2px]")
        .class("cursor-pointer")
}

pub fn icon(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.apply(super::icon)
        .class("mr-0")
}