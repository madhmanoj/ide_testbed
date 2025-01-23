use dominator::DomBuilder;
use futures_signals::signal::{Broadcaster, Map, Signal, SignalExt};
use web_sys::{HtmlElement, SvgElement};

pub fn area(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("grid")
        .class("grid-rows-[auto_1fr]")
        .class("gap-0")
        .class("h-full")
}

pub fn background(
    dom: DomBuilder<HtmlElement>,
    height: impl Signal<Item = u32> + 'static
) -> DomBuilder<HtmlElement> {
    dom.style_signal("height", height.map(|height| format!("{height}px")))
        .style("background-image", "url('images/background.png')")
        .style("background-repeat", "no-repeat")
        .style("background-position", "center")
        .style("background-size", "auto 40%")
}

pub fn container(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("h-full")
}

pub fn editor(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("block")
        .class("h-full")
}

pub fn welcome_icon(dom: DomBuilder<SvgElement>) -> DomBuilder<SvgElement> {
    dom.class("h-[1.5em]")
        .class("fill-coreblue")
}

pub fn welcome_1(
    dom: DomBuilder<HtmlElement>,
    height: impl Signal<Item = u32> + 'static
) -> DomBuilder<HtmlElement> {
    dom.class("justify-center")
        .class("gap-0")
        .style("overflow-y", "scroll")
        .style_signal("height", height.map(|height| format!("{height}px")))
}

pub fn welcome_2(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("place-items-center")
        .class("h-full")
}

pub fn welcome_3(
    dom: DomBuilder<HtmlElement>,
    content_max_width: Broadcaster<Map<impl Signal<Item = u32> + 'static, impl FnMut(u32) -> u32 + 'static>>
) -> DomBuilder<HtmlElement> {
    dom.class("block")
        .class("h-full")
        .class("place-content-center")
        .class("py-6")
        .style_signal("max-width", content_max_width.signal_ref(|width| format!("{width}px")))
}