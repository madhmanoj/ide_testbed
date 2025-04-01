use std::rc::Rc;
use dominator::{Dom, html, clone, events};
use futures_signals::signal::Mutable;
use crate::workspace::panel::LayoutPanel;
use crate::{styles, vfs::{Directory, File}, DEFAULT_DIRECTORY_MODE, DEFAULT_FILE_MODE};
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
        context_menu: &ContextMenu,
        rename: Mutable<Option<Target>>
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
                    .event(clone!(context_menu, rename => move |_: events::MouseDown| {
                        context_menu.add_folder(rename.clone());
                    }))
                }), 
                html!("div", {
                    .text("New File")
                    .apply(styles::contextmenu::option)
                    .event(clone!(context_menu, rename => move |_: events::MouseDown| {
                        context_menu.add_file(rename.clone());
                    }))
                }),
                html!("div", {
                    .text("Rename Folder")
                    .apply(styles::contextmenu::option)
                    .event(clone!(context_menu, rename => move |_: events::MouseDown| {
                        if let Target::Directory(dir) = &context_menu.target  {
                            rename.set(Some(Target::Directory(dir.clone())));
                        }
                    }))
                })
            ])
        })
    }
    
    pub fn file_menu_render(
        context_menu: &ContextMenu,
        rename: Mutable<Option<Target>>
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
                    .event(clone!(context_menu, rename => move |_: events::MouseDown| {
                        if let Target::File(file) = &context_menu.target  {
                            rename.set(Some(Target::File(file.clone())));
                        }
                    }))
                })
            ])
        })
    }

    // to add folder under a folder
    pub fn add_folder(
        &self, 
        rename: Mutable<Option<Target>>
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
            rename.set(Some(Target::Directory(new_directory.clone())));
        } 
    }

    // to add file under a folder
    pub fn add_file(
        &self,
        rename: Mutable<Option<Target>>
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
            rename.set(Some(Target::File(new_file.clone())));
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
        html!("div", {
            .style("left", format!("{}px", position.0))
            .style("top", format!("{}px", position.1))
            .style("gap", "2px")
            .style("position", "absolute")
            .style("width", "10rem")
            .style("z-index", dominator::HIGHEST_ZINDEX)
            .style("background-color", "lightblue")
            .apply(styles::contextmenu::body)
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