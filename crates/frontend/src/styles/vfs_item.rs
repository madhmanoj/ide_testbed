use dominator::DomBuilder;
use web_sys::HtmlElement;

pub fn list(dom_builder: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom_builder
        .class("pl-5")
        .class("pt-0")
}

pub fn body(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom
        .apply(super::icon_text)
        // this grid doesnt change it is just there to make the icon for file and the filename remain inline
        // so we can leave it in styles
        .class("grid")
        .class("grid-cols-[auto_1fr]")
        .class("p-[2px]")
        .class("cursor-pointer")
}

pub fn icon(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom
        .apply(super::icon)
        .class("mr-0")
}