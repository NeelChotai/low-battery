use std::fs;
use std::process::Command;

const NOTIF_HIGH: i32 = 10;
const NOTIF_LOW: i32 = 5;

fn main() {
    let file = fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
                .expect("I/O error.");
    let level: i32 = file.trim().parse::<i32>().unwrap();
    let status = fs::read_to_string("/sys/class/power_supply/BAT0/status")
                .expect("I/O error.");
    
    if status.trim() == "Discharging" {
        if level <= NOTIF_LOW {
            Command::new("notify-send")
                    .arg("Battery level is critical.")
                    .spawn()
                    .expect("Failed to notify.");
        }
        else if level <= NOTIF_HIGH {
            Command::new("notify-send")
                    .arg("Battery level is low.")
                    .spawn()
                    .expect("Failed to notify.");
        }
    }
}