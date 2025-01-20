use std::rc::Rc;

use dominator::{clone, events::{self, MouseButton}, html, svg, Dom, EventOptions, with_node};
use futures_signals::{signal::{self, Mutable, Signal, SignalExt}, signal_vec::SignalVecExt};

use crate::{contextmenu::{ContextMenu, Target}, vfs::Directory};

const ICON_SVG_PATH: &str =
    "M16 0H8C6.9 0 6 .9 6 2V18C6 19.1 6.9 20 8 20H20C21.1 20 22 19.1 22 \
     18V6L16 0M20 18H8V2H15V7H20V18M4 4V22H20V24H4C2.9 24 2 23.1 2 22V4H4Z";

thread_local! {
    static DRAGGED_ITEM: Mutable<Option<Target>> = Mutable::new(None);
    pub static RENAME: Mutable<Option<Target>> = Mutable::new(None);
}
     
fn folder_open_icon() -> Dom {
    // downward arrow
    const FOLDER_OPEN_ICON: &str = "M2,7 12,17 22,7Z";
    svg!("svg", {
        .attr("pointer-events", "none")
        .attr("height", "1em")
        .attr("viewBox", "0 0 27 27")
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
        .attr("viewBox", "0 0 27 27")
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

fn find_and_remove_from_parent(target: &Target, root: &Rc<Directory>) {
    match target {
        Target::File(file) => {
            // Search for the parent directory containing the file
            let mut files = root.files.lock_mut();
            if let Some(pos) = files.iter().position(|f| Rc::ptr_eq(f, file)) {
                files.remove(pos); // Remove the file
                return;
            }
        }
        Target::Directory(dragged_dir) => {
            // Search for the parent directory containing the directory
            let mut directories = root.directories.lock_mut();
            if let Some(pos) = directories.iter().position(|d| Rc::ptr_eq(d, dragged_dir)) {
                directories.remove(pos); // Remove the directory
                return;
            }
        }
    }

    // Recursively search child directories
    for child in root.directories.lock_ref().iter() {
        find_and_remove_from_parent(target, child);
    }
}

fn render_contents(
    directory: &Rc<Directory>,
    workspace_command_tx: &crate::WorkspaceCommandSender, 
    context_menu: Mutable<Option<ContextMenu>>
) -> Dom {
    let directories = directory.directories
        .signal_vec_cloned()
        .sort_by_cloned(|left_directory, right_directory|
            left_directory.name.lock_ref().cmp(&*right_directory.name.lock_ref()))
        .map(clone!(workspace_command_tx, context_menu => move |directory| {
            let expanded = Mutable::new(true);
            html!("li", {
                .class("pl-5")
                .class("pt-0")
                .attr("draggable", "true")
                .event(clone!(directory => move |_: events::DragStart| {
                    DRAGGED_ITEM.with(|dragged| {
                        dragged.set(Some(Target::Directory(directory.clone())));
                    })
                }))
                .event_with_options(&EventOptions::preventable(), |event: events::DragOver| {
                    event.prevent_default(); // Allow drop
                })
                .event_with_options(&EventOptions::preventable(), clone!(directory => move |event: events::Drop| {
                    event.prevent_default();
                    DRAGGED_ITEM.with(|dragged| {
                        if let Some(target) = dragged.get_cloned() {
                            crate::PROJECT.with(|root| {
                                // Remove the dragged item from its original parent
                                find_and_remove_from_parent(&target, &root);
                            });
                
                            // Add the dragged item to the target directory
                            match target {
                                Target::File(file) => directory.files.lock_mut().push_cloned(file),
                                Target::Directory(dragged_dir) => directory.directories.lock_mut().push_cloned(dragged_dir),
                            }
                        }
                    });
                }))
                .event(|_: events::DragEnd| {
                    DRAGGED_ITEM.with(|dragged| {
                        dragged.set(None);
                    });
                })
                .child(html!("div", {
                    .class("icon_text")
                    .class("grid")
                    .class("grid-cols-[auto_1fr]")
                    .class("p-[2px]")
                    .class("cursor-pointer")
                    .event(clone!(expanded => move |event: events::MouseDown| {
                        // left click to expand directory
                        let rename = RENAME.with(|rename| rename.get_cloned().is_some());
                        let is_drag_and_drop = DRAGGED_ITEM.with(|dragged| dragged.get_cloned().is_some());
                        if !rename && !is_drag_and_drop && event.button() == MouseButton::Left {
                            let mut expanded = expanded.lock_mut();
                            *expanded = !*expanded;
                        }
                    }))
                    .children(&mut [
                        html!("div", {
                            .class("icon")
                            .class("mr-0")
                            .child_signal(expanded.signal_ref(|expanded| match expanded {
                                true => folder_open_icon(),
                                false => folder_closed_icon(),
                            }.into()))
                        }),
                        html!("div", {
                            // input box for renaming
                            .child_signal(RENAME.with(|rename| rename.signal_cloned().map(clone!(directory => move |global_target| {
                                match global_target {
                                    Some(Target::Directory(ref dir)) if Rc::ptr_eq(dir, &directory) => {
                                        Some(html!("input" => web_sys::HtmlInputElement, {
                                            .class("input")
                                            .attr("type", "text")
                                            .attr("value", &*directory.name.get_cloned())
                                            .with_node!(element => {
                                                .event(clone!(directory => move |_: events::Input| {
                                                    directory.name.set(element.value());
                                                }))
                                                .event(|_: events::Blur| {
                                                    RENAME.with(|rename| rename.set(None));
                                                })
                                                .event(|event: events::KeyDown| {
                                                    if event.key() == "Enter" {
                                                        RENAME.with(|rename| rename.set(None));
                                                    }
                                                })
                                            })
                                        }))
                                    },
                                    _ => Some(html!("span", {
                                        .text_signal(directory.name.signal_cloned())
                                    })),
                                }
                            }))))
                        })
                    ])
                    // event listener for right click
                    .event(clone!(context_menu, directory => move |event: events::ContextMenu| {
                        web_sys::console::log_1(&"Right-clicked".into());
                        context_menu.set(Some(ContextMenu::new(
                            (event.x(), event.y()),
                            Target::Directory(directory.clone()),
                        )));
                    }))
                }))
                .child_signal(expanded.signal_ref(clone!(directory, workspace_command_tx, context_menu => move |expanded| {
                    expanded.then_some(render_contents(&directory, &workspace_command_tx, context_menu.clone()))
                })))
            })
        }));

    let files = directory.files
        .signal_vec_cloned()
        .sort_by_cloned(|left_file, right_file|
            left_file.name.lock_ref().cmp(&*right_file.name.lock_ref()))
        .map(clone!(workspace_command_tx => move |file| html!("li", {
            .class("pl-5")
            .class("pt-0")
            .attr("draggable", "true")
            .event(clone!(file => move |_: events::DragStart| {
                DRAGGED_ITEM.with(|dragged| {
                    dragged.set(Some(Target::File(file.clone())));
                })
            }))
            .event_with_options(&EventOptions::preventable(), |event: events::DragOver| {
                event.prevent_default(); // Allow drop
            })
            .event(|_: events::DragEnd| {
                DRAGGED_ITEM.with(|dragged| {
                    dragged.set(None);
                });
            })
            .child(html!("div", {
                .class("icon_text")
                .class("grid")
                .class("grid-cols-[auto_1fr]")
                .class("p-[2px]")
                .class("cursor-pointer")
                .event(clone!(workspace_command_tx, file => move |event: events::MouseDown| {
                    // left-click to open file in workspace
                    let rename = RENAME.with(|rename| rename.get_cloned().is_some());
                    let is_drag_and_drop = DRAGGED_ITEM.with(|dragged| dragged.get_cloned().is_some());
                    if !rename && !is_drag_and_drop && event.button() == MouseButton::Left {
                        workspace_command_tx
                            .unbounded_send(crate::WorkspaceCommand::OpenFile(file.clone()))
                            .unwrap()
                    }
                }))
                .children(&mut [
                    html!("div", {
                        .class("icon")
                        .class("mr-0")
                        .child(file_icon())
                    }),
                    html!("div", {
                        // input box for renaming
                        .child_signal(RENAME.with(|rename| rename.signal_cloned().map(clone!(file => move |target| {
                            match target {
                                Some(Target::File(ref fil)) if Rc::ptr_eq(fil, &file) => {
                                    Some(html!("input" => web_sys::HtmlInputElement, {
                                        .class("input")
                                        .attr("type", "text")
                                        .attr("value", &*file.name.get_cloned())
                                        .with_node!(element => {
                                            .event(clone!(file => move |_: events::Input| {
                                                file.name.set(element.value());
                                                element.focus().unwrap();
                                            }))
                                            .event(|_: events::Blur| {
                                                RENAME.with(|rename| rename.set(None));
                                            })
                                            .event(|event: events::KeyDown| {
                                                if event.key() == "Enter" {
                                                    RENAME.with(|rename| rename.set(None));
                                                }
                                            })
                                        })
                                    }))
                                },
                                _ => Some(html!("span", {
                                    .text_signal(file.name.signal_cloned())
                                })),
                            }
                        }))))
                    })
                ])
                // event listener for right click
                .event(clone!(context_menu => move |event: events::ContextMenu| {
                    web_sys::console::log_1(&"Right-clicked".into());
                    context_menu.set(Some(ContextMenu::new(
                        (event.x(), event.y()),
                        Target::File(file.clone())
                    )));
                }))
            }))
        })));

    html!("ul", {
        .children_signal_vec(directories)
        .children_signal_vec(files)
    })
}

pub struct Explorer {
    workspace: Rc<Directory>,
    // context menu
    context_menu: Mutable<Option<ContextMenu>>
}

impl Default for Explorer {
    fn default() -> Self {
        Self {
            workspace: crate::PROJECT.with(|workspace| Rc::clone(workspace)),
            context_menu: Mutable::new(None),
        }
    }
}

impl Explorer {
    pub fn render(this: &Rc<Explorer>, workspace_command_tx: &crate::WorkspaceCommandSender) -> dominator::Dom {
        let expanded = Mutable::new(true);
        html!("div", {
            .class("block")
            .class("bg-lightgray")
            .class("h-screen")
            .child(html!("div", {
                .class("block")
                .class("m-0")
                .class("h-[35px]")
                .child(html!("div", { 
                    .class("icon_text")
                    .class("pl-6")
                    .class("pt-2")
                    .child(html!("span", {
                        .class("text-darkgray")
                        .class("text-[0.70em]")
                        .class("tracking-tight")
                        .class("uppercase")
                        .text("Explorer")
                    }))
                }))
            }))
            // project listing
            .child(html!("ul", {
                .child(html!("li", {
                    .attr("draggable", "true")
                    .event_with_options(&EventOptions::preventable(), |event: events::DragOver| {
                        event.prevent_default(); // Allow drop
                    })
                    .event_with_options(&EventOptions::preventable(), clone!(this => move |event: events::Drop| {
                        event.prevent_default();
                        DRAGGED_ITEM.with(|dragged| {
                            if let Some(target) = dragged.get_cloned() {
                                crate::PROJECT.with(|root| {
                                    // Remove the dragged item from its original parent
                                    find_and_remove_from_parent(&target, &root);
                                });
                    
                                // Add the dragged item to the target directory
                                match target {
                                    Target::File(file) => this.workspace.files.lock_mut().push_cloned(file),
                                    Target::Directory(dragged_dir) => this.workspace.directories.lock_mut().push_cloned(dragged_dir),
                                }
                            }
                        });
                    }))
                    .event(|_: events::DragEnd| {
                        DRAGGED_ITEM.with(|dragged| {
                            dragged.set(None);
                        });
                    })
                    .child(html!("div", {
                        .class("icon_text")
                        .class("grid")
                        .class("grid-cols-[auto_1fr]")
                        .class("p-[2px]")
                        .class("cursor-pointer")
                        .event(clone!(expanded => move |event: events::MouseDown| {
                            // left-click to expand directory
                            let rename = RENAME.with(|rename| rename.get_cloned().is_some());
                            let is_drag_and_drop = DRAGGED_ITEM.with(|dragged| dragged.get_cloned().is_some());
                            if !rename && !is_drag_and_drop && event.button() == MouseButton::Left {
                                let mut expanded = expanded.lock_mut();
                                *expanded = !*expanded;
                            }
                        }))
                        .children(&mut [
                            html!("div", {
                                .class("mr-0")
                                .class("icon")
                                .child_signal(expanded.signal_ref(|expanded| match expanded {
                                    true => folder_open_icon(),
                                    false => folder_closed_icon(),
                                }.into()))
                            }),
                            html!("div", {
                                // input box for renaming
                                .child_signal(RENAME.with(|rename| rename.signal_cloned().map(clone!(this => move |target| {
                                    match target {
                                        Some(Target::Directory(ref dir)) if Rc::ptr_eq(dir, &this.workspace) => {
                                            Some(html!("input" => web_sys::HtmlInputElement, {
                                                .class("input")
                                                .attr("type", "text")
                                                .attr("value", &*this.workspace.name.get_cloned())
                                                .with_node!(element => {
                                                    .event(clone!(this => move |_: events::Input| {
                                                        this.workspace.name.set(element.value());
                                                    }))
                                                    .event(|_: events::Blur| {
                                                        RENAME.with(|rename| rename.set(None));
                                                    })
                                                    .event(|event: events::KeyDown| {
                                                        if event.key() == "Enter" {
                                                            RENAME.with(|rename| rename.set(None));
                                                        }
                                                    })
                                                })
                                            }))
                                        },
                                        _ => Some(html!("span", {
                                            .text_signal(this.workspace.name.signal_cloned())
                                        })),
                                    }
                                }))))
                            })
                        ])
                        // event listener for right click
                        .event(clone!(this => move |event: events::ContextMenu| {
                            web_sys::console::log_1(&"Right-clicked".into());
                            this.context_menu.set(Some(ContextMenu::new(
                                (event.x(), event.y()),
                                Target::Directory(this.workspace.clone()),
                            )));
                        }))
                    }))
                    // check for update in show to render context menu
                    .child_signal(this.context_menu.signal_ref(|menu_state| {
                        menu_state.as_ref().map(|menu| {
                            match &menu.target {
                                Target::Directory(_) => ContextMenu::folder_menu_render(menu),
                                Target::File(_) => ContextMenu::file_menu_render(menu),
                            }
                        })
                    }))
                    // prevents default chrome context menu for the whole vfs structure
                    .event_with_options(&EventOptions::preventable(), |event: events::ContextMenu| {
                        event.prevent_default();
                    })
                    // global event listener to close context menu
                    .global_event(clone!(this => move |_:events::Click| {
                        this.context_menu.set(None)
                    }))
                    .child_signal(expanded.signal_ref(clone!(this, workspace_command_tx => move |expanded| {
                        expanded.then_some(render_contents(&this.workspace, &workspace_command_tx, this.context_menu.clone()))
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
            .attr("viewBox", "0 0 27 27")
            .class_signal("fill-white", active.signal())
            .class_signal("fill-darkgray", signal::not(active.signal()))
            .child(svg!("path", {
                .attr("d", ICON_SVG_PATH)
            }))
        })
    }
}