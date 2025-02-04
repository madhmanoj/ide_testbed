use std::{collections::BTreeMap, rc::Rc};

use activity_panel::ActivityPanel;
use dominator::{clone, events, html, Dom, EventOptions};
use futures_signals::{signal::{Mutable, Signal, SignalExt}, signal_map::MutableBTreeMap, signal_vec::{MutableVec, SignalVecExt}};
use uuid::Uuid;
use crate::styles;

pub mod console;
pub mod activity_panel;

const DEFAULT_CONSOLE_HEIGHT: u32 = 200;
const RESIZER_PX: u32 = 3;

#[derive(Clone)]
pub enum ColumnType {
    Auto,
    Fr
}

pub struct Workspace {
    pub activity_panel_list: MutableBTreeMap<Uuid, Rc<activity_panel::ActivityPanel>>,
    console: Rc<console::Console>,
    pub console_height: Mutable<u32>,
    resize_active: Mutable<bool>,
    resizer_hover: Mutable<bool>,
    pub last_active_panel: Mutable<Uuid>,
    pub cols: MutableVec<ColumnType>
}

impl Default for Workspace {
    fn default() -> Self {
        let uuid = Uuid::new_v4();

        Self {
            activity_panel_list: MutableBTreeMap::with_values(BTreeMap::from([
                (uuid, ActivityPanel::default())
            ])),
            console: Default::default(),
            console_height: Mutable::new(DEFAULT_CONSOLE_HEIGHT),
            resize_active: Mutable::new(false),
            resizer_hover: Mutable::new(false),
            last_active_panel: Mutable::new(uuid),
            cols: MutableVec::new_with_values(vec![
                ColumnType::Fr
            ])
        }
    }
}
   
// part of the problem is that I need to respond to the user moving the mouse, but also the size of the window
impl Workspace {
    
    pub fn render_activity_panel(
        this: &Rc<Workspace>,
        width: impl Signal<Item = u32> + 'static,
        height: impl Signal<Item = u32> + 'static
    ) -> Dom {
        let width = width.broadcast();
        let height = height.broadcast();

        html!("div", {
            .class("col-span-1")
            .class("row-span-1")
            .class("grid")
            .style_signal("grid-template-columns", this.cols.signal_vec_cloned()
                .map(|col_type| match col_type {
                    ColumnType::Auto => "auto".to_string(),
                    ColumnType::Fr => "1fr".to_string()
                })
                .to_signal_cloned()
                .map(|columns| columns.join(" "))
            )
            .children_signal_vec(this.activity_panel_list.entries_cloned().map(clone!(this, width, height => move |(_, panel)| {
                ActivityPanel::render(&this, &panel, width.signal(), height.signal())
            })))
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

    pub fn render_console(this: &Rc<Workspace>) -> Dom {
        html!("div", {
            .class("col-span-1")
            .class("row-span-1")
            .style_signal("height", this.console_height.signal().map(|height| format!("{height}px")))
            .apply(styles::console::container)
            .child(this.console.render())
        })
    }
}
