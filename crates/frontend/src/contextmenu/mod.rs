use std::rc::Rc;
use dominator::{Dom, html, clone, events};
use uuid::Uuid;
use crate::workspace::{activity_panel::{Activity, ActivityPanel}, ColumnType, GridPanel, Workspace};
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
    pub position: (i32, i32)
}

impl TabMenu {
    pub fn new(position: (i32, i32)) -> Self {
        Self {
            position
        }
    }

    pub fn render(
        tab_menu: &TabMenu,
        workspace: &Rc<Workspace>,
        activity: &Rc<Activity>
    ) -> Dom {
        html!("div", {
            .style("position", "absolute")
            .style("z-index", "1000")
            .style("width", "15rem")
            .style("left", &format!("{}px", tab_menu.position.0)) // X position
            .style("top", &format!("{}px", tab_menu.position.1))  // Y position
            .apply(styles::contextmenu::body)
            .children(&mut [
                html!("div", {
                    .text("Split Right")
                    .apply(styles::contextmenu::option)
                    .event(clone!(workspace, activity => move |_: events::MouseDown| {
                        TabMenu::split_panel(&workspace, &activity);
                    }))
                })
            ])
        })
    }

    pub fn split_panel(
        workspace: &Rc<Workspace>,
        activity: &Rc<Activity>
    ) -> () {
        let new_uuid = Uuid::new_v4();
        let new_panel = ActivityPanel::new(activity);

        workspace.cols.lock_mut().extend(vec![ColumnType::Auto, ColumnType::Fr]);

        let index = workspace.activity_panel_list.lock_ref()
            .iter()
            .position(|(uuid, panel)| 
                *uuid == workspace.last_active_panel.get() && matches!(panel, GridPanel::Panel(_))
            )
            .unwrap();
        web_sys::console::log_1(&format!("{}", index).into());

        workspace.activity_panel_list.lock_mut().insert_cloned(index + 1, (new_uuid, GridPanel::Resizer));
        workspace.activity_panel_list.lock_mut().insert_cloned(index + 2, (new_uuid, GridPanel::Panel(new_panel)));

        workspace.last_active_panel.set(new_uuid);
    }
}