use std::rc::Rc;

use activity_panel::ActivityPanel;
use dominator::{clone, events, html, Dom, EventOptions};
use futures_signals::{signal::{Mutable, Signal, SignalExt}, signal_vec::MutableVec};
use crate::styles;
use panel::{GridAttributes, LayoutPanel};

pub mod console;
pub mod activity_panel;
pub mod panel;

const DEFAULT_CONSOLE_HEIGHT: u32 = 200;
const RESIZER_PX: u32 = 3;

pub struct Workspace {
    pub activity_panels: Rc<LayoutPanel>,
    pub active_panel: Mutable<Rc<ActivityPanel>>,
    console: Rc<console::Console>,
    pub console_height: Mutable<u32>,
    resize_active: Mutable<bool>,
    resizer_hover: Mutable<bool>,
}

impl Default for Workspace {
    fn default() -> Self {
        let activity_panels = Rc::new(LayoutPanel::VerticalSplit { parent: None, children: MutableVec::default()});
        let panel = ActivityPanel::default();
        activity_panels.append_widget(100.0, panel.clone());

        Self {
            activity_panels,
            active_panel: Mutable::new(panel.clone()),
            console: Default::default(),
            console_height: Mutable::new(DEFAULT_CONSOLE_HEIGHT),
            resize_active: Mutable::new(false),
            resizer_hover: Mutable::new(false),
        }
    }
}
   
impl Workspace {
    pub fn render_activity_panel(
        this: &Rc<Workspace>,
        width: impl Signal<Item = u32> + 'static,
        height: impl Signal<Item = u32> + 'static
    ) -> Dom {
        let width = width.map(|width| width as f64).broadcast();
        let height = height.map(|height| height as f64).broadcast();

        html!("div", {
            .class("col-span-1")
            .class("row-span-1")
            .child(LayoutPanel::render(
                this,
                &this.activity_panels,
                GridAttributes { column_start: 1, column_end: 2, row_start: 1, row_end: 2 },
                width.signal().boxed_local(),
                height.signal().boxed_local(),
            ))
        })
    }

    pub fn render_horizontal_resizer(this: &Rc<Workspace>) -> Dom {
        html!("div", {
            .class("col-span-1")
            .class("row-span-1")
            .class("cursor-ns-resize")
            .style("height", &format!("{RESIZER_PX}px"))
            .apply(|dom| styles::resizer(dom, this.resize_active.signal(), this.resizer_hover.signal()))
            .event_with_options(&EventOptions::preventable(),
                clone!(this => move |ev: events::PointerDown| {
                this.resize_active.set_neq(true);
                ev.prevent_default();
            }))
            .global_event(clone!(this => move |_: events::PointerUp| {
                this.resize_active.set_neq(false);
                if this.console_height.get() == 0 {
                    // close console and reset default size, this could be a boolean
                    // e.g., console visible OR we could use something more similar
                    // to the sidebar/menu logic
                    //this.active_panel.set(None);
                    this.console_height.set(DEFAULT_CONSOLE_HEIGHT)
                }
            }))
            .event(clone!(this => move |_: events::PointerEnter| {
                this.resizer_hover.set_neq(true);
            }))
            .event(clone!(this => move |_: events::PointerLeave| {
                this.resizer_hover.set_neq(false);
            }))
            .global_event(clone!(this => move |event: events::PointerMove| {
                if this.resize_active.get() {
                    let available_height = web_sys::window()
                        .unwrap()
                        .inner_height()
                        .unwrap()
                        .as_f64()
                        .map(|window_size| window_size.max(0.0))
                        .unwrap() as u32;
                    let console_height = available_height
                        .saturating_sub(event.y().max(0) as u32 + RESIZER_PX);
                    match console_height {
                        0..=75 => {
                            this.console_height.set(0);
                        }
                        76..=150 => {}
                        _ => {
                            this.console_height.set(console_height);
                        }
                    }
                }
            }))
        })
    }

    pub fn render_console(
        this: &Rc<Workspace>,
        console_width: impl Signal<Item = u32> + 'static
    ) -> Dom {
        let console_width = console_width.broadcast();
        html!("div", {
            .class("col-span-1")
            .class("row-span-1")
            // we set both width and height since the fit addon needs to know the height of the terminal container
            // explicitly
            .style_signal("height", this.console_height.signal().map(|height| format!("{height}px")))
            .style_signal("width", console_width.signal().map(|width| format!("{width}px")))
            // .apply(styles::console::container)
            .child(this.console.render(console_width.signal(), this.console_height.signal()))
        })
    }
}