/// Send a desktop notification.
///
/// Uses notify-rust on Linux, falls back to terminal bell.
pub fn send_notification(title: &str, message: &str) {
    #[cfg(target_os = "linux")]
    {
        match notify_rust::Notification::new()
            .summary(title)
            .body(message)
            .timeout(notify_rust::Timeout::Milliseconds(5000))
            .show()
        {
            Ok(_) => return,
            Err(_) => {
                // Fall through to terminal bell
            }
        }
    }

    // Fallback: terminal bell
    eprintln!("\x07[{}] {}", title, message);
}