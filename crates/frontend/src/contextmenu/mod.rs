use std::rc::Rc;
use dominator::{Dom, html, clone, events};
use crate::sidebar::explorer::RENAME;
use crate::styles::{contextmenu, contextmenu_option};
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

    // rendering context menu for folder
    pub fn folder_menu_render(
        context_menu: &ContextMenu
    ) -> Dom {
        html!("div", {
                .apply(contextmenu)
                .style("left", &format!("{}px", context_menu.position.0)) // X position
                .style("top", &format!("{}px", context_menu.position.1)) // Y position
                .children(&mut [
                    html!("div", {
                        .text("New Folder")
                        .apply(contextmenu_option)
                        .event(clone!(context_menu => move |_event: events::MouseDown| {
                            web_sys::console::log_1(&"New Folder Created".into());
                            context_menu.add_folder();
                        }))
                    }), 
                    html!("div", {
                        .text("New File")
                        .apply(contextmenu_option)
                        .event(clone!(context_menu => move |_event: events::MouseDown| {
                            web_sys::console::log_1(&"New File Created".into());
                            context_menu.add_file();
                        }))
                    }),
                    html!("div", {
                        .text("Rename Folder")
                        .apply(contextmenu_option)
                        .event(clone!(context_menu => move |_event: events::MouseDown| {
                            web_sys::console::log_1(&"Renaming Folder".into());
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
            .apply(contextmenu)
            .style("left", &format!("{}px", context_menu.position.0)) // X position
            .style("top", &format!("{}px", context_menu.position.1)) // Y position
            .children(&mut [
                html!("div", {
                    .text("Rename File")
                    .apply(contextmenu_option)
                    .event(clone!(context_menu => move |_event: events::MouseDown| {
                        web_sys::console::log_1(&"Renaming File".into());
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