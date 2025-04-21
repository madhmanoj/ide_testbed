use dominator::{html, Dom};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};

thread_local! {
    static NOTIFICATIONS: MutableVec<Notification> = Default::default();
}

#[derive(Clone)]
pub enum NotificationType {
    Info,
    Warn,
    Error
}

#[derive(Clone)]
pub struct Notification {
    pub title: String,
    pub description: String,
    pub message_type: NotificationType
}

pub fn render() -> Dom {
    html!("div", {
        .class("w-[400px]")
        .style("z-index", dominator::HIGHEST_ZINDEX)
        .style("position", "absolute")
        .style("right", "10px")
        .style("bottom", "0px")
        .children_signal_vec(NOTIFICATIONS.with(|vec| {
            vec.signal_vec_cloned().map(|notification| {
                match notification.message_type {
                    NotificationType::Info => html!("div", {
                        .style("background-color", "blue")
                        .class("mb-2")
                        .child(html!("div", {
                            .class("m-2")
                            .text(&notification.title)
                        }))
                        .child(html!("div", {
                            .class("m-2")
                            .text(&notification.description)
                        }))
                    }),
                    NotificationType::Warn => html!("div", {
                        .style("background-color", "yellow")
                        .class("mb-2")
                        .child(html!("div", {
                            .class("m-2")
                            .text(&notification.title)
                        }))
                        .child(html!("div", {
                            .class("m-2")
                            .text(&notification.description)
                        }))
                    }),
                    NotificationType::Error => html!("div", {
                        .style("background-color", "red")
                        .class("mb-2")
                        .child(html!("div", {
                            .class("m-2")
                            .text(&notification.title)
                        }))
                        .child(html!("div", {
                            .class("m-2")
                            .text(&notification.description)
                        }))
                    }),
                }
            })  
        }))
    })
}


pub fn append_notification(notification: Notification) {
    NOTIFICATIONS.with(move |vec| {
        vec.lock_mut().push_cloned(notification);
    });
}