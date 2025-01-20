use dominator::DomBuilder;
use web_sys::HtmlElement;

pub fn container(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("grid")
       .class("grid-cols-[auto_1fr]")
       .class("gap-0")
}