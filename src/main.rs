mod logo;
mod utility;
use std::collections::HashMap;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

#[derive(Debug, EnumIter, EnumString)]
enum Functions {
    Help,
    Settings,
    WebBrowser,
    RaptorTextEditor,
    Calculator,
    RaptorImageEditor,
    RaptorFileExplorer,
    Startup,
    Back,
}

trait Execute {
    fn run(&self);
    fn help() {
        println!("Sorry, no help has been defined.");
    }
}

impl Execute for Functions {
    fn run(&self) {
        match self {
            Functions::Help => {
                utility::border("Help");
                println!("Here is a list of all our menus. Please choose one to learn more.");
                exec_loop();
            }
            _ => {}
        }
    }
}

fn exec_loop() {
    let mut functions_hash = HashMap::new();
    for (index, function) in Functions::iter().enumerate() {
        functions_hash.insert(index + 1, function);
    }
    for (index, function) in Functions::iter().enumerate() {
        let index = index + 1;
        if index % 2 == 0 {
            print!("[{}] {:?}\n", index, function);
        } else {
            print!("[{}] {:?} ", index, function);
        }
    }
    let input = utility::input("\n[*] ").trim_end().to_owned();
    match input.as_str() {
        "" => {
            utility::clear_screen();
            println!("Invalid Input. Please enter the name or number of the menu.");
            exec_loop();
        },
        input if input.chars().all(|c| c.is_digit(10)) => {
            let choice: usize = input.parse::<usize>().unwrap();
            match functions_hash.get(&choice) {
                Some(function) => function.run(),
                None => {
                    utility::clear_screen();
                    println!("Please enter a valid number within the range 1-{}.", functions_hash.keys().len());
                    exec_loop();
                }
            }
        }
        input if input.chars().all(|c| c.is_ascii_alphabetic()) => {
            match Functions::from_str(input) {
                Ok(function) => function.run(),
                Err(_) => {
                    utility::clear_screen();
                    println!("Menu does not exist. Please input a valid name or number for the menu.");
                    exec_loop();
                }
            }
        }
        _ => {
            println!("Invalid Input. Please enter the name or number of the menu.");
            exec_loop();
        },
    }
}

fn main() {
    utility::date();
    println!("{}", logo::TEXT);
    println!("Welcome to Rusty_OS.");
    println!("type in the Menu name to go to it and 'Back' to return here.");
    println!("If help is needed, please choose 'Help'.");
    println!("-------------------");
    utility::border("Main Menu");
    exec_loop();
}
