use crate::log::color::{green, magenta, red, white, yellow};
use std::fmt::Display;

pub fn info(action: impl Display, subject: impl Display) {
    println!("  {} {}{}", green(action), white(subject), white("..."));
}

pub fn downloading(dependency: impl Display, location: impl Display) {
    println!(
        "    {} {} {} {}",
        magenta("Downloading"),
        white(dependency),
        magenta("to"),
        white(location)
    );
}

pub fn building(module: impl Display) {
    println!(
        "    {} {}{}",
        green("Building"),
        white(module),
        white("...")
    );
}

pub fn finished(task: impl Display) {
    println!("  {} {}", green("Finished"), white(task));
}

pub fn warning(warning: impl Display) {
    println!("    {} {}", yellow("Warning:"), white(warning));
}

pub fn importing(module: impl Display) {
    println!("      {} {}", magenta("Importing"), white(module));
}

pub fn failure() {
    println!("  {}", red("Failure"));
}

pub fn error(error: impl Display) {
    failure();

    let e = error.to_string();
    if let Some("Error") = e.as_str().split(' ').nth(0) {
        eprintln!("{} {}", red("Error"), white(&e[6..]));
    } else {
        let e = match e.to_string().chars().nth(0) {
            Some(ch) => ch.to_lowercase().to_string() + &e[1..],
            None => String::from("")
        };
        eprintln!("{}{} {}", red("Error"), white(":"), white(e));
    }
}
