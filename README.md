# qTime

qTime is a simple command-line application written in Rust that retrieves the current user's name and displays it along with the current date and time.

## Prerequisites

- Rust programming language (version 1.0 or later)

## Installation

- Clone the repository or download the source code files.

      git clone https://github.com/your-username/qTime.git

- Navigate to the project directory.

      cd qTime
      
- Build the application using the Rust package manager, Cargo.

      cargo build --release
      
- The executable will be generated in the target/release directory.

## Usage

- Open a terminal or command prompt
- Navigate to the directory containing the `qTime` executable.
- Run the application.

      ./qTime

## Features

- Retrieves the current username using the users crate.
- Displays a personalized greeting with the user's name.
- Retrieves the current date and time using the chrono crate.
- Prints the date in the format: YYYY-MM-DD.
- Prints the time in the format: HH:MM:SS AM/PM.

      Hello: JohnDoe
      The date is: 2023-05-25
      The time is: 07:35:12 PM
     
## License

This project is licensed under the GPL 3.0
