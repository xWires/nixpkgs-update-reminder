use std::collections::HashMap;
use zbus::{proxy, zvariant::OwnedValue};

#[derive(Debug)]
pub enum NotificationSeverity {
    Low = 0,
    Normal = 1,
    Critical = 2
}

#[proxy(
    interface = "org.freedesktop.Notifications",
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
pub trait Notifications {
    async fn notify(&self, app_name: &str, replaces_id: u32, app_icon: &str, summary: &str, body: &str, actions: Vec<&str>, hints: HashMap<&str, OwnedValue>, expire_timeout: i32) -> zbus::Result<u32>;
}
