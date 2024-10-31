use std::rc::Rc;
use futures_signals::signal::Mutable;
use dominator::{Dom, html, clone, events};

#[derive(Clone, Copy)]
pub enum Menutype {
    File, 
    Directory
}

pub struct ContextMenu {
    pub show: Mutable<bool>,
    pub position: Mutable<(i32, i32)>,
    pub menu_type: Mutable<Option<Menutype>>,
}


impl ContextMenu {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            show: Mutable::new(false),
            position: Mutable::new((0, 0)),
            menu_type: Mutable::new(None)
        })
    }

    // rendering context menu for folder
    pub fn render_menu(
        context_menu_state: Rc<ContextMenu>
    ) -> Dom {
        match context_menu_state.menu_type.get() {
            Some(Menutype::Directory) => html!("div", {
                    .class("menu")
                    .class("has-background-light")
                    .style("position", "absolute")
                    .style("border", "1px solid black")
                    .style("padding", "10px")
                    .style("z-index", "1000")
                    .style_signal("left", context_menu_state.position.signal_ref(|(x, _y)| {
                        format!("{}px", x)
                    }))
                    .style_signal("top", context_menu_state.position.signal_ref(|(_x, y)| {
                        format!("{}px", y)
                    }))
                    .children(&mut [
                        html!("div", {
                            .text("New Folder")
                            .style("cursor", "pointer")
                            .event(clone!(context_menu_state => move |_event: events::MouseDown| {
                                web_sys::console::log_1(&"New Folder Created".into());
                                context_menu_state.show.set_neq(false); // Hide the menu after clicking
                            }))
                        }),
                        html!("div", {
                            .text("New File")
                            .style("cursor", "pointer")
                            .event(clone!(context_menu_state => move |_event: events::MouseDown| {
                                web_sys::console::log_1(&"Option 2 clicked".into());
                                context_menu_state.show.set_neq(false); // Hide the menu after clicking
                            }))
                        }),
                        html!("div", {
                            .text("Rename Folder")
                            .style("cursor", "pointer")
                            .event(clone!(context_menu_state => move |_event: events::MouseDown| {
                                web_sys::console::log_1(&"Option 3 clicked".into());
                                context_menu_state.show.set_neq(false); // Hide the menu after clicking
                            }))
                        })
                    ])
                }),
            Some(Menutype::File) =>  html!("div", {
                .class("menu")
                .class("has-background-light")
                .style("position", "absolute")
                .style("border", "1px solid black")
                .style("padding", "10px")
                .style("z-index", "1000")
                .style_signal("left", context_menu_state.position.signal_ref(|(x, _y)| {
                    format!("{}px", x)
                }))
                .style_signal("top", context_menu_state.position.signal_ref(|(_x, y)| {
                    format!("{}px", y)
                }))
                .children(&mut [
                    html!("div", {
                        .text("Open File")
                        .style("cursor", "pointer")
                        .event(clone!(context_menu_state => move |_event: events::MouseDown| {
                            web_sys::console::log_1(&"Option 1 clicked".into());
                            context_menu_state.show.set_neq(false); // Hide the menu after clicking
                        }))
                    }),
                    html!("div", {
                        .text("Rename File")
                        .style("cursor", "pointer")
                        .event(clone!(context_menu_state => move |_event: events::MouseDown| {
                            web_sys::console::log_1(&"Option 2 clicked".into());
                            context_menu_state.show.set_neq(false); // Hide the menu after clicking
                        }))
                    })
                ])
            }),
            None => Dom::empty()
        }
    }
}