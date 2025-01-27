use dominator::DomBuilder;
use futures_signals::signal::{Signal, SignalExt};
use web_sys::HtmlElement;

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

pub fn container(
    dom: DomBuilder<HtmlElement>,
    height: impl Signal<Item = u32> + 'static
) -> DomBuilder<HtmlElement> {
    dom.style("background-color", super::BACKGROUND_COLOR)
        .style_signal("height", height
            .map(|height| format!("{height}px")))
}

pub fn title_text(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("text-xs")
        .class("uppercase")
        .class("tracking-widest")
}

pub fn message_area(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("block")
        .class("p-2")
        .style("background-color", super::FOREGROUND_COLOR)
        .style("overflow-y", "scroll")
}

pub fn render_object(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.apply(tag)
        .style("background-color", super::FOREGROUND_COLOR)
        .style("color", super::TEXT_COLOR)
}