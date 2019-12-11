# Low battery monitor

A simple battery monitor written in Rust. Notifications are sent at 10% and 5%.

Requires a notification daemon such as Dunst.


Todo:
- Prevent notification from being displayed if one has already been displayed within the last x minutes.
- Show battery percentage in notification.