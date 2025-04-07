use dominator::{clone, html, stylesheet};
use futures_signals::{map_ref, signal::{Signal, SignalExt}};
use web_sys::HtmlDivElement;
// use crate::styles;
// use once_cell::sync::Lazy;
// use regex::Regex;
// use chrono::DateTime;
// use std::{sync::Arc, time::{Duration, UNIX_EPOCH}};

use xterm_js_sys::{Terminal, FitAddon};

#[derive(Default)]
pub struct Console {}

impl Console {
    pub fn render(
        &self,
        height: impl Signal<Item = u32> + 'static,
        width: impl Signal<Item = u32> + 'static
    ) -> dominator::Dom {
        stylesheet!(".xterm", {
            // have to give explicit height for the xterm terminal so that it takes up the whole space
            .style("height", "100%")
            .style("cursor", "text")
            .style("position", "relative")
            .style("user-select", "none")
        });

        stylesheet!(".xterm .xterm-helpers", {
            .style("position", "absolute")
            .style("top", "0")
            .style("z-index", "5")
        });
        
        stylesheet!(".xterm .xterm-viewport", {
            .style("background-color", "#000")
            .style("overflow-y", "scroll")
            .style("cursor", "default")
            .style("position", "absolute")
            .style("right", "0")
            .style("left", "0")
            .style("top", "0")
            .style("bottom", "0")
        });

        stylesheet!(".xterm .xterm-screen", {
            .style("position", "relative")
        });
        
        stylesheet!(".xterm .xterm-scroll-area", {
            .style("visibility", "hidden")
        });  

        stylesheet!(".xterm .xterm-helper-textarea", {
            .style("padding", "0")
            .style("border", "0")
            .style("margin", "0")
            .style("position", "absolute")
            .style("opacity", "0")
            .style("left", "-9999em")
            .style("top", "0")
            .style("width", "0")
            .style("height", "0")
            .style("z-index", "-5")
            .style("white-space", "nowrap")
            .style("overflow", "hidden")
            .style("resize", "none")
        });

        stylesheet!(".xterm .composition-view", {
            .style("background", "#000")
            .style("color", "#FFF")
            .style("display", "none")
            .style("position", "absolute")
            .style("white-space", "nowrap")
            .style("z-index", "1")
        });
        
        stylesheet!(".xterm-char-measure-element", {
            .style("display", "inline-block")
            .style("visibility", "hidden")
            .style("position", "absolute")
            .style("top", "0")
            .style("left", "-9999em")
            .style("line-height", "normal")
        });        

        let terminal = Terminal::new();
        let fit = FitAddon::new();
        terminal.loadAddon(&fit);

        let sum_signal = map_ref! {
            width,
            height => {
                width + height
            }
        };

        html!("div" => HtmlDivElement, {
            .class("h-full")
            .class("w-full")
            .future(sum_signal.for_each(clone!(fit => move |_| {
                fit.fit();
                async {}
            })))
            .after_inserted(move |el| {
                terminal.open(&el);
                fit.fit();
                for i in 0..=30 {
                    terminal.write(&format!("HELLO{}\r\n", i));
                }
            })
        })
    }
}

// fn render_entry(message: Arc<str>) -> Dom {
//     static PATTERN: Lazy<Regex> =
//         Lazy::new(|| Regex::new(r"^\[([A-Z]+)\] \[([0-9]+\.[0-9]+)] \[([^\]]+)\]: (.+)$").unwrap());
//     let structured_message = PATTERN.captures(&message)
//         .and_then(|captures| match (captures.get(1), captures.get(2), captures.get(3), captures.get(4)) {
//             (Some(category), Some(timestamp), Some(node), Some(message)) => Some((category, timestamp, node, message)),
//             _ => None
//         })
//         .map(|(category, timestamp, node, message)| {
//             html!("p", {
//                 .class("p-1")
//                 .child(render_category(category.as_str()))
//                 .child(render_timestamp(timestamp.as_str()))
//                 .child(render_node(node.as_str()))
//                 .child(render_message(message.as_str()))
//             })
//         });
    
//     structured_message.unwrap_or(html!("p", { .text(&message) }))
// }

// fn render_message(node: &str) -> Dom {
//     html!("div", {
//         .apply(styles::console::render_object)
//         .text(&node)
//     })
// }

// fn render_node(node: &str) -> Dom {
//     html!("div", {
//         .apply(styles::console::render_object)
//         .text(&node)
//     })
// }

// fn render_timestamp(timestamp: &str) -> Dom {
//     if let Ok(duration) = timestamp.parse::<f64>().map(Duration::from_secs_f64) {
//         let datetime = DateTime::<chrono::Local>::from(UNIX_EPOCH + duration)
//             .format("%Y-%m-%d %H:%M:%S")
//             .to_string();
//         html!("div", {
//             .apply(styles::console::render_object)
//             .text(&datetime)
//         })
//     }
//     else {
//         html!("div", {
//             .apply(styles::console::render_object)
//             .text(&timestamp)
//         })
//     }
// }

// fn render_category(category: &str) -> Dom {
//     html!("div", {
//         .apply(|builder| match category {
//             "INFO" => builder.text("info").class("bg-[#48c774]"), // green
//             "WARN" => builder.text("warn").class("bg-[#ffdd57]"), // yellow
//             "ERROR" => builder.text("error").class("bg-[#ff3860]"), // red
//             _ => builder.text("unknown")
//         })
//         .class("uppercase")
//         .style("min-width", "65px")
//         .style("letter-spacing", ".1em")
//         .apply(styles::console::render_object)
//     })
// }
