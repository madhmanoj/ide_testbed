use dominator::{clone, html, stylesheet};
use futures_signals::{map_ref, signal::{Signal, SignalExt}};
use web_sys::HtmlDivElement;

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
