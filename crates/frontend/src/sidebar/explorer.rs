use std::{future::Future, rc::Rc};

use dominator::{clone, events::{self, MouseButton}, html, svg, Dom, EventOptions, with_node};
use either::Either;
use futures::{stream::FuturesUnordered, FutureExt, StreamExt};
use futures_signals::{signal::{Mutable, Signal, SignalExt}, signal_vec::SignalVecExt};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{FileSystemDirectoryHandle, FileSystemFileHandle, FileSystemHandle, FileSystemHandleKind};

use crate::{contextmenu::{ContextMenu, Target}, styles, vfs::{self, Directory}};

const ICON_SVG_PATH: &str =
    "M16 0H8C6.9 0 6 .9 6 2V18C6 19.1 6.9 20 8 20H20C21.1 20 22 19.1 22 \
     18V6L16 0M20 18H8V2H15V7H20V18M4 4V22H20V24H4C2.9 24 2 23.1 2 22V4H4Z";
     
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

fn render_contents(
    explorer: &Rc<Explorer>,
    parent_directory: &Rc<Directory>,
    workspace_command_tx: &crate::WorkspaceCommandSender
) -> Dom {
    let directories = parent_directory.directories
        .signal_vec_cloned()
        .sort_by_cloned(|left_directory, right_directory|
            left_directory.name.lock_ref().cmp(&*right_directory.name.lock_ref()))
        .map(clone!(workspace_command_tx, explorer, parent_directory => move |directory| {
            let expanded = Mutable::new(true);
            html!("li", {
                .apply(styles::vfs_item::list)
                .attr("draggable", "true")
                .event(clone!(directory, explorer, parent_directory => move |_: events::DragStart| {
                    explorer.drag_object.set(Some((Target::Directory(directory.clone()), parent_directory.clone())));
                }))
                .event(clone!(directory, explorer => move |_: events::DragEnter| {
                    explorer.drop_target.set(Some(directory.clone()));
                }))
                .child(html!("div", {
                    .apply(styles::vfs_item::body)
                    .event(clone!(expanded, explorer => move |event: events::Click| {
                        // left click to expand directory
                        let rename_active = explorer.rename.get_cloned().is_none();
                        if rename_active && event.button() == MouseButton::Left {
                            let mut expanded = expanded.lock_mut();
                            *expanded = !*expanded;
                        }
                    }))
                    .children(&mut [
                        html!("div", {
                            .apply(styles::vfs_item::icon)
                            .child_signal(expanded.signal_ref(|expanded| match expanded {
                                true => folder_open_icon(),
                                false => folder_closed_icon(),
                            }.into()))
                        }),
                        html!("div", {
                            // input box for renaming
                            .child_signal(explorer.rename.signal_cloned().map(clone!(directory, explorer => move |global_target| {
                                match global_target {
                                    Some(Target::Directory(ref dir)) if Rc::ptr_eq(dir, &directory) => {
                                        Some(html!("input" => web_sys::HtmlInputElement, {
                                            .apply(styles::input)
                                            .attr("type", "text")
                                            .attr("value", &*directory.name.get_cloned())
                                            .with_node!(element => {
                                                .event(clone!(directory => move |_: events::Input| {
                                                    directory.name.set(element.value());
                                                }))
                                                .event(clone!(explorer => move |_: events::Blur| {
                                                    explorer.rename.set(None);
                                                }))
                                                .event(clone!(explorer => move |event: events::KeyDown| {
                                                    if event.key() == "Enter" {
                                                        explorer.rename.set(None);
                                                    }
                                                }))
                                            })
                                        }))
                                    },
                                    _ => Some(html!("span", {
                                        .text_signal(directory.name.signal_cloned())
                                    })),
                                }
                            })))
                        })
                    ])
                    // event listener for right click
                    .event(clone!(explorer, directory, expanded => move |event: events::ContextMenu| {
                        expanded.set(true);
                        explorer.context_menu.set(Some(ContextMenu::new(
                            (event.x(), event.y()),
                            Target::Directory(directory.clone()),
                        )));
                    }))
                }))
                .child_signal(expanded.signal_ref(clone!(explorer, directory, workspace_command_tx => move |expanded| {
                    expanded.then_some(render_contents(&explorer, &directory, &workspace_command_tx))
                })))
            })
        }));

    let files = parent_directory.files
        .signal_vec_cloned()
        .sort_by_cloned(|left_file, right_file|
            left_file.name.lock_ref().cmp(&*right_file.name.lock_ref()))
        .map(clone!(workspace_command_tx, parent_directory, explorer => move |file| html!("li", {
            .apply(styles::vfs_item::list)
            .attr("draggable", "true")
            .event(clone!(file, explorer, parent_directory => move |_: events::DragStart| {
                explorer.drag_object.set(Some((Target::File(file.clone()), parent_directory.clone())));
            }))
            .child(html!("div", {
                .apply(styles::vfs_item::body)
                .event(clone!(workspace_command_tx, file, explorer => move |event: events::Click| {
                    // left-click to open file in workspace
                    let rename_active = explorer.rename.get_cloned().is_none();
                    if rename_active && event.button() == MouseButton::Left {
                        workspace_command_tx
                            .unbounded_send(crate::WorkspaceCommand::OpenFile(file.clone()))
                            .unwrap()
                    }
                }))
                .children(&mut [
                    html!("div", {
                        .apply(styles::vfs_item::icon)
                        .child(file_icon())
                    }),
                    html!("div", {
                        // input box for renaming
                        .child_signal(explorer.rename.signal_cloned().map(clone!(file, explorer => move |target| {
                            match target {
                                Some(Target::File(ref fil)) if Rc::ptr_eq(fil, &file) => {
                                    Some(html!("input" => web_sys::HtmlInputElement, {
                                        .apply(styles::input)
                                        .attr("type", "text")
                                        .attr("value", &*file.name.get_cloned())
                                        .with_node!(element => {
                                            .event(clone!(file => move |_: events::Input| {
                                                file.name.set(element.value());
                                                element.focus().unwrap();
                                            }))
                                            .event(clone!(explorer => move |_: events::Blur| {
                                                explorer.rename.set(None);
                                            }))
                                            .event(clone!(explorer => move |event: events::KeyDown| {
                                                if event.key() == "Enter" {
                                                    explorer.rename.set(None);
                                                }
                                            }))
                                        })
                                    }))
                                },
                                _ => Some(html!("span", {
                                    .text_signal(file.name.signal_cloned())
                                })),
                            }
                        })))
                    })
                ])
                // event listener for right click
                .event(clone!(explorer => move |event: events::ContextMenu| {
                    explorer.context_menu.set(Some(ContextMenu::new(
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
    context_menu: Mutable<Option<ContextMenu>>,
    // (dragged_object, dragged_object_parent)
    drag_object: Mutable<Option<(Target, Rc<Directory>)>>,
    rename: Mutable<Option<Target>>,
    drop_target: Mutable<Option<Rc<Directory>>>
}

impl Default for Explorer {
    fn default() -> Self {
        Self {
            workspace: crate::PROJECT.with(|workspace| Rc::clone(workspace)),
            context_menu: Mutable::new(None),
            drag_object: Mutable::new(None),
            rename: Mutable::new(None),
            drop_target: Mutable::new(None)
        }
    }
}

use util::StreamTools;

fn handle_drop(
    event: events::Drop
) -> impl Future<Output = (Vec<vfs::File>, Vec<vfs::Directory>)> {
    event.prevent_default();
    event.data_transfer()
        .into_iter()
        .flat_map(|transfer| {
            let items = transfer.items();
            (0..items.length()).flat_map(move |index| items.get(index).into_iter())
        })
        .filter_map(|item| match item.kind().as_ref() {
            "file" => JsFuture::from(item.get_as_file_system_handle()).into(),
            _ => None
        })
        .collect::<FuturesUnordered<_>>()
        .filter_map(|handle| async {
            // this currently throws away errors
            let handle = handle.ok()?
                .unchecked_into::<FileSystemHandle>();
            match handle.kind() {
                FileSystemHandleKind::File =>
                    vfs::File::from_handle(handle.unchecked_ref::<FileSystemFileHandle>())
                        .map(Either::Left)
                        .map(Some).await,
                FileSystemHandleKind::Directory =>
                    vfs::Directory::from_handle(handle.unchecked_ref::<FileSystemDirectoryHandle>())
                        .map(Either::Right)
                        .map(Some).await,
                _ => unreachable!()
            }
        })
        .partition_map(|entry| entry)
}

impl Explorer {
    pub fn render(this: &Rc<Explorer>, workspace_command_tx: &crate::WorkspaceCommandSender) -> dominator::Dom {
        let expanded = Mutable::new(true);
        html!("div", {
            .class("block")
            .apply(styles::panel::body)
            .child(html!("div", {
                .class("h-[35px]")
                .apply(styles::panel::title)
                .child(html!("span", {
                    .apply(styles::panel::title_text)
                    .text("Explorer")
                }))
            }))
            // project listing
            .child(html!("ul", {
                .child(html!("li", {
                    .attr("draggable", "true")
                    .event(clone!(this => move |_: events::DragEnter| {
                        this.drop_target.set(Some(this.workspace.clone()));
                    }))
                    .event_with_options(&EventOptions::preventable(), |event: events::DragOver| {
                        event.prevent_default(); // Allow drop
                    })
                    .event_with_options(&EventOptions::preventable(), clone!(this => move |event: events::Drop| {
                        if let Some(((drag_target, drag_parent), drop_target)) = this.drag_object.get_cloned().zip(this.drop_target.get_cloned()) {
                            match drag_target {
                                Target::File(file) => {
                                    let mut files = drag_parent.files.lock_mut();
                                    // unwrapping is safe since we expect the drag_target to be in its parent
                                    let index = files.iter().position(|f| Rc::ptr_eq(f, &file)).unwrap();
                                    let file = files.get(index).unwrap().clone();
                                    files.remove(index);
                                    drop_target.files.lock_mut().push_cloned(file);
                                },
                                Target::Directory(directory) => {
                                    let mut directories = drag_parent.directories.lock_mut();
                                    // unwrapping is safe since we expect the drag_target to be in its parent
                                    let index = directories.iter().position(|f| Rc::ptr_eq(f, &directory)).unwrap();
                                    let directory = directories.get(index).unwrap().clone();
                                    directories.remove(index);
                                    drop_target.directories.lock_mut().push_cloned(directory);
                                },
                            }
                        } else {
                            wasm_bindgen_futures::spawn_local(clone!(this => async move {
                                let (dropped_files, dropped_directories) = handle_drop(event).await;
                                // NOTE: Never hold a lock across an await point
                                if let Some(drop_target) = this.drop_target.get_cloned() {
                                    let mut directories = drop_target.directories.lock_mut();
                                    let mut files = drop_target.files.lock_mut();
                                    dropped_files.into_iter()
                                        .for_each(|file| files.push_cloned(file.into()));
                                    dropped_directories.into_iter()
                                        .for_each(|directory| directories.push_cloned(directory.into()));
                                }
                            }));
                        }
                    }))
                    .child(html!("div", {
                        .apply(styles::vfs_item::body)
                        .event(clone!(expanded, this => move |event: events::Click| {
                            // left-click to expand directory
                            let rename_active = this.rename.get_cloned().is_none();
                            if rename_active && event.button() == MouseButton::Left {
                                let mut expanded = expanded.lock_mut();
                                *expanded = !*expanded;
                            }
                        }))
                        .children(&mut [
                            html!("div", {
                                .apply(styles::vfs_item::icon)
                                .child_signal(expanded.signal_ref(|expanded| match expanded {
                                    true => folder_open_icon(),
                                    false => folder_closed_icon(),
                                }.into()))
                            }),
                            html!("div", {
                                // input box for renaming
                                .child_signal(this.rename.signal_cloned().map(clone!(this => move |target| {
                                    match target {
                                        Some(Target::Directory(ref dir)) if Rc::ptr_eq(dir, &this.workspace) => {
                                            Some(html!("input" => web_sys::HtmlInputElement, {
                                                .apply(styles::input)
                                                .attr("type", "text")
                                                .attr("value", &*this.workspace.name.get_cloned())
                                                .with_node!(element => {
                                                    .event(clone!(this => move |_: events::Input| {
                                                        this.workspace.name.set(element.value());
                                                    }))
                                                    .event(clone!(this => move |_: events::Blur| {
                                                        this.rename.set(None);
                                                    }))
                                                    .event(clone!(this => move |event: events::KeyDown| {
                                                        if event.key() == "Enter" {
                                                            this.rename.set(None);
                                                        }
                                                    }))
                                                })
                                            }))
                                        },
                                        _ => Some(html!("span", {
                                            .text_signal(this.workspace.name.signal_cloned())
                                        })),
                                    }
                                })))
                            })
                        ])
                        // event listener for right click
                        .event(clone!(this => move |event: events::ContextMenu| {
                            this.context_menu.set(Some(ContextMenu::new(
                                (event.x(), event.y()),
                                Target::Directory(this.workspace.clone()),
                            )));
                        }))
                    }))
                    // check for update in show to render context menu
                    .child_signal(this.context_menu.signal_ref(clone!(this => move |menu_state| {
                        menu_state.as_ref().map(clone!(this => move |menu| {
                            match &menu.target {
                                Target::Directory(_) => ContextMenu::folder_menu_render(menu, this.rename.clone()),
                                Target::File(_) => ContextMenu::file_menu_render(menu, this.rename.clone()),
                            }
                        }))
                    })))
                    // prevents default chrome context menu for the whole vfs structure
                    .event_with_options(&EventOptions::preventable(), |event: events::ContextMenu| {
                        event.prevent_default();
                    })
                    // global event listener to close context menu if tab_menu is opened
                    .global_event(clone!(this => move |event: events::MouseDown| {
                        if event.button() == MouseButton::Right {
                            this.context_menu.set(None)
                        }
                    }))
                    // global event listener to close context menu
                    .global_event(clone!(this => move |_: events::Click| {
                        this.context_menu.set(None);
                    }))
                    .child_signal(expanded.signal_ref(clone!(this, workspace_command_tx => move |expanded| {
                        expanded.then_some(render_contents(&this, &this.workspace, &workspace_command_tx))
                    })))
                }))
            }))
        })
    }

    pub fn tooltip(&self) -> &'static str {
        "Explorer"
    }

    pub fn icon(&self, active: impl Signal<Item = bool> + 'static) -> Dom {
        svg!("svg", {
            .apply(|dom| styles::menu::button_toggle(dom, active))
            .attr("viewBox", "0 0 27 27")
            .child(svg!("path", {
                .attr("d", ICON_SVG_PATH)
            }))
        })
    }
}