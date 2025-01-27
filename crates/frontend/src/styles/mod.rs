use dominator::DomBuilder;
use futures_signals::signal::{self, Signal, SignalExt};
use web_sys::{HtmlElement, HtmlInputElement, SvgElement};

pub mod menu;
pub mod tab;
pub mod panel;
pub mod console;
pub mod contextmenu;
pub mod vfs_item;

const BACKGROUND_COLOR: &str = "#f3f3f3"; // lightgray
const FEATURE_COLOR: &str = "#007acc"; // blue
const FOREGROUND_COLOR: &str = "#ffffff"; // TOGGLE_ACTIVE_COLOR -- white
const TITLE_COLOR: &str = "#828282"; // TOGGLE_INACTIVE_COLOR -- darkgray
const TEXT_COLOR: &str = "#2c2c2c"; // mineshaft

pub fn default_layout(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("grid")
        .class("h-full")
        .class("gap-0")
}

pub fn sidebar_layout(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("col-span-1")
        .class("row-span-3")
}

pub fn workspace_layout(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("col-span-1")
        .class("row-span-1")
}

pub fn resizer(
    dom: DomBuilder<HtmlElement>,
    active: impl Signal<Item = bool> + 'static,
    hover: impl Signal<Item = bool> + 'static
) -> DomBuilder<HtmlElement> {
    dom.style_signal("background-color", signal::or(active, hover).map(|flag| {
        if flag {
            FEATURE_COLOR
        } else {
            BACKGROUND_COLOR
        }
    }))
}

pub fn input(dom: DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> {
    dom.class("w-full")
        .class("ring-coreblue")
        .class("focus:outline-none")
        .class("focus:ring-2")
}

pub fn icon(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("grid")
        .class("w-6")
        .class("place-items-center")
        .style("transition-property", "color")
        .style("transition-duration", "294ms")
}

pub fn icon_text(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("grid")
        .class("grid-flow-col")
        .class("auto-cols-max")
        .class("gap-0")
        .class("text-inherit")
        .class("leading-5")
}

pub fn welcome_icon(dom: DomBuilder<SvgElement>) -> DomBuilder<SvgElement> {
    dom.class("h-[1.5em]")
        .attr("fill", FEATURE_COLOR)
}

