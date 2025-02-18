use std::rc::Rc;

use activity_panel::ActivityPanel;
use dominator::{clone, events, html, Dom, EventOptions};
use futures_signals::{map_ref, signal::{Mutable, Signal, SignalExt}, signal_vec::{MutableVec, SignalVecExt}};
use uuid::Uuid;
use crate::styles;

pub mod console;
pub mod activity_panel;

const DEFAULT_CONSOLE_HEIGHT: u32 = 200;
const RESIZER_PX: u32 = 3;

#[derive(Clone)]
pub enum GridPanel {
    Panel(Rc<ActivityPanel>),
    Resizer
}

#[derive(Clone)]
pub enum ColumnType {
    Auto,
    Fr
}

pub struct Workspace {
    pub activity_panel_list: MutableVec<(Uuid, GridPanel)>,
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
            activity_panel_list: MutableVec::new_with_values(vec![
                (uuid, GridPanel::Panel(ActivityPanel::default()))
            ]),
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

        // vector to hold the widths of panels
        let panel_widths: MutableVec<Mutable<i32>> = MutableVec::new();

        // to track changes in the width of window and number of activity panels so that panels are split widthwise equally
        let full_width_signal = map_ref! {
            let total_width = width.signal(),
            let num_panels = this.activity_panel_list.signal_vec_cloned().len() => {
                let num_panels = *num_panels as u32;
                let total_width = total_width - (((num_panels - 1) / 2) * 3);
                total_width / ((num_panels + 1) / 2)
            }
        };

        // to set the widths of new panels 
        let temp = Mutable::new(0 as i32);

        html!("div", {
            .class("col-span-1")
            .class("row-span-1")  
            .class("grid")
            // future to track changes in width of individual panels based on changes in window or dropping panels
            // ISSUE: it automatically resizes everytime you drop a panel or resize the window
            .future(full_width_signal.for_each(clone!(panel_widths, temp => move |full_width| clone!(panel_widths, temp => async move {
                temp.set(full_width as i32);
                for i in panel_widths.lock_mut().iter() {
                    i.set(full_width as i32);
                }
            }))))
            .style("overflow", "hidden")
            .style_signal("width", width.signal().map(|width| format!("{}px", width)))
            .style_signal("height", height.signal().map(|height| format!("{}px", height)))
            .style_signal("grid-template-columns", this.cols.signal_vec_cloned()
                .map(|col_type| match col_type {
                    ColumnType::Auto => "3px".to_string(),
                    ColumnType::Fr => "1fr".to_string()
                })
                .to_signal_cloned()
                .map(|columns| columns.join(" "))
            )
            .children_signal_vec(this.activity_panel_list.signal_vec_cloned().enumerate().map(clone!(this, width, height => move |(index, (uuid, panel))| {
                let index = index.get().unwrap();
                match panel {
                    GridPanel::Panel(panel) => {
                        let index = index / 2;
                        let w = Mutable::new(temp.get());
                        panel_widths.lock_mut().insert_cloned(index, w.clone());
                        ActivityPanel::render(&this, &panel, &uuid, w.clone(), width.signal(), height.signal())
                    },
                    GridPanel::Resizer => {
                        horizontal_resizer(&this, &uuid, panel_widths.clone())   
                    }
                }
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

pub fn horizontal_resizer(
    workspace: &Rc<Workspace>,
    uuid: &Uuid,
    panel_widths: MutableVec<Mutable<i32>>
) -> Dom {
    let resize_active = Mutable::new(false);
    let resizer_hover = Mutable::new(false);
    let initial_position = Mutable::new(0 as i32);

    html!("div", {
        .class("cursor-ew-resize")
        .apply(|dom| styles::vertical_resizer(dom, resize_active.signal(), resizer_hover.signal()))
        .event_with_options(&EventOptions::preventable(), clone!(resize_active, initial_position, uuid => move |event: events::PointerDown| {
            web_sys::console::log_1(&format!("{uuid}").into());
            initial_position.set(event.x());
            resize_active.set_neq(true);
            event.prevent_default();
        }))
        .global_event(clone!(resize_active => move |_: events::PointerUp| {
            resize_active.set_neq(false);
        }))
        .event(clone!(resizer_hover => move |_: events::PointerEnter| {
            resizer_hover.set_neq(true);
        }))
        .event(clone!(resizer_hover => move |_: events::PointerLeave| {
            resizer_hover.set_neq(false);
        }))
        .global_event(clone!(resize_active, initial_position, panel_widths, workspace, uuid => move |event:events::PointerMove| {
            if resize_active.get() {
                let index = workspace.activity_panel_list
                    .lock_ref()
                    .iter()
                    .position(|(target_uuid, panel)| matches!(panel, GridPanel::Resizer) && *target_uuid == uuid)
                    .unwrap();

                let left_index = (index - 1) / 2; // Get left panel index
                let right_index = left_index + 1; // Ensure correct right panel index

                let panel_width_lock = panel_widths.lock_ref();
                
                let left_panel_width = panel_width_lock.get(left_index).unwrap();
                let right_panel_width = panel_width_lock.get(right_index).unwrap();

                let offset = event.x() - initial_position.get();

                // hack to prevent the panel from not allowing resizing when it reaches min width (100px)
                if left_panel_width.get() <= 100 {
                    let relative_offset = (100 - left_panel_width.get()) + 1;
                    left_panel_width.set(left_panel_width.get() + relative_offset);
                    right_panel_width.set(right_panel_width.get() - relative_offset);
                }

                // hack to prevent the panel from not allowing resizing when it reaches min width (100px)
                if right_panel_width.get() <= 100 {
                    let relative_offset = (100 - right_panel_width.get()) + 1;
                    right_panel_width.set(right_panel_width.get() + relative_offset);
                    left_panel_width.set(left_panel_width.get() - relative_offset);
                }

                // main resizing logic
                if offset != 0 && left_panel_width.get() > 100 && right_panel_width.get() > 100 {
                    left_panel_width.set(left_panel_width.get() + offset);
                    right_panel_width.set(right_panel_width.get() - offset);

                    initial_position.set(event.x());
                }
            }
        }))
    })
}