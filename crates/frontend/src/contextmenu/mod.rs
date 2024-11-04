use std::rc::Rc;
use futures_signals::signal::Mutable;
use dominator::{Dom, html, clone, events};
use wasm_bindgen::JsValue;
use crate::{vfs::{Directory, File}, DEFAULT_DIRECTORY_MODE, DEFAULT_FILE_MODE};

#[derive(Clone, Copy)]
pub enum Menutype {
    File, 
    Directory
}

pub struct ContextMenu {
    // visibility and position of contextmenu
    pub show: Mutable<bool>,
    pub position: Mutable<(i32, i32)>,
    // to split the contextmenu separately for file and folder
    pub menu_type: Mutable<Option<Menutype>>,
    // to see which file or folder is clicked
    pub target_object: Mutable<Option<Rc<Directory>>>
}


impl ContextMenu {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            show: Mutable::new(false),
            position: Mutable::new((0, 0)),
            menu_type: Mutable::new(None),
            target_object: Mutable::new(None)
        })
    }

    // rendering context menu for folder
    // split for file and folder
    pub fn render_menu(
        context_menu_state: Rc<ContextMenu>
    ) -> Dom {
        match context_menu_state.menu_type.get() {
            // folder contextmenu
            Some(Menutype::Directory) => html!("div", {
                    .class("menu")
                    .class("has-background-light")
                    .style("position", "absolute")
                    .style("border", "1px solid black")
                    .style("padding", "10px")
                    .style("z-index", "1000")
                    .style_signal("left", context_menu_state.position.signal_ref(|(x, _y)| {
                        format!("{}px", x)
                    }))
                    .style_signal("top", context_menu_state.position.signal_ref(|(_x, y)| {
                        format!("{}px", y)
                    }))
                    .children(&mut [
                        html!("div", {
                            .text("New Folder")
                            .style("cursor", "pointer")
                            .event(clone!(context_menu_state => move |_event: events::MouseDown| {
                                web_sys::console::log_1(&"New Folder Created".into());
                                // to print in console log for debugging
                                web_sys::console::log_1(&JsValue::from_str(
                                    &context_menu_state.target_object
                                        .get_cloned()
                                        .as_ref()
                                        .map(|dir| dir.name.lock_ref().to_string())
                                        .unwrap_or_else(|| "No directory selected".to_string())
                                ));
                                context_menu_state.add_folder();
                                context_menu_state.show.set_neq(false); // Hide the menu after clicking
                            }))
                        }),
                        html!("div", {
                            .text("New File")
                            .style("cursor", "pointer")
                            .event(clone!(context_menu_state => move |_event: events::MouseDown| {
                                web_sys::console::log_1(&"New File Created".into());
                                web_sys::console::log_1(&JsValue::from_str(
                                    &context_menu_state.target_object
                                        .get_cloned()
                                        .as_ref()
                                        .map(|dir| dir.name.lock_ref().to_string())
                                        .unwrap_or_else(|| "No directory selected".to_string())
                                ));
                                context_menu_state.add_file();
                                context_menu_state.show.set_neq(false); // Hide the menu after clicking
                            }))
                        }),
                        html!("div", {
                            .text("Rename Folder")
                            .style("cursor", "pointer")
                            .event(clone!(context_menu_state => move |_event: events::MouseDown| {
                                web_sys::console::log_1(&"Option 3 clicked".into());
                                context_menu_state.show.set_neq(false); // Hide the menu after clicking
                            }))
                        })
                    ])
                }),
            // file contextmenu
            Some(Menutype::File) =>  html!("div", {
                .class("menu")
                .class("has-background-light")
                .style("position", "absolute")
                .style("border", "1px solid black")
                .style("padding", "10px")
                .style("z-index", "1000")
                .style_signal("left", context_menu_state.position.signal_ref(|(x, _y)| {
                    format!("{}px", x)
                }))
                .style_signal("top", context_menu_state.position.signal_ref(|(_x, y)| {
                    format!("{}px", y)
                }))
                .children(&mut [
                    html!("div", {
                        .text("Open File")
                        .style("cursor", "pointer")
                        .event(clone!(context_menu_state => move |_event: events::MouseDown| {
                            web_sys::console::log_1(&"Option 1 clicked".into());
                            context_menu_state.show.set_neq(false); // Hide the menu after clicking
                        }))
                    }),
                    html!("div", {
                        .text("Rename File")
                        .style("cursor", "pointer")
                        .event(clone!(context_menu_state => move |_event: events::MouseDown| {
                            web_sys::console::log_1(&"Option 2 clicked".into());
                            context_menu_state.show.set_neq(false); // Hide the menu after clicking
                        }))
                    })
                ])
            }),
            None => Dom::empty()
        }
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
        if let Some(ref dir) = self.target_object.get_cloned() {
            dir.directories.lock_mut().push_cloned(new_directory);
        } else {
            // Handle the case when no directory is selected
            web_sys::console::log_1(&"No target directory selected".into());
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
        if let Some(ref dir) = self.target_object.get_cloned() {
            dir.files.lock_mut().push_cloned(new_file);
        } else {
            // Handle the case when no directory is selected
            web_sys::console::log_1(&"No target directory selected".into());
        }
    }
}