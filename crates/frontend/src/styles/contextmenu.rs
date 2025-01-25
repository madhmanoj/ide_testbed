use dominator::DomBuilder;
use web_sys::HtmlElement;

pub fn body(
    dom: DomBuilder<HtmlElement>,
    x: &i32,
    y: &i32
) -> DomBuilder<HtmlElement> {
    dom.class("absolute")
        .class("z-[1000]")
        .class("w-60")
        .class("p-1")
        .class("border-transparent")
        .class("rounded")
        .class("shadow-md")
        .style("left", &format!("{}px", x)) // X position
        .style("top", &format!("{}px", y)) // Y position
        .style("background-color", super::FOREGROUND_COLOR)
}

pub fn option(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("pl-5")
        .class("pt-0.5")
        .class("bg-inherit")
        .class("hover:bg-coreblue")
        .class("hover:rounded")
        .class("hover:text-white")
        .class("text-[0.9rem]")
        .class("cursor-pointer")
}
