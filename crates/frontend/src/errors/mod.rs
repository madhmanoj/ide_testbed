use dominator::{html, Dom, events, clone};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use crate::styles::FOREGROUND_COLOR;
use std::rc::Rc;

const NOTIFICATION_FONT_SIZE: i32 = 15;

thread_local! {
    static NOTIFICATIONS: MutableVec<Rc<Notification>> = Default::default();
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
                    NotificationType::Info => todo!(),
                    NotificationType::Warn => todo!(),
                    NotificationType::Error => html!("div", {
                        .style("background-color", FOREGROUND_COLOR)
                        .style("border", "1px solid black")
                        .style("border-radius", "6px")
                        .style("position", "relative")
                        .class("mb-2")
                        .class("shadow-lg")
                        
                        // Close button
                        .child(html!("button", {
                            .style("position", "absolute")
                            .style("top", "8px")
                            .style("right", "8px")
                            .style("background", "none")
                            .style("border", "none")
                            .style("color", "black")
                            .style("cursor", "pointer")
                            .style("font-size", "18px")
                            .style("line-height", "1")
                            .text("x")
                            .event(clone!(notification => move |_: events::Click| {
                                close_notification(&notification);
                            }))
                        }))
                        
                        // Title
                        .child(html!("div", {
                            .class("pr-[30px]")
                            .class("m-2")
                            .style("font-size", format!("{}px", NOTIFICATION_FONT_SIZE))
                            .class("font-bold")
                            .style("color", "#dc2626")
                            .text(&notification.title)
                        }))
                        
                        // Description
                        .child(html!("div", {
                            .class("pr-[30px]")
                            .class("m-2")
                            .style("font-size", format!("{}px", NOTIFICATION_FONT_SIZE))
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
        vec.lock_mut().push_cloned(Rc::new(notification));
    });
}

fn close_notification(notification: &Rc<Notification>) {
    NOTIFICATIONS.with(|vec| {
        let mut vec_lock = vec.lock_mut();
        if let Some(index) = vec_lock.iter().position(|n| Rc::ptr_eq(n, notification)) {
            vec_lock.remove(index);
        }
    });
}