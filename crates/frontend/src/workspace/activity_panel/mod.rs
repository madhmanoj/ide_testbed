use std::{pin::Pin, rc::Rc};

use dominator::{clone, events::{self, MouseButton}, html, svg, Dom, EventOptions};
use futures::{channel::mpsc::{self, UnboundedSender}, StreamExt};
use futures_signals::{signal::{Mutable, Signal, SignalExt}, signal_vec::{MutableVec, SignalVecExt}};

use crate::{styles, vfs};
use crate::contextmenu::TabMenu;
use super::Workspace;

pub mod editor;
pub mod welcome;

const TAB_HEIGHT: u32 = 35;

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
        width: impl Signal<Item = u32> + 'static,
        height: impl Signal<Item = u32> + 'static
    ) -> Pin<Box<dyn Signal<Item = Option<dominator::Dom>>>> {
        match this.as_ref() {
            Activity::Editor(editor) => Box::pin(editor::Editor::render(editor, width, height)),
            Activity::Welcome(welcome) => Box::pin(welcome::Welcome::render(welcome, width, height)),
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
        workspace: &Rc<Workspace>,
        this: &Rc<Activity>,
        panel: &Rc<ActivityPanel>
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
        let is_active = panel.active_activity.signal_ref(clone!(this => move |active_activity| {
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
            .event(clone!(panel, this => move |_: events::PointerDown| {                
                panel.active_activity.set(Some(this.clone()))
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
                        .event_with_options(&EventOptions::preventable(), clone!(panel, this => move |ev: events::PointerDown| {
                            ev.stop_propagation();
                            panel.activities.lock_mut().retain(|activity| !Rc::ptr_eq(activity, &this));
                            let mut active_activity = panel.active_activity.lock_mut();
                            if active_activity.as_ref().is_some_and(|active_activity| Rc::ptr_eq(active_activity, &this)) {
                                // simple logic, VS Code is smart and keeps track of the last tab you looked at
                                *active_activity = panel.activities.lock_ref().first().cloned();
                            }
                        }))
                        .child(close_icon)
                    }))
                })
            }))
            // rendering tab menu
            .child_signal(tab_menu.signal_ref(clone!(workspace, panel => move |menu_state| {
                menu_state.as_ref().map(clone!(workspace, panel => move |menu| {
                    if let Some(activity) = panel.active_activity.lock_ref().as_ref() {
                        TabMenu::render(menu, &workspace, activity)
                    }  else {
                        Dom::empty()
                    }        
                }))
            })))
            // event handler for tab context menu
            .event(clone!(tab_menu => move |event: events::ContextMenu| {
                tab_menu.set(Some(TabMenu::new(
                    (event.x(), event.y())
                )));
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
    activities: MutableVec<Rc<Activity>>,
    active_activity: Mutable<Option<Rc<Activity>>>,
    pub activity_panel_tx: UnboundedSender<ActivityPanelCommand>
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
        let (tx, rx) = mpsc::unbounded();
        let panel = Rc::new(Self {
            activities: vec![welcome.clone()].into(),
            active_activity: Some(welcome).into(),
            activity_panel_tx: tx
        });

        wasm_bindgen_futures::spawn_local(rx.for_each(clone!(panel => move |command| clone!(panel => async move {
            match command {
                ActivityPanelCommand::OpenFile(file) => {
                    let mut activities = panel.activities.lock_mut();
                    let editor = activities.iter()
                        .find(|activity| match &***activity {
                            Activity::Editor(editor) => Rc::ptr_eq(&editor.file, &file),
                            _ => false,
                        })
                        .cloned()
                        .unwrap_or_else(move || {
                            let editor = Rc::new(Activity::Editor(Rc::new(editor::Editor::new(file))));
                            activities.push_cloned(editor.clone());
                            editor
                        });
                    panel.active_activity.set(Some(editor));
                },
            }
        }))));
        panel
    }

    pub fn new(activity: &Rc<Activity>) -> Rc<Self> {
        let (tx, rx) = mpsc::unbounded();
        let panel = Rc::new(Self {
            activities: vec![activity.clone()].into(),
            active_activity: Some(activity.clone()).into(),
            activity_panel_tx: tx
        });

        wasm_bindgen_futures::spawn_local(rx.for_each(clone!(panel => move |command| clone!(panel => async move {
            match command {
                ActivityPanelCommand::OpenFile(file) => {
                    let mut activities = panel.activities.lock_mut();
                    let editor = activities.iter()
                        .find(|activity| match &***activity {
                            Activity::Editor(editor) => Rc::ptr_eq(&editor.file, &file),
                            _ => false,
                        })
                        .cloned()
                        .unwrap_or_else(move || {
                            let editor = Rc::new(Activity::Editor(Rc::new(editor::Editor::new(file))));
                            activities.push_cloned(editor.clone());
                            editor
                        });
                    panel.active_activity.set(Some(editor));
                },
            }
        }))));
        panel
    }

    pub fn render(
        workspace: &Rc<Workspace>,
        this: &Rc<ActivityPanel>,
        width: impl Signal<Item = u32> + 'static,
        height: impl Signal<Item = u32> + 'static
    ) -> dominator::Dom {

        let activity_count = this.activities.signal_vec_cloned().len().broadcast();
        let width = width.broadcast();
        let height = height.broadcast();

        html!("div", {
            .class("col-span-1")
            .class("grid")
            .class("grid-rows-[auto_1fr]")
            .class("h-full")

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
                .children_signal_vec(this.activities.signal_vec_cloned().map(clone!(this, workspace => move |activity| {
                    html!("div", {
                        .class("h-full")
                        .child(Activity::render_tab(&workspace, &activity, &this))
                    })
                })))
            }))
            .child_signal(this.active_activity
                .signal_cloned()
                .map(move |activity: Option<Rc<Activity>>| activity
                    .map(clone!(width, height => move |activity| html!("div", {
                        .class("h-full")
                        .child_signal(Activity::render(
                            &activity,
                            width.signal(),
                            height.signal_ref(|height| height.saturating_sub(TAB_HEIGHT))))
                    })))
                )
            )
        })
    }

    fn render_background(
        height: impl Signal<Item = u32> + 'static
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
