use chrono::prelude::*;
use users::get_current_username;

fn main() {
    //get's the user name of the user and then prints
    match get_current_username() {
        Some(username) => {
            println!("Hello: {}", username.to_string_lossy());
        }
        None => {
            eprintln!("Failed to retrieve the username.");
        }
    }

    //get's time and then prints it
    let local: DateTime<Local> = Local::now();
    //date
    println!("The date is: {}", local.format("%Y-%m-%d"));
    //time
    println!("The time is: {}", local.format("%I:%M:%S %p"));
}
