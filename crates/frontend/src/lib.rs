use std::{rc::Rc, sync::Arc};

use dominator::{clone, html};
use futures::{channel::mpsc, StreamExt};
use futures_signals::{map_ref, signal::SignalExt, signal_vec::{MutableVec, SignalVecExt}};
use once_cell::sync::Lazy;
use tracing_subscriber::{prelude::*, EnvFilter};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use workspace::{activity_panel::ActivityPanelCommand, ColumnType};

mod sidebar;
mod workspace;
mod vfs;
mod contextmenu;
mod styles;

const RESIZER_PX: u32 = 3;

enum WorkspaceCommand {
    OpenFile(Option<Uuid>, Rc<vfs::File>),
}
type WorkspaceCommandSender = mpsc::UnboundedSender<WorkspaceCommand>;
// type WorkspaceCommandReceiver = mpsc::UnboundedReceiver<WorkspaceCommand>;

#[wasm_bindgen(start)]
pub async fn main() {
    console_error_panic_hook::set_once();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()
            .with_ansi(true)
            .without_time()
            .with_writer(tracing_web::MakeConsoleWriter))
            .with(EnvFilter::from("frontend=trace"))
        .init();

    use sidebar::Sidebar;
    use workspace::Workspace;

    let (workspace_command_tx, workspace_command_rx) = mpsc::unbounded();

    let sidebar: Rc<Sidebar> = Default::default();
    let workspace: Rc<Workspace> = Default::default();
    
    let window_height = dominator::window_size()
        .map(|size| size.height.max(0.0) as u32);
    let window_width = dominator::window_size()
        .map(|size| size.width.max(0.0) as u32);
    let sidebar_width = Sidebar::width(&sidebar);

    let workspace_width = map_ref!(window_width, sidebar_width => {
        window_width.saturating_sub(*sidebar_width)
    });

    let console_height = workspace.console_height.signal();
    let activity_panel_height = 
        map_ref!(window_height, console_height => window_height.saturating_sub(console_height + RESIZER_PX));

    // signal resetting div layout based on sidebar resizing
    let global_cols = MutableVec::new_with_values(vec![
        ColumnType::Auto,
        ColumnType::Auto,
        ColumnType::Auto,
        ColumnType::Fr
    ]);

    let outer = html!("div", {

        .future(workspace_command_rx.for_each(clone!(workspace => move |command| clone!(workspace => async move {
            match command {
                WorkspaceCommand::OpenFile(maybe_uuid, file) => {
                    if let Some(uuid) = maybe_uuid {
                        if let Some(activity_panel) = workspace.activity_panel_list.lock_mut().get(&uuid) {
                            activity_panel
                                .activity_panel_tx
                                .unbounded_send(ActivityPanelCommand::OpenFile(file.clone()))
                                .unwrap();
                        }
                    } else if let Some(activity_panel) = workspace.activity_panel_list.lock_mut().get(&workspace.last_active_panel.lock_ref()) {
                        activity_panel
                            .activity_panel_tx
                            .unbounded_send(ActivityPanelCommand::OpenFile(file.clone()))
                            .unwrap();
                    }
                }
            }            
        }))))
        .class("grid")
        .style_signal("grid-template-columns", global_cols.signal_vec_cloned()
            .map(|col_type| match col_type {
                ColumnType::Auto => "auto".to_string(),
                ColumnType::Fr => "1fr".to_string()
            })
            .to_signal_cloned()
            .map(|columns| columns.join(" "))
        )
        .class("grid-rows-[1fr_auto_auto]")
        .class("h-full")

        .child(Sidebar::render_menu(&sidebar, global_cols.clone()))

        .child_signal(Sidebar::render_panel(&sidebar, &workspace_command_tx))

        .child_signal(Sidebar::render_vertical_resizer(&sidebar, global_cols.clone()))

        .child(Workspace::render_activity_panel(&workspace, workspace_width, activity_panel_height))

        .child(Workspace::render_horizontal_resizer(&workspace))

        .child(Workspace::render_console(&workspace))
    });

    dominator::append_dom(&dominator::body(), outer);
}

const DEFAULT_FILE_MODE: u32 = 0o664;
const DEFAULT_DIRECTORY_MODE: u32 = 0o775;
const LAUNCH_XML: &str = "\
<launch>
  <node pkg=\"velocity_control\" exec=\"run\" />
  <world size=\"3.0 6.0\">
    <model type=\"turtlebot\" pose=\"-0.5 0.0 2.0 0 0.785 0\" />
    <model type=\"turtlebot\" pose=\"0.5 0.0 -2.0 0 1.57 0\" />
  </world>
</launch>
";

const VELOCITY_CONTROL_PY: &str = "\
import rclpy

from rclpy.node import Node
from example_interfaces.msg import Velocity

class VelocityPublisher(Node):
    def __init__(self):
        super().__init__('velocity_publisher')
        self.publisher_ = \
            self.create_publisher(Velocity, 'velocity', 10)
        timer_period = 5.0  # seconds
        self.timer = \
            self.create_timer(timer_period, self.timer_callback)
        self.drive_forwards = True
    def timer_callback(self):
        if self.drive_forwards:
            # drive forwards
            self.get_logger().info('Driving forwards')
            self.publisher_.publish(Velocity(left=5.0, right=-5.0))
        else:
            # turn on the spot
            self.get_logger().info('Turning')
            self.publisher_.publish(Velocity(left=2.5, right=2.5))
        # toggle mode
        self.drive_forwards = not self.drive_forwards
        
rclpy.init()
velocity_publisher = VelocityPublisher()
rclpy.spin(velocity_publisher)
velocity_publisher.destroy_node()
rclpy.shutdown()
";

thread_local! {
    pub static GLOBAL_LOG: Lazy<MutableVec<Arc<str>>> = Lazy::new(Default::default);

    pub static PROJECT: Lazy<Rc<vfs::Directory>> = Lazy::new(|| {
        vfs::Directory {
            name: "project".to_owned().into(),
            mode: Default::default(),
            files: vec![
                vfs::File {
                    name: "launch.xml".to_owned().into(),
                    mode: DEFAULT_FILE_MODE.into(),
                    data: LAUNCH_XML.as_bytes().to_vec().into()
                }.into(),
            ].into(),
            directories: vec![
                vfs::Directory {
                    name: "velocity_control".to_owned().into(),
                    mode: DEFAULT_DIRECTORY_MODE.into(),
                    directories: vec![].into(),
                    files: vec![
                        vfs::File {
                            name: "run.py".to_owned().into(),
                            mode: DEFAULT_FILE_MODE.into(),
                            data: VELOCITY_CONTROL_PY.as_bytes().to_vec().into()
                        }.into(),
                    ].into(),
                }.into()
            ].into()
        }.into()
    });
}

