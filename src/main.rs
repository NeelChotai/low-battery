use std::{fs, process, thread, time};
use notify_rust::Notification;
use clap::{Arg, App};

fn main() {
    let args = App::new("Low battery monitor")
        .version("0.2.0")
        .author("Neel Chotai <neel@chot.ai>")
        .about("Monitors battery and notifies at specified levels.")
        .arg(Arg::with_name("low")
            .short("l")
            .long("low")
            .value_name("LEVEL")
            .help("First level to notify at.")
            .required(true))
        .arg(Arg::with_name("critical")
            .short("c")
            .long("critical")
            .value_name("LEVEL")
            .help("Last level to notify at.")
            .required(true))
        .get_matches();

    let low: i32 = args.value_of("low").unwrap_or("10").parse::<i32>().unwrap_or(10);
    let crit: i32 = args.value_of("critical").unwrap_or("5").parse::<i32>().unwrap_or(5);

    if low <= crit {
        eprintln!("Critical value is greater than low value! Exiting...");
        process::exit(0);
    }

    loop {
        let file = fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
            .expect("I/O error.");
        let level: i32 = file.trim().parse::<i32>().unwrap();
        let status = fs::read_to_string("/sys/class/power_supply/BAT0/status")
            .expect("I/O error.");
    
        if status.trim() == "Discharging" {
            if level <= crit {
                Notification::new()
                    .body(&format!("Battery level is critical ({}%).", level))
                    .show()
                    .expect("Failed to notify.");
            }
            else if level <= low {
                Notification::new()
                    .body(&format!("Battery level is low ({}%).", level))
                    .show()
                    .expect("Failed to notify.");
            }
        }

        if level > 30 {
            thread::sleep(time::Duration::from_secs(600));
        }
        else {
            thread::sleep(time::Duration::from_secs(120));
        }
    }
}
