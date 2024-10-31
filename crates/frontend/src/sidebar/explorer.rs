use std::rc::Rc;

use dominator::{clone, events::{self, MouseButton}, html, svg, Dom, EventOptions};
use dominator_bulma::{block, icon, icon_text};
use futures_signals::{signal::{self, Mutable, Signal, SignalExt}, signal_vec::SignalVecExt};

use crate::{contextmenu::{ContextMenu, Menutype}, vfs::Directory};

const ICON_SVG_PATH: &str =
    "M16 0H8C6.9 0 6 .9 6 2V18C6 19.1 6.9 20 8 20H20C21.1 20 22 19.1 22 \
     18V6L16 0M20 18H8V2H15V7H20V18M4 4V22H20V24H4C2.9 24 2 23.1 2 22V4H4Z";

fn folder_open_icon() -> Dom {
    // downward arrow
    const FOLDER_OPEN_ICON: &str = "M2,7 12,17 22,7Z";
    svg!("svg", {
        .attr("pointer-events", "none")
        .attr("height", "1em")
        .attr("viewBox", "0 0 24 24")
        .child(svg!("path", {
            .attr("d", FOLDER_OPEN_ICON)
        }))
    })
}

fn folder_closed_icon() -> Dom {
    // sideways arrow
    const FOLDER_CLOSED_ICON: &str = "M 7,22 17,12 7,2 Z";
    svg!("svg", {
        .attr("pointer-events", "none")
        .attr("height", "1em")
        .attr("viewBox", "0 0 24 24")
        .child(svg!("path", {
            .attr("d", FOLDER_CLOSED_ICON)
        }))
    })
}

fn file_icon() -> Dom {
    const FILE_ICON_PATH: &str = "M14,2H6A2,2 0 0,0 4,4V20A2,2 0 \
        0,0 6,22H18A2,2 0 0,0 20,20V8L14,2M18,20H6V4H13V9H18V20Z";
    svg!("svg", {
        .attr("pointer-events", "none")
        .attr("height", "1em")
        .attr("viewBox", "0 0 24 24")
        .child(svg!("path", {
            .attr("d", FILE_ICON_PATH)
        }))
    })
}

fn render_contents(
    directory: &Rc<Directory>,
    workspace_command_tx: &crate::WorkspaceCommandSender,
    context_menu_state: &Rc<ContextMenu>
) -> Dom {
    let cms_d = context_menu_state.clone();
    let cms_f = context_menu_state.clone();
    let directories = directory.directories
        .signal_vec_cloned()
        .sort_by_cloned(|left_directory, right_directory|
            left_directory.name.lock_ref().cmp(&*right_directory.name.lock_ref()))
        .map(clone!(workspace_command_tx => move |directory| {
            let expanded = Mutable::new(true);
            html!("li", {
                .class("pl-5")
                .class("pt-1")
                .child(icon_text!({
                    .style("cursor", "pointer")
                    .event(clone!(expanded => move |event: events::MouseDown| {
                        // left click to expand directory
                        if event.button() == MouseButton::Left {
                            let mut expanded = expanded.lock_mut();
                            *expanded = !*expanded;
                        }
                    }))
                    .child(icon!("mr-0", {
                        .child_signal(expanded.signal_ref(|expanded| match expanded {
                            true => folder_open_icon(),
                            false => folder_closed_icon(),
                        }.into()))
                    }))
                    .child(html!("span", {
                        .text_signal(directory.name.signal_cloned())
                    }))
                    // event listener for right click
                    .event(clone!(cms_d=> move |event: events::ContextMenu| {
                        web_sys::console::log_1(&"Right-clicked".into());
                        cms_d.show.set(true);
                        cms_d.position.set((event.x(), event.y()));
                        cms_d.menu_type.set(Some(Menutype::Directory));
                    }))
                }))
                // check for update in show to render context menu
                // not required since the parent is looking for signal (not very sure of the behavior/need to look in to this)
                .child_signal(cms_d.show.signal_ref(clone!(cms_d => move |&show| {
                    show.then_some(ContextMenu::render_menu(cms_d.clone()))
                })))
                .child_signal(expanded.signal_ref(clone!(directory, workspace_command_tx, cms_d => move |expanded| {
                    expanded.then_some(render_contents(&directory, &workspace_command_tx, &cms_d))
                })))
            })
        }));

    let files = directory.files
        .signal_vec_cloned()
        .sort_by_cloned(|left_file, right_file|
            left_file.name.lock_ref().cmp(&*right_file.name.lock_ref()))
        .map(clone!(workspace_command_tx => move |file| html!("li", {
            .class("pl-5")
            .class("pt-1")
            .child(icon_text!({
                .style("cursor", "pointer")
                .event(clone!(workspace_command_tx, file => move |event: events::MouseDown| {
                    // left-click to open file in workspace
                    if event.button() == MouseButton::Left {
                        workspace_command_tx
                            .unbounded_send(crate::WorkspaceCommand::OpenFile(file.clone()))
                            .unwrap()
                    }
                }))
                .child(icon!("mr-0", {
                    .child(file_icon())
                }))
                .child(html!("span", {
                    .text_signal(file.name.signal_cloned())
                }))
                // event listener for right click
                .event(clone!(cms_f => move |event: events::ContextMenu| {
                    web_sys::console::log_1(&"Right-clicked".into());
                    cms_f.show.set(true);
                    cms_f.position.set((event.x(), event.y()));
                    cms_f.menu_type.set(Some(Menutype::File));
                }))
            }))
            // check for update in show to render context menu
            // not required since the parent is looking for signal (not very sure of the behavior/need to look into this)
            .child_signal(cms_f.show.signal_ref(clone!(cms_f => move |&show| {
                show.then_some(ContextMenu::render_menu(cms_f.clone()))
            })))
        })));

    html!("ul", {
        .children_signal_vec(directories)
        .children_signal_vec(files)
    })
}

pub struct Explorer {
    workspace: Rc<Directory>,
    // context menu Rc
    context_menu_state: Rc<ContextMenu>
}

impl Default for Explorer {
    fn default() -> Self {
        Self {
            workspace: crate::PROJECT.with(|workspace| Rc::clone(workspace)),
            context_menu_state: ContextMenu::new()
        }
    }
}

impl Explorer {
    pub fn render(this: &Rc<Explorer>, workspace_command_tx: &crate::WorkspaceCommandSender) -> dominator::Dom {
        let expanded = Mutable::new(true);
        let context_menu_state = this.context_menu_state.clone();
        block!({
            .class("has-background-white-ter")
            .style("height", "100vh")
            .child(block!("p-3", "m-0", {
                .child(icon_text!({
                    .child(html!("span", {
                        .style("font-size", ".75em")
                        .style("letter-spacing", ".1em")
                        .style("text-transform", "uppercase")
                        .text("Explorer")
                    }))
                }))
            }))
            // project listing
            .child(html!("ul", {
                .child(html!("li", {
                    .class("pl-2")
                    .child(icon_text!({
                        .style("cursor", "pointer")
                        .event(clone!(expanded => move |event: events::MouseDown| {
                            // left-click to expand directory
                            if event.button() == MouseButton::Left {
                                let mut expanded = expanded.lock_mut();
                                *expanded = !*expanded;
                            }
                        }))
                        .child(icon!("mr-0", {
                            .child_signal(expanded.signal_ref(|expanded| match expanded {
                                true => folder_open_icon(),
                                false => folder_closed_icon(),
                            }.into()))
                        }))
                        .child(html!("span", {
                            .text_signal(this.workspace.name.signal_cloned())
                        }))
                        // event listener for right click
                        .event(clone!(context_menu_state=> move |event: events::ContextMenu| {
                            web_sys::console::log_1(&"Right-clicked".into());
                            context_menu_state.show.set(true);
                            context_menu_state.position.set((event.x(), event.y()));
                            context_menu_state.menu_type.set(Some(Menutype::Directory));
                        }))
                    }))
                    // check for update in show to render context menu
                    .child_signal(context_menu_state.show.signal_ref(clone!(context_menu_state => move |&show| {
                        show.then_some(ContextMenu::render_menu(context_menu_state.clone()))
                    })))
                    // prevents default chrome context menu for the whole vfs structure
                    .event_with_options(&EventOptions::preventable(), |event: events::ContextMenu| {
                        event.prevent_default();
                    })
                    // global event listener to close context menu
                    .global_event(clone!(context_menu_state => move |_:events::Click| {
                        context_menu_state.show.set(false);
                    }))
                    .child_signal(expanded.signal_ref(clone!(this, workspace_command_tx, context_menu_state => move |expanded| {
                        expanded.then_some(render_contents(&this.workspace, &workspace_command_tx, &context_menu_state))
                    })))
                }))
            }))
        })
    }

    pub fn tooltip(&self) -> &'static str {
        "Explorer"
    }

    pub fn icon(&self, active: impl Signal<Item = bool> + 'static) -> Dom {
        let active = active.broadcast();
        svg!("svg", {
            .attr("viewBox", "0 0 24 24")
            .class_signal("has-fill-white", active.signal())
            .class_signal("has-fill-grey", signal::not(active.signal()))
            .child(svg!("path", {
                .attr("d", ICON_SVG_PATH)
            }))
        })
    }
}