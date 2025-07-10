#[derive(Clone, Debug, Default)]
/// A pack of notifications, used to group multiple NotificationBuffer objects together.
pub struct NotificationPack {
    // PREND UN ATTRIBUT ET G2RE LA COOLECT DE NOTIFICATIONS
    /// Vector of notifications in the pack
    ///
    pub notifications: Vec<NotificationBuffer>,
}

impl NotificationPack {
    /// Create a new instance
    ///
    pub fn new(notifications: Vec<NotificationBuffer>) -> Self {
        Self { notifications }
    }

    /// Push a new notification into the pack
    ///
    pub fn push(&mut self, notification: NotificationBuffer) {
        self.notifications.push(notification);
    }

    /// Reset the pack, clearing all notifications
    ///
    pub fn reset(&mut self) {
        self.notifications.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.notifications.is_empty()
    }

    pub fn has_alert(&self) -> bool {
        for notification in &self.notifications {
            if notification.notification_type().unwrap() == NotificationType::Alert {
                return true;
            }
        }
        false
    }
}
