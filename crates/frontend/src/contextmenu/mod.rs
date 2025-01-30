use std::rc::Rc;
use dominator::{Dom, html, clone, events};
use futures_signals::signal::Mutable;
use crate::sidebar::explorer::RENAME;
use crate::workspace::activity_panel;
use crate::{styles, ColumnType, COLS};
use crate::{vfs::{Directory, File}, DEFAULT_DIRECTORY_MODE, DEFAULT_FILE_MODE};
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
            .class("absolute")
            .class("z-[1000]")
            .class("w-60")
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
            .class("absolute")
            .class("z-[1000]")
            .class("w-60")
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
    ) -> () {
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
    ) -> () {
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
    // position of the tab context menu
    pub position: (i32, i32),
    // target activity
    pub activity_panel: Mutable<Option<Rc<activity_panel::Activity>>>
}

impl TabMenu {
    pub fn new(position: (i32, i32), activity_panel: Mutable<Option<Rc<activity_panel::Activity>>>) -> Self {
        Self {
            position,
            activity_panel
        }
    }

    pub fn render(
        tab_menu: &TabMenu,
        
    ) -> Dom {
        html!("div", {
            .class("absolute")
            .class("z-[1000]")
            .class("w-60")
            .style("left", &format!("{}px", tab_menu.position.0)) // X position
            .style("top", &format!("{}px", tab_menu.position.1))  // Y position
            .apply(styles::contextmenu::body)
            .children(&mut [
                html!("div", {
                    .text("Split Right")
                    .apply(styles::contextmenu::option)
                    .event(|_: events::MouseDown| {
                        COLS.with(|cols| cols.lock_mut().extend(vec![
                            ColumnType::Auto,
                            ColumnType::Fr
                        ]))
                    })
                })
            ])
        })
    }
}