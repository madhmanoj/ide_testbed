use dominator::DomBuilder;
use futures_signals::signal::{self, Broadcaster, Mutable, Signal, SignalExt};
use web_sys::{HtmlElement, HtmlInputElement, SvgElement};
pub fn container(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("grid")
        .class("grid-cols-[auto_1fr]")
        .class("gap-0")
}

// SIDEBAR STYLES

pub fn sidebar(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("grid")
        .class("grid-cols-[auto_auto_auto]")
        .class("gap-0")
}

pub fn sidebar_menu_container(dom: DomBuilder<HtmlElement>, size_px: &u32) -> DomBuilder<HtmlElement> {
    dom.style("width", &format!("{size_px}px"))
}

pub fn menu(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("block")
        .class("bg-offblack")
        .class("min-h-screen")
}

pub fn menu_btn(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("px-2")
        .class("pt-4")
        .class("cursor-pointer")
}

pub fn menu_btn_explorer(dom: DomBuilder<SvgElement>, active: &Broadcaster<impl Signal<Item = bool> + 'static>) -> DomBuilder<SvgElement> {
    dom.class_signal("fill-white", active.signal())
        .class_signal("fill-darkgray", signal::not(active.signal()))
}

pub fn menu_btn_search(dom: DomBuilder<SvgElement>, active: &Broadcaster<impl Signal<Item = bool> + 'static>) -> DomBuilder<SvgElement> {
    dom.class_signal("fill-white", active.signal())
        .class_signal("fill-darkgray", signal::not(active.signal()))
}

pub fn panel_container(dom: DomBuilder<HtmlElement>, size: &Mutable<u32>) -> DomBuilder<HtmlElement> {
    dom.style_signal("width", size.signal_ref(|s| format!("{s}px")))
        .class_signal("hidden", size.signal().eq(0))
}

pub fn panel(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("block")
        .class("bg-lightgray")
        .class("h-screen")
}

pub fn panel_title_container(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("block")
        .class("m-0")
        .class("h-[35px]")
}

pub fn panel_title(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("icon_text")
        .class("pl-6")
        .class("pt-2")
}

pub fn panel_title_text(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("text-darkgray")
        .class("text-[0.70em]")
        .class("tracking-tight")
        .class("uppercase")
}

pub fn vfs_item_container(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("pl-5")
        .class("pt-0")
}

pub fn vfs_item(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("icon_text")
        .class("grid")
        .class("grid-cols-[auto_1fr]")
        .class("p-[2px]")
        .class("cursor-pointer")
}

pub fn vfs_item_icon(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("mr-0")
        .class("icon")
}

pub fn panel_input(dom: DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> {
    dom.class("w-full")
        .class("focus:outline-none")
        .class("focus:ring-2")
        .class("ring-coreblue")
}

pub fn vertical_resizer(
    dom: DomBuilder<HtmlElement>, 
    width: &u32, 
    active: &Mutable<bool>, 
    hover: &Mutable<bool>
) -> DomBuilder<HtmlElement> {
    dom.class("cursor-ew-resize")
        .class("min-h-screen")
        .style("width", &format!("{width}px"))
        .class_signal("bg-lightgray", signal::not(signal::or(active.signal(), hover.signal())))
        .class_signal("bg-coreblue", signal::or(active.signal(), hover.signal()))
}

// WORKSPACE STYLES