use std::rc::Rc;
use futures_signals::signal::Mutable;
use dominator::{Dom, html, clone, events};
use wasm_bindgen::JsValue;
use crate::{vfs::{Directory, File}, DEFAULT_DIRECTORY_MODE, DEFAULT_FILE_MODE};
#[allow(dead_code)]
#[derive(Clone)]
pub enum Menutype {
    File(Rc<File>), 
    Directory(Rc<Directory>)
}
#[allow(dead_code)]
#[derive(Clone)]
pub struct ContextMenu {
    // visibility and position of contextmenu
    pub position: (i32, i32),
    // to see which file or folder is clicked
    pub target_object: Menutype
}


impl ContextMenu {
    pub fn new(pos: (i32, i32), tar_obj: Menutype) -> Self {
        Self {
            position: pos,
            target_object: tar_obj
        }
    }

    // rendering context menu for folder
    pub fn folder_menu_render(
        context_menu_state: &ContextMenu
    ) -> Dom {
        html!("div", {
                .class("menu")
                .class("has-background-light")
                .style("position", "absolute")
                .style("border", "1px solid black")
                .style("padding", "10px")
                .style("z-index", "1000")
                .style("left", &format!("{}px", context_menu_state.position.0)) // X position
                .style("top", &format!("{}px", context_menu_state.position.1)) // Y position
                .children(&mut [
                    html!("div", {
                        .text("New Folder")
                        .style("cursor", "pointer")
                        .event(clone!(context_menu_state => move |_event: events::MouseDown| {
                            web_sys::console::log_1(&"New Folder Created".into());
                            context_menu_state.add_folder();
                        }))
                    }),
                    html!("div", {
                        .text("New File")
                        .style("cursor", "pointer")
                        .event(clone!(context_menu_state => move |_event: events::MouseDown| {
                            web_sys::console::log_1(&"New File Created".into());
                            context_menu_state.add_file();
                        }))
                    }),
                    html!("div", {
                        .text("Rename Folder")
                        .style("cursor", "pointer")
                        .event(clone!(context_menu_state => move |_event: events::MouseDown| {
                            web_sys::console::log_1(&"Renaming Folder".into());
                        }))
                    })
                ])
            })
    }
    
    pub fn file_menu_render(
        context_menu_state: &ContextMenu
    ) -> Dom {
        html!("div", {
            .class("menu")
            .class("has-background-light")
            .style("position", "absolute")
            .style("border", "1px solid black")
            .style("padding", "10px")
            .style("z-index", "1000")
            .style("left", &format!("{}px", context_menu_state.position.0)) // X position
            .style("top", &format!("{}px", context_menu_state.position.1)) // Y position
            .children(&mut [
                html!("div", {
                    .text("Open File")
                    .style("cursor", "pointer")
                    .event(clone!(context_menu_state => move |_event: events::MouseDown| {
                        web_sys::console::log_1(&"Option 1 clicked".into());
                    }))
                }),
                html!("div", {
                    .text("Rename File")
                    .style("cursor", "pointer")
                    .event(clone!(context_menu_state => move |_event: events::MouseDown| {
                        web_sys::console::log_1(&"Option 2 clicked".into());
                    }))
                })
            ])
        })
    }

    pub fn rename_object(
        &self
    ) -> () {
        todo!()
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
        if let Menutype::Directory(dir) = &self.target_object {
            dir.directories.lock_mut().push_cloned(new_directory);
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
        if let Menutype::Directory(dir) = &self.target_object {
            dir.files.lock_mut().push_cloned(new_file);
        }
    }
}