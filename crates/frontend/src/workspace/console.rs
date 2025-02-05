use std::{sync::Arc, time::{Duration, UNIX_EPOCH}};

use chrono::DateTime;
use dominator::{html, Dom};
use futures_signals::signal_vec::SignalVecExt;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::styles;

#[derive(Default)]
pub struct Console {}

impl Console {
    pub fn render(&self) -> dominator::Dom {
        html!("div", {
            .class("grid")
            .class("grid-rows-[auto_1fr]")
            .class("h-full")
            .child(html!("div", {
                .apply(styles::console::title)
                .child(html!("span", {
                    .apply(styles::console::title_text)
                    .text("Log messages")
                }))
            }))
            .child(html!("div", {
                .class("block")
                .style("overflow-y", "scroll")
                .apply(styles::console::message_area)
                .children_signal_vec(crate::GLOBAL_LOG.with(|messages| messages
                    .signal_vec_cloned().map(render_entry)))
                .scroll_top_signal(crate::GLOBAL_LOG.with(|messages| messages
                    .signal_vec_cloned().to_signal_map(|_| Some(i32::MAX))))
            }))
        })
    }
}

fn render_entry(message: Arc<str>) -> Dom {
    static PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^\[([A-Z]+)\] \[([0-9]+\.[0-9]+)] \[([^\]]+)\]: (.+)$").unwrap());
    let structured_message = PATTERN.captures(&message)
        .and_then(|captures| match (captures.get(1), captures.get(2), captures.get(3), captures.get(4)) {
            (Some(category), Some(timestamp), Some(node), Some(message)) => Some((category, timestamp, node, message)),
            _ => None
        })
        .map(|(category, timestamp, node, message)| {
            html!("p", {
                .class("p-1")
                .child(render_category(category.as_str()))
                .child(render_timestamp(timestamp.as_str()))
                .child(render_node(node.as_str()))
                .child(render_message(message.as_str()))
            })
        });
    
    structured_message.unwrap_or(html!("p", { .text(&message) }))
}

fn render_message(node: &str) -> Dom {
    html!("div", {
        .apply(styles::console::render_object)
        .text(&node)
    })
}

fn render_node(node: &str) -> Dom {
    html!("div", {
        .apply(styles::console::render_object)
        .text(&node)
    })
}

fn render_timestamp(timestamp: &str) -> Dom {
    if let Ok(duration) = timestamp.parse::<f64>().map(Duration::from_secs_f64) {
        let datetime = DateTime::<chrono::Local>::from(UNIX_EPOCH + duration)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        html!("div", {
            .apply(styles::console::render_object)
            .text(&datetime)
        })
    }
    else {
        html!("div", {
            .apply(styles::console::render_object)
            .text(&timestamp)
        })
    }
}

fn render_category(category: &str) -> Dom {
    html!("div", {
        .apply(|builder| match category {
            "INFO" => builder.text("info").class("bg-[#48c774]"), // green
            "WARN" => builder.text("warn").class("bg-[#ffdd57]"), // yellow
            "ERROR" => builder.text("error").class("bg-[#ff3860]"), // red
            _ => builder.text("unknown")
        })
        .class("uppercase")
        .style("min-width", "65px")
        .style("letter-spacing", ".1em")
        .apply(styles::console::render_object)
    })
}
