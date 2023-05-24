use chrono::prelude::*;

fn main() {
    let local: DateTime<Local> = Local::now();
    println!("Current time: {}", local.format("%Y-%m-%d %I:%M:%S %p"));
}
