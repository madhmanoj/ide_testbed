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

pub fn vertical_resizer(
    dom_builder: DomBuilder<HtmlElement>,
    active: impl Signal<Item = bool> + 'static,
    hover: impl Signal<Item = bool> + 'static
) -> DomBuilder<HtmlElement> {
    dom_builder
        .class("max-h-screen")
        .class("cursor-ew-resize")
        .apply(|dom| resizer(dom, active, hover))
}

pub fn welcome_icon(dom_builder: DomBuilder<SvgElement>) -> DomBuilder<SvgElement> {
    dom_builder
        .class("h-[1.5em]")
        .attr("fill", FEATURE_COLOR)
}

pub fn input(dom_builder: DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> {
    dom_builder
        .class("w-full")
        .class("focus:outline-none")
        .style("border", format!("2px solid {}", FEATURE_COLOR))
        .style("box-shadow", "none")
}

// HELPER FUNCTIONS

pub fn icon(dom_builder: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom_builder
        .class("grid")
        .class("w-6")
        .class("place-items-center")
        .style("transition-property", "color")
        .style("transition-duration", "294ms")
}

pub fn icon_text(dom_builder: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom_builder
        .class("grid")
        .class("grid-flow-col")
        .class("auto-cols-max")
        .class("gap-0")
        .class("text-inherit")
        .class("leading-5")
}

pub fn resizer(
    dom_builder: DomBuilder<HtmlElement>,
    active: impl Signal<Item = bool> + 'static,
    hover: impl Signal<Item = bool> + 'static
) -> DomBuilder<HtmlElement> {
    dom_builder
        .style_signal("background-color", signal::or(active, hover).map(|flag| {
            if flag {
                FEATURE_COLOR
            } else {
                BACKGROUND_COLOR
            }
        }))
}