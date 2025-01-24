use dominator::DomBuilder;
use futures_signals::signal::{self, Mutable};
use web_sys::HtmlElement;

pub mod menu;
pub mod tab;
pub mod panel;
pub mod console;
pub mod activity;

const RESIZER_PX: u32 = 2;

// MAIN CONTAINER

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

pub fn vfs_item_list(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
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

pub fn vertical_resizer(
    dom: DomBuilder<HtmlElement>, 
    active: &Mutable<bool>, 
    hover: &Mutable<bool>
) -> DomBuilder<HtmlElement> {
    dom.class("cursor-ew-resize")
        .class("min-h-screen")
        .style("width", &format!("{RESIZER_PX}px"))
        .class_signal("bg-lightgray", signal::not(signal::or(active.signal(), hover.signal())))
        .class_signal("bg-coreblue", signal::or(active.signal(), hover.signal()))
}

// WORKSPACE STYLES

pub fn workspace(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("grid")
        .class("grid-rows-[1fr_auto_auto]")
        .class("gap-0")
        .class("h-full")
}

pub fn horizontal_resizer(
    dom: DomBuilder<HtmlElement>,
    active: &Mutable<bool>,
    hover: &Mutable<bool>
) -> DomBuilder<HtmlElement> {
    dom.style("cursor", "ns-resize")
        .style("height", &format!("{RESIZER_PX}px"))
        .class_signal("bg-lightgray",
            signal::not(signal::or(active.signal(),hover.signal())))
        .class_signal("bg-coreblue",
            signal::or(active.signal(), hover.signal()))
}

// CONTEXTMENU STYLES

pub fn contextmenu(
    dom: DomBuilder<HtmlElement>,
    x: &i32,
    y: &i32
) -> DomBuilder<HtmlElement> {
    dom.class("bg-white")
        .class("absolute")
        .class("z-[1000]")
        .class("w-60")
        .class("p-1")
        .class("rounded")
        .class("shadow-md")
        .class("border-transparent")
        .style("left", &format!("{}px", x)) // X position
        .style("top", &format!("{}px", y)) // Y position
}

pub fn contextmenu_option(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("text-[0.9rem]")
        .class("pl-5")
        .class("pt-0.5")
        .class("bg-inherit")
        .class("hover:bg-coreblue")
        .class("hover:rounded")
        .class("hover:text-white")
        .class("cursor-pointer")
}