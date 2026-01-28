use chrono;
use notify_rust::Notification;
use std::thread;
use std::time::Duration;
use std::io;
use std::process::Command;


fn main() {
    // 1. Set your bedtime (24-hour format: Hour, Minute, Second)

    let mut raw_input = String::new();

    println!("Please enter your desired bed-time below in format 00:00.");

    io::stdin()
        .read_line(&mut raw_input)
        .expect("Failed to read line. Input has to be in format: 00:00");

    let input = raw_input.trim();

    // split(<separator>) creates an iterator
    // collect () transforms the iterator into a collection
    // split() returns a string slice for efficiency & ownership
    let parts: Vec<&str> = input.split(':').collect();

    let hour: u32 = parts[0].parse().expect("Hour is not a number!");
    let minute: u32 = parts[1].parse().expect("Minute is not a number!");

    let bedtime = chrono::NaiveTime::from_hms_opt(hour, minute, 0).unwrap();

    let key_times = [
        bedtime - chrono::Duration::hours(3),
        bedtime - chrono::Duration::minutes(250),
        bedtime - chrono::Duration::hours(2),
        bedtime - chrono::Duration::hours(1)
    ];

    println!("Timer started! I'll remind you at {}.", bedtime);

    // Creating flags to avoid bugs regarding overlapping times
    let mut key_time_1_over = false;
    let mut key_time_2_over = false;
    let mut key_time_3_over = false;
    let mut reminder_sent = false;

    loop {
        let now = chrono::Local::now().time();

        // 2. Check if it's time
        // We check if 'now' is greater than bedtime to trigger the alert

        if now > key_times[0] && now < key_times[1] && !key_time_1_over {
            Command::new("xsct")
                .arg("4000")
                .spawn()
                .expect("Failed to execute xsct. Is it installed?");

            key_time_1_over = true;
        }

        if now >= key_times[1] && now < key_times[2] && !key_time_2_over {
            Command::new("xsct")
                .arg("3000")
                .spawn()
                .expect("Failed to execute xsct. Is it installed?");

            key_time_2_over = true;
        }

        if now >= key_times[2] && now < key_times[3] && !key_time_3_over {

            Command::new("xsct")
                .arg("2000")
                .spawn()
                .expect("Failed to execute xsct. Is it installed?");

            key_time_3_over = true;
        }

        // Sending reminder 1 hour before bed
        if now >= key_times[3] && now < bedtime && !reminder_sent {
            Notification::new()
                .summary("Bedtime-Reminder")
                .body("Bedtime is in 1 hour!")
                .icon("alarm-clock") // Standard Ubuntu icon name
                .timeout(0)          // 0 means the notification won't disappear until clicked
                .show()
                .unwrap();

            Command::new("xsct")
                .arg("2000")
                .spawn()
                .expect("Failed to execute xsct. Is it installed?");

            reminder_sent = true;
        }

        if now >= bedtime {
            Notification::new()
                .summary("Bed time!")
                .body("Go to sleep! Your tomorrow-self will thank you.")
                .icon("alarm-clock") // Standard Ubuntu icon name
                .timeout(0)          // 0 means the notification won't disappear until clicked
                .show()
                .unwrap();

            Command::new("xsct")
                .arg("1000")
                .spawn()
                .expect("Failed to execute xsct. Is it installed?");

            break; // Exit the program after the notification
        }

        // 3. Wait a bit before checking again to save CPU
        thread::sleep(Duration::from_secs(60));
    }
}
