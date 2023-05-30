use chrono::prelude::*;
use users::get_current_username;

fn main() {
    // Get's the user name of the user and then prints
    match get_current_username() {
        Some(username) => {
            println!("Hello: {}", username.to_string_lossy());
        }
        None => {
            eprintln!("Failed to retrieve the username.");
        }
    }

    // Get's time and date
    let local: DateTime<Local> = Local::now();
    // Prints the date to console
    println!("The date is: {}", local.format("%Y-%m-%d"));
    // Prints the time to console
    println!("The time is: {}", local.format("%I:%M:%S %p"));
}
