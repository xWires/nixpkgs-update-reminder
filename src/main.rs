mod github;
mod notifications;

use std::{collections::HashMap, error::Error, process::Command};
use chrono::Utc;
use zbus::{Connection, zvariant::OwnedValue};

use crate::notifications::{NotificationSeverity, NotificationsProxy};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Connecting to D-Bus");

    let conn = Connection::session().await?;
    let proxy = NotificationsProxy::new(&conn).await?;

    println!("Getting current nixpkgs hash");

    let output = Command::new("nixos-version").output()?;

    // Example value:
    // 26.11.20260730.1559d3d (Zokor)
    let stdout_string = String::from_utf8_lossy(&output.stdout);

    // Example value:
    // 1559d3d (Zokor)
    let last_segment = stdout_string.split('.').last().expect("failed to parse NixOS version");

    // Example value:
    // 1559d3d
    let commit_hash = last_segment.split(' ').next().expect("failed to parse NixOS version");

    println!("Current hash is {commit_hash}");
    println!("Getting commit date");

    let commit_date = github::get_commit_date("NixOS", "nixpkgs", commit_hash).await?;

    let days_since = Utc::now().signed_duration_since(commit_date).num_days();

    println!("This system's nixpkgs is {days_since} days old");

    match days_since {
        0..5 => {}, // Do nothing if it's been less than 5 days
        5..7 => send_notification(&proxy, days_since, NotificationSeverity::Low).await?,
        7..14 => send_notification(&proxy, days_since, NotificationSeverity::Normal).await?,
        14.. => send_notification(&proxy, days_since, NotificationSeverity::Critical).await?,
        ..0 => unreachable!("nixpkgs was updated in the future! Is your date/time set correctly?")
    };

    println!("Closing D-Bus connection");

    conn.close().await?;

    Ok(())
}

async fn send_notification(proxy: &NotificationsProxy<'_>, days_since: i64, severity: NotificationSeverity) -> zbus::Result<()> {
    println!("Sending notification with \"{:?}\" severity", severity);

    let _ = proxy.notify(
        "Update Reminder",
        0,
        "nix-snowflake",
        "nixpkgs Update Reminder",
        &format!("This system's nixpkgs is {days_since} days old!"),
        vec![],
        HashMap::from([
            ("urgency", OwnedValue::from(severity as u8))
        ]),
        30 * 1000
    ).await?;

    Ok(())
}
