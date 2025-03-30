use std::rc::Rc;
use dominator::{Dom, html, clone, events};
use crate::workspace::panel::LayoutPanel;
use crate::workspace::activity_panel::{Activity, ActivityPanel};
use crate::{styles, vfs::{Directory, File}, DEFAULT_DIRECTORY_MODE, DEFAULT_FILE_MODE, sidebar::explorer::RENAME};
#[derive(Clone)]
pub enum Target {
    File(Rc<File>), 
    Directory(Rc<Directory>)
}
#[derive(Clone)]
pub struct ContextMenu {
    // visibility and position of contextmenu
    pub position: (i32, i32),
    // to see which file or folder is clicked
    pub target: Target,
}

impl ContextMenu {
    pub fn new(position: (i32, i32), target: Target) -> Self {
        Self {
            position,
            target,
        }
    }

    pub fn folder_menu_render(
        context_menu: &ContextMenu
    ) -> Dom {
        html!("div", {
            .style("position", "absolute")
            .style("z-index", "1000")
            .style("width", "15rem")
            .style("left", &format!("{}px", context_menu.position.0)) // X position
            .style("top", &format!("{}px", context_menu.position.1))  // Y position
            .apply(styles::contextmenu::body)
            .children(&mut [
                html!("div", {
                    .text("New Folder")
                    .apply(styles::contextmenu::option)
                    .event(clone!(context_menu => move |_: events::MouseDown| {
                        context_menu.add_folder();
                    }))
                }), 
                html!("div", {
                    .text("New File")
                    .apply(styles::contextmenu::option)
                    .event(clone!(context_menu => move |_: events::MouseDown| {
                        context_menu.add_file();
                    }))
                }),
                html!("div", {
                    .text("Rename Folder")
                    .apply(styles::contextmenu::option)
                    .event(clone!(context_menu => move |_: events::MouseDown| {
                        web_sys::console::log_1(&"Hello bois".into());
                        if let Target::Directory(dir) = &context_menu.target  {
                            RENAME.with(|rename| {
                                rename.set(Some(Target::Directory(dir.clone())));
                            });
                        }
                    }))
                })
            ])
        })
    }
    
    pub fn file_menu_render(
        context_menu: &ContextMenu
    ) -> Dom {
        html!("div", {
            .style("position", "absolute")
            .style("z-index", "1000")
            .style("width", "15rem")
            .style("left", &format!("{}px", context_menu.position.0)) // X position
            .style("top", &format!("{}px", context_menu.position.1))  // Y position
            .apply(styles::contextmenu::body)
            .children(&mut [
                html!("div", {
                    .text("Rename File")
                    .apply(styles::contextmenu::option)
                    .event(clone!(context_menu => move |_: events::MouseDown| {
                        if let Target::File(file) = &context_menu.target  {
                            RENAME.with(|rename| {
                                rename.set(Some(Target::File(file.clone())));
                            });
                        }
                    }))
                })
            ])
        })
    }

    // to add folder under a folder
    pub fn add_folder(
        &self
    ) {
        // initialise a new directory
        let new_directory = Rc::new(
            Directory {
                name: "Placeholder".to_owned().into(),
                mode: DEFAULT_DIRECTORY_MODE.into(),
                directories: vec![].into(),
                files: vec![].into()
            }
        );

        // to access the target directory for modification
        if let Target::Directory(dir) = &self.target {
            dir.directories.lock_mut().push_cloned(new_directory.clone());
            // this signals renaming after creating and pushing it into the directory structure
            RENAME.with(|rename| {
                rename.set(Some(Target::Directory(new_directory.clone())));
            });
        } 
    }

    // to add file under a folder
    pub fn add_file(
        &self
    ) {
        // initialise a new file
        let new_file = Rc::new(
            File {
                name: "Placeholder".to_owned().into(),
                mode: DEFAULT_FILE_MODE.into(),
                data: "Placeholder".as_bytes().to_vec().into()
            }
        );

        // to access the target directory for modification
        if let Target::Directory(dir) = &self.target {
            dir.files.lock_mut().push_cloned(new_file.clone());
            // this signals renaming after creating and pushing it into the directory structure
            RENAME.with(|rename| {
                rename.set(Some(Target::File(new_file.clone())));
            });
        }
    }
}

pub struct TabMenu {
    pub position: (i32, i32),
    pub panel: Rc<LayoutPanel>
}

impl TabMenu {
    pub fn render(
        &self
    ) -> Dom {
        let TabMenu { position, panel } = self;
        let show_close = if let LayoutPanel::Widget { parent: Some(parent), .. } = panel.as_ref() {
            match parent.as_ref() {
                LayoutPanel::HorizontalSplit { children, parent: grand_parent } 
                | LayoutPanel::VerticalSplit { children, parent: grand_parent } => {
                    children.lock_ref().len() == 1 && grand_parent.is_none()
                },
                _ => false
            }
        } else {
            false
        };
        html!("div", {
            .style("left", format!("{}px", position.0))
            .style("top", format!("{}px", position.1))
            .style("gap", "2px")
            .style("position", "absolute")
            .style("width", "10rem")
            .style("z-index", dominator::HIGHEST_ZINDEX)
            .style("background-color", "lightblue")
            .apply(styles::contextmenu::body)
            // .child(
            //     html!("div", {
            //         .apply_if(!show_close, |dom| 
            //             dom
            //                 .text("Close")
            //                 .class("cursor-pointer")
            //                 .event(clone!(panel => move |_:events::MouseDown| {
            //                     if let Panel::Widget { parent: Some(parent), .. } = panel.as_ref() {
            //                         match parent.as_ref() {
            //                             Panel::HorizontalSplit { children, parent: grand_parent } | Panel::VerticalSplit { children, parent: grand_parent } => {
            //                                 // index should always be a Some value, since self should always be present in children mutablevec of its parent
            //                                 // otherwise panic since the panel is invalid
            //                                 let mut parent_children = children.lock_mut();
            //                                 let index = parent_children.iter().position(|(child, _)| {
            //                                     Rc::ptr_eq(child, &panel)
            //                                 }).unwrap();
                                            
            //                                 web_sys::console::log_1(&format!("{index}").into());
            //                                 // remove widget
            //                                 parent_children.remove(index);

            //                                 // check if parent has only one child left, if yes, clean up the panel layout
            //                                 if parent_children.len() == 1 {
            //                                     // there is only 1 element according to the if condition
            //                                     let (only_child, _) = parent_children.first().unwrap();

            //                                     match only_child.as_ref() {
            //                                         // for cases where the only child is a split, the logic is to promote all the panels of the child to the mutablevec 
            //                                         // of the panel's parent
            //                                         // this makes sense because in the nesting structure according to our implementation, there can only ever be a 
            //                                         // horizontal split inside a vertical split and vice versa, and if we directly promote the split panel, we will have a 
            //                                         // vertical split inside a vertical split (similarly for horizontal split) which is redundant
            //                                         Panel::HorizontalSplit { children: only_child_children, .. }
            //                                         | Panel::VerticalSplit { children: only_child_children, .. } => {
            //                                             if let Some(grand_parent) = grand_parent {
            //                                                 match grand_parent.as_ref() {
            //                                                     Panel::HorizontalSplit { children: grand_parent_children, .. }
            //                                                     | Panel::VerticalSplit { children: grand_parent_children, .. } => {
            //                                                         let mut grand_parent_children = grand_parent_children.lock_mut();
            //                                                         if let Some(index) = grand_parent_children.iter().position(|(child, _)| Rc::ptr_eq(child, parent)) {
            //                                                             for (i, (panel, size)) in only_child_children.lock_ref().iter().enumerate() {
            //                                                                 let new_child = match panel.as_ref() {
            //                                                                     Panel::HorizontalSplit { children, .. } => Rc::new(Panel::HorizontalSplit { 
            //                                                                         parent: Some(grand_parent.clone()), 
            //                                                                         children: children.clone() 
            //                                                                     }),
            //                                                                     Panel::VerticalSplit { children, .. } => Rc::new(Panel::VerticalSplit { 
            //                                                                         parent: Some(grand_parent.clone()), 
            //                                                                         children: children.clone()
            //                                                                     }),
            //                                                                     Panel::Widget { color, .. } => Rc::new(Panel::Widget { 
            //                                                                         parent: Some(grand_parent.clone()), 
            //                                                                         color 
            //                                                                     })
            //                                                                 };
            //                                                                 let size = size.get();
            //                                                                 if i == 0 {
            //                                                                     grand_parent_children.set_cloned(index + i, (new_child.clone(), Mutable::new(size)));
            //                                                                 } else {
            //                                                                     grand_parent_children.insert_cloned(index + i, (new_child.clone(), Mutable::new(size)));
            //                                                                 }
            //                                                             }
            //                                                         }
            //                                                     },
            //                                                     _ => {}
            //                                                 }
            //                                             }  
            //                                         },
            //                                         // for cases where only_child is a widget, promote widget to the grand_parent directly if its the only panel left in the parent after removal of the original widget
            //                                         Panel::Widget { color, .. } => {
            //                                             let new_widget = Rc::new(Panel::Widget { 
            //                                                 parent: grand_parent.clone(), 
            //                                                 color 
            //                                             });

            //                                             if let Some(grand_parent) = grand_parent {
            //                                                 match grand_parent.as_ref() {
            //                                                     Panel::HorizontalSplit { children: grand_parent_children, .. } 
            //                                                     | Panel::VerticalSplit { children: grand_parent_children, .. } => {
            //                                                         let mut grand_parent_children = grand_parent_children.lock_mut();
            //                                                         if let Some(index) = grand_parent_children.iter().position(|(child, _)| Rc::ptr_eq(child, parent)) {
            //                                                             let size = grand_parent_children[index].1.get();
            //                                                             grand_parent_children.set_cloned(index, (new_widget, Mutable::new(size)));
            //                                                         }
            //                                                     },
            //                                                     _ => {}
            //                                                 }
            //                                             }
            //                                         },
            //                                     }

            //                                 }
            //                             },
            //                             _ => {}
            //                         }
            //                     }
            //                 }))
            //         )
            //     })
            // )
            .child(html!("div", {
                .text("Split Right")
                .class("cursor-pointer")
                .apply(styles::contextmenu::option)
                .event(clone!(panel => move |_: events::MouseDown| {
                    panel.split_horizontal(false);
                }))
            }))
            .child(html!("div", {
                .text("Split Left")
                .class("cursor-pointer")
                .apply(styles::contextmenu::option)
                .event(clone!(panel => move |_: events::MouseDown| {
                    panel.split_horizontal(true);
                }))
            }))
            .child(html!("div", {
                .text("Split Up")
                .class("cursor-pointer")
                .apply(styles::contextmenu::option)
                .event(clone!(panel => move |_: events::MouseDown| {
                    panel.split_vertical(true);
                }))
            }))
            .child(html!("div", {
                .text("Split Down")
                .class("cursor-pointer")
                .apply(styles::contextmenu::option)
                .event(clone!(panel => move |_: events::MouseDown| {
                    panel.split_vertical(false);
                }))
            }))
        })
    }
}