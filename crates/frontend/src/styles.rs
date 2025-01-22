use dominator::DomBuilder;
use futures_signals::signal::{self, Broadcaster, Mutable, Signal, SignalExt};
use web_sys::{HtmlElement, HtmlInputElement, SvgElement};

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

pub fn workspace(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("grid")
        .class("grid-rows-[1fr_auto_auto]")
        .class("gap-0")
        .class("h-full")
}

pub fn activity_area(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("grid")
        .class("grid-rows-[auto_1fr]")
        .class("gap-0")
        .class("h-full")
}

pub fn tab_bar_container(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("row-span-1")
        .class("bg-lightgray")
        .class("h-[35px]")
}

pub fn tab_bar(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("inline-flex")
        .class("h-full")
        .class("gap-0")
}

pub fn tab_container(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("h-full")
        .class("w-[110px]")
}

pub fn tab(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("block")
        .class("h-full")
        .class("gap-1")
        .class("pt-1.5w")
        .class("cursor-pointer")
}

pub fn tab_content(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("icon_text")
        .class("inline-flex")
}

pub fn tab_icon_default(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("icon")
}

pub fn tab_icon(
    dom: DomBuilder<HtmlElement>,
    mouse_over_close: &Mutable<bool>,
    mouse_over: &Mutable<bool>
) -> DomBuilder<HtmlElement> {
    dom.class("icon")
        .class_signal("bg-lightgray", mouse_over_close.signal())
        .class_signal("invisible", signal::not(mouse_over.signal()))
}

pub fn welcome_icon(dom: DomBuilder<SvgElement>) -> DomBuilder<SvgElement> {
    dom.class("h-[1.5em]")
        .class("fill-coreblue")
}

pub fn activity_editor(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("h-full")
}

pub fn horizontal_resizer(
    dom: DomBuilder<HtmlElement>,
    height: &u32,
    active: &Mutable<bool>,
    hover: &Mutable<bool>
) -> DomBuilder<HtmlElement> {
    dom.style("cursor", "ns-resize")
        .style("height", &format!("{height}px"))
        .class_signal("bg-lightgray",
            signal::not(signal::or(active.signal(),hover.signal())))
        .class_signal("bg-coreblue",
            signal::or(active.signal(), hover.signal()))
}

pub fn console_container(
    dom: DomBuilder<HtmlElement>,
    height: &Mutable<u32>
) -> DomBuilder<HtmlElement> {
    dom.style_signal("height", height.signal()
        .map(|height| format!("{height}px")))
}

pub fn console(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("h-full")
        .class("bg-lightgray")
}

pub fn console_title_container(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("block")
        .class("p-3")
        .class("m-0")
}

pub fn console_title(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("icon-text")
}

pub fn console_title_text(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("text-xs")
        .class("uppercase")
        .class("tracking-widest")
}

pub fn console_message_area(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("p-2")
        .class("block")
        .style("overflow-y", "scroll")
        .style("height", "calc(100% - 40px)") // 40 px for the block above
        .class("bg-white")
}

// CONTEXTMENU STYLES

pub fn contextmenu(dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    dom.class("bg-white")
        .class("absolute")
        .class("z-[1000]")
        .class("w-60")
        .class("p-1")
        .class("rounded")
        .class("shadow-md")
        .class("border-transparent")
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