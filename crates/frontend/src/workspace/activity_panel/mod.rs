use std::{pin::Pin, rc::Rc};

use dominator::{clone, events::{self, MouseButton}, html, svg, Dom, EventOptions};
use futures::{channel::mpsc::UnboundedSender, FutureExt};
use futures_signals::{signal::{Mutable, Signal, SignalExt}, signal_vec::{MutableVec, SignalVecExt}};
use crate::{styles, vfs};
use crate::contextmenu::TabMenu;

use super::panel::LayoutPanel;

pub mod editor;
pub mod welcome;

//const TAB_HEIGHT: u32 = 35;

pub enum ActivityPanelCommand {
    OpenFile(Rc<vfs::File>)
}

pub enum Activity {
    Editor(Rc<editor::Editor>),
    Welcome(Rc<welcome::Welcome>),
}

impl Activity {
    pub fn render(
        this: &Rc<Activity>,
        // width: impl Signal<Item = u32> + 'static,
        // height: impl Signal<Item = u32> + 'static
    ) -> Pin<Box<dyn Signal<Item = Option<dominator::Dom>>>> {
        match this.as_ref() {
            Activity::Editor(editor) => Box::pin(editor::Editor::render(editor)),
            Activity::Welcome(welcome) => Box::pin(welcome::Welcome::render(welcome)),
        }
    }

    pub fn label(&self) -> Dom {
        match self {
            Activity::Editor(editor) => editor.label(),
            Activity::Welcome(welcome) => welcome.label(),
        }
    }

    pub fn icon(&self) -> Dom {
        match self {
            Activity::Editor(editor) => editor.icon(),
            Activity::Welcome(welcome) => welcome.icon(),
        }
    }

    fn render_tab(
        panel: &Rc<LayoutPanel>,
        this: &Rc<Activity>,
        activity_panel: &Rc<ActivityPanel>
    ) -> dominator::Dom {
        let close_icon = svg!("svg", {
            .attr("height", "1em")
            .attr("viewBox", "0 0 24 24")
            .child(svg!("path", {
                .attr("d", CLOSE_ICON_PATH)
            }))
        });

        let mouse_over = Mutable::new(false);
        let mouse_over_close = Mutable::new(false);
        let is_active = activity_panel.active_activity.signal_ref(clone!(this => move |active_activity| {
            active_activity.as_ref().is_some_and(|active_activity| Rc::ptr_eq(active_activity, &this))
        }));
        let tab_menu: Mutable<Option<TabMenu>> = Mutable::new(None);

        html!("div", {
            .class("block")
            .class("h-full")
            .apply(|dom| styles::tab::body(dom, is_active, mouse_over.signal()))
            .event(clone!(mouse_over => move |_: events::PointerOver| {
                mouse_over.set_neq(true);
            }))
            .event(clone!(mouse_over => move |_: events::PointerOut| {
                mouse_over.set_neq(false);
            }))
            .event(clone!(activity_panel, this => move |_: events::PointerDown| {                
                activity_panel.active_activity.set(Some(this.clone()))
            }))
            .child(html!("div", {
                .apply(styles::icon_text)
                .child(html!("div", {
                    .apply(styles::icon)
                    .child(this.icon())
                }))
                .child(this.label())
                // HACK DO NOT SHOW THE CLOSE ICON 
                .apply_if(matches!(**this, Activity::Editor(_)), |dom| {
                    dom.child(html!("div", {
                        .apply(|dom| styles::tab::icon(dom, mouse_over_close.signal(), mouse_over.signal()))
                        .event(clone!(mouse_over_close => move |_: events::PointerOver| {
                            mouse_over_close.set_neq(true);
                        }))
                        .event(clone!(mouse_over_close => move |_: events::PointerOut| {
                            mouse_over_close.set_neq(false);
                        }))
                        .event_with_options(&EventOptions::preventable(), clone!(activity_panel, this => move |ev: events::PointerDown| {
                            ev.stop_propagation();
                            activity_panel.activities.lock_mut().retain(|activity| !Rc::ptr_eq(activity, &this));
                            let mut active_activity = activity_panel.active_activity.lock_mut();
                            if active_activity.as_ref().is_some_and(|active_activity| Rc::ptr_eq(active_activity, &this)) {
                                // simple logic, VS Code is smart and keeps track of the last tab you looked at
                                *active_activity = activity_panel.activities.lock_ref().first().cloned();
                            }
                        }))
                        .child(close_icon)
                    }))
                })
            }))
            // rendering tab menu
            .child_signal(tab_menu.signal_ref(|menu| {
                menu.as_ref().map(|menu| menu.render())
            }))
            // event handler for tab context menu
            .event(clone!(tab_menu, panel => move |event: events::ContextMenu| {
                tab_menu.set(Some(TabMenu { 
                    position: (event.x(), event.y()), 
                    panel: panel.clone() 
                }));
            }))
            // prevents default chrome context menu for the the tab bar
            .event_with_options(&EventOptions::preventable(), |event: events::ContextMenu| {
                event.prevent_default();
            })
            // global event listener to close tab menu if context menu is opened
            .global_event(clone!(tab_menu => move |event: events::MouseDown| {
                if event.button() == MouseButton::Right {
                    tab_menu.set(None)
                }
            }))
            // global event listener to close tab menu
            .global_event(clone!(tab_menu => move |_: events::Click| {
                tab_menu.set(None);
            }))
        })
        
    }
}

pub struct ActivityPanel {
    pub activities: MutableVec<Rc<Activity>>,
    pub active_activity: Mutable<Option<Rc<Activity>>>,
    // we set it during rendering, dont know how this is but it allows us to get rid of the wasm_bindgen::spawnlocal
    pub activity_panel_tx: Mutable<Option<UnboundedSender<ActivityPanelCommand>>>
}

// clicking a file in the explorer opens the file in the editor
// perhaps we just have a channel over which we send mutables? such that content can be synchronised
// how do I determine if a file is already open? Files should be uniquely identifiable from their
// paths

const CLOSE_ICON_PATH: &str = "M19,6.41L17.59,5L12,10.59L6.41,5L5,6.41L10.59,12L5,17.59L6.41,19L12,13.41L17.59,19L19,17.59L13.41,12L19,6.41Z";
//const CHANGED_ICON_PATH: &str = "M12,2A10,10 0 0,0 2,12A10,10 0 0,0 12,22A10,10 0 0,0 22,12A10,10 0 0,0 12,2Z";

impl ActivityPanel {

    pub fn default() -> Rc<Self> {
        let welcome = Rc::new(Activity::Welcome(Rc::new(welcome::Welcome::new())));
        Rc::new(Self {
            activities: vec![welcome.clone()].into(),
            active_activity: Some(welcome).into(),
            activity_panel_tx: Mutable::new(None)
        })
    }

    pub fn new(activity: &Rc<Activity>) -> Rc<Self> {
        Rc::new(Self {
            activities: vec![activity.clone()].into(),
            active_activity: Some(activity.clone()).into(),
            activity_panel_tx: Mutable::new(None)
        })
    }

    pub fn render(
        panel: &Rc<LayoutPanel>,
        this: &Rc<ActivityPanel>,
        //width: impl Signal<Item = f64> + 'static,
        height: impl Signal<Item = f64> + 'static
    ) -> dominator::Dom {

        let activity_count = this.activities.signal_vec_cloned().len().broadcast();
        //let width = width.broadcast();
        let height = height.broadcast();

        html!("div", {
            .class("h-full")
            .class("w-full")
            .class("col-span-1")
            .class("grid")
            .class("grid-rows-[auto_1fr]")
            .class("overflow-x-scroll")
            .future(activity_count.signal().wait_for(0).map(clone!(panel => move |_| {
                if let LayoutPanel::Widget { parent: Some(parent), .. } = panel.as_ref() {
                    match parent.as_ref() {
                        LayoutPanel::HorizontalSplit { children, parent: grand_parent } | LayoutPanel::VerticalSplit { children, parent: grand_parent } => {
                            // index should always be a Some value, since self should always be present in children mutablevec of its parent
                            // otherwise panic since the panel is invalid
                            let mut parent_children = children.lock_mut();
                            let index = parent_children.iter().position(|(child, _)| {
                                Rc::ptr_eq(child, &panel)
                            }).unwrap();
                            
                            // remove widget
                            parent_children.remove(index);

                            // check if parent has only one child left, if yes, clean up the panel layout
                            if parent_children.len() == 1 {
                                // there is only 1 element according to the if condition
                                let (only_child, _) = parent_children.first().unwrap();

                                match only_child.as_ref() {
                                    // for cases where the only child is a split, the logic is to promote all the panels of the child to the mutablevec 
                                    // of the panel's parent
                                    // this makes sense because in the nesting structure according to our implementation, there can only ever be a 
                                    // horizontal split inside a vertical split and vice versa, and if we directly promote the split panel, we will have a 
                                    // vertical split inside a vertical split (similarly for horizontal split) which is redundant
                                    LayoutPanel::HorizontalSplit { children: only_child_children, .. }
                                    | LayoutPanel::VerticalSplit { children: only_child_children, .. } => {
                                        if let Some(grand_parent) = grand_parent {
                                            match grand_parent.as_ref() {
                                                LayoutPanel::HorizontalSplit { children: grand_parent_children, .. }
                                                | LayoutPanel::VerticalSplit { children: grand_parent_children, .. } => {
                                                    let mut grand_parent_children = grand_parent_children.lock_mut();
                                                    if let Some(index) = grand_parent_children.iter().position(|(child, _)| Rc::ptr_eq(child, parent)) {
                                                        for (i, (panel, size)) in only_child_children.lock_ref().iter().enumerate() {
                                                            let new_child = match panel.as_ref() {
                                                                LayoutPanel::HorizontalSplit { children, .. } => Rc::new(LayoutPanel::HorizontalSplit { 
                                                                    parent: Some(grand_parent.clone()), 
                                                                    children: children.clone() 
                                                                }),
                                                                LayoutPanel::VerticalSplit { children, .. } => Rc::new(LayoutPanel::VerticalSplit { 
                                                                    parent: Some(grand_parent.clone()), 
                                                                    children: children.clone()
                                                                }),
                                                                LayoutPanel::Widget { activity_panel, .. } => Rc::new(LayoutPanel::Widget { 
                                                                    parent: Some(grand_parent.clone()), 
                                                                    activity_panel: activity_panel.clone()
                                                                })
                                                            };
                                                            let size = size.get();
                                                            if i == 0 {
                                                                grand_parent_children.set_cloned(index + i, (new_child.clone(), Mutable::new(size)));
                                                            } else {
                                                                grand_parent_children.insert_cloned(index + i, (new_child.clone(), Mutable::new(size)));
                                                            }
                                                        }
                                                    }
                                                },
                                                _ => {}
                                            }
                                        }  
                                    },
                                    // for cases where only_child is a widget, promote widget to the grand_parent directly if its the only panel left in the parent after removal of the original widget
                                    LayoutPanel::Widget { activity_panel, .. } => {
                                        let new_widget = Rc::new(LayoutPanel::Widget { 
                                            parent: grand_parent.clone(), 
                                            activity_panel: activity_panel.clone()
                                        });

                                        if let Some(grand_parent) = grand_parent {
                                            match grand_parent.as_ref() {
                                                LayoutPanel::HorizontalSplit { children: grand_parent_children, .. } 
                                                | LayoutPanel::VerticalSplit { children: grand_parent_children, .. } => {
                                                    let mut grand_parent_children = grand_parent_children.lock_mut();
                                                    if let Some(index) = grand_parent_children.iter().position(|(child, _)| Rc::ptr_eq(child, parent)) {
                                                        let size = grand_parent_children[index].1.get();
                                                        grand_parent_children.set_cloned(index, (new_widget, Mutable::new(size)));
                                                    }
                                                },
                                                _ => {}
                                            }
                                        }
                                    },
                                }

                            }
                        },
                        _ => {}
                    }
                }
            })))
            // this takes up the full height but should only display when there are no activities
            // and hence no tab bar
            .child_signal(activity_count.signal().map(clone!(height => move |count| {
                (count == 0).then(|| Self::render_background(height.signal()))
            })))
            // tabs take up one full line
            .child(html!("div", {
                .class("inline-flex")
                .class("h-[35px]")
                .apply(styles::tab::bar)
                .children_signal_vec(this.activities.signal_vec_cloned().map(clone!(this, panel => move |activity| {
                    html!("div", {
                        .class("h-full")
                        .child(Activity::render_tab(&panel, &activity, &this))
                    })
                })))
            }))
            .child_signal(this.active_activity
                .signal_cloned()
                .map(move |activity: Option<Rc<Activity>>| activity
                    .map(|activity| html!("div", {
                        .class("h-full")
                        .child_signal(Activity::render(
                            &activity,
                            // width.signal(),
                            // height.signal_ref(|height| height.saturating_sub(TAB_HEIGHT + 17))
                        ))
                    }))
                )
            )
        })
    }

    fn render_background(
        height: impl Signal<Item = f64> + 'static
    ) -> Dom {
        html!("div", {
            .style_signal("height", height.map(|height| format!("{height}px")))
            .style("background-image", "url('images/background.png')")
            .style("background-repeat", "no-repeat")
            .style("background-position", "center")
            .style("background-size", "auto 40%")
        })
    }
}
