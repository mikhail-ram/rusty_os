use chrono::{DateTime, Local};
use std::io::prelude::*;

pub fn input(string: &str) -> String {
    let mut input = String::new();
    if string != "" {
        print!("{}", string);
    }
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout.");
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn border(string: &str) {
    let border_vertical: String = format!(
        "+{}+",
        std::iter::repeat("-")
            .take(string.len() + 2)
            .collect::<String>()
    );
    println!("{}", border_vertical);
    println!("| {} |", string);
    println!("{}", border_vertical);
}

pub fn date() {
    let now: DateTime<Local> = Local::now();
    println!("{}", now.format("%a, %d %b %Y %H:%M."));
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
