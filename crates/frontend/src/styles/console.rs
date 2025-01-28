use dominator::DomBuilder;
use web_sys::HtmlElement;

pub fn container(
    dom: DomBuilder<HtmlElement>,
) -> DomBuilder<HtmlElement> {
    dom.style("background-color", super::BACKGROUND_COLOR)
}

pub fn title(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("p-3")
        .apply(super::icon_text)
}

pub fn title_text(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("text-xs")
        .class("uppercase")
        .class("tracking-widest")
}

pub fn message_area(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("p-2")
        .style("background-color", super::FOREGROUND_COLOR)
}

pub fn render_object(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.style("background-color", super::FOREGROUND_COLOR)
        .style("color", super::TEXT_COLOR)
        .apply(tag)
}

// HELPER FUNCTION

pub fn tag(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("inline-flex")
        .class("h-[2em]")
        .class("pr-[0.75em]")
        .class("pl-[0.75em]")
        .class("rounded-md")
        .class("text-xs")
        .class("leading-[1.5]")
        .class("justify-center")
        .class("items-center")
        .class("whitespace-nowrap")
}