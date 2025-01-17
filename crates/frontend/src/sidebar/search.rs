use dominator::{html, svg, Dom};
use futures_signals::signal::{self, Signal, SignalExt};

const ICON_SVG_PATH: &str =
    "M9.5,3A6.5,6.5 0 0,1 16,9.5C16,11.11 15.41,12.59 14.44,13.73L14.71,14H15.5L20.5,\
    19L19,20.5L14,15.5V14.71L13.73,14.44C12.59,15.41 11.11,16 9.5,16A6.5,6.5 0 0,1 3,\
    9.5A6.5,6.5 0 0,1 9.5,3M9.5,5C7,5 5,7 5,9.5C5,12 7,14 9.5,14C12,14 14,12 14,9.5C14,\
    7 12,5 9.5,5Z";

#[derive(Default)]
pub struct Search {

}

impl Search {
    pub fn tooltip(&self) -> &'static str {
        "Search"
    }

    pub fn icon(&self, active: impl Signal<Item = bool> + 'static) -> Dom {
        let active = active.broadcast();
        svg!("svg", {
            .attr("viewBox", "0 0 24 24")
            .class_signal("fill-white", active.signal())
            .class_signal("fill-darkgray", signal::not(active.signal()))
            .child(svg!("path", {
                .attr("d", ICON_SVG_PATH)
            }))
        })
    }

    pub fn render(&self) -> dominator::Dom {
        html!("div", {
            .class("block")
            .class("bg-lightgray")
            .class("h-screen")
            .child(html!("div", {
                .class("block")
                .class("m-0")
                .class("h-[35px]")
                .child(html!("div", {
                    .class("icon_text")
                    .class("pl-6")
                    .class("pt-2")
                    .child(html!("span", {
                        .class("text-darkgray")
                        .class("text-[0.70em]")
                        .class("tracking-tight")
                        .class("uppercase")
                        .text("Search")
                    }))
                }))
            }))
        })
    }
}
