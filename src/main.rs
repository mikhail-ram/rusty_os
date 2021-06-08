mod logo;
mod utility;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use std::collections::HashMap;

#[derive(Debug, EnumIter)]
enum Functions {
    Help,
    Settings,
    WebBrowser,
    RaptorTextEditor,
    Calculator,
    RaptorImageEditor,
    RaptorFileExplorer,
    Startup,
    Back
}

const FUNCTIONS: [&str; 9] = ["Help", "Settings", "Web_Browser",
                              "RaptorText_Editor", "Calculator",
                              "RaptorImage_Editor", "RaptorFile_Xplorer",
                              "Startup", "Back"];

fn exec_loop() {
    for (index, function) in Functions::iter().enumerate() {
        let index = index + 1;
        if index % 2 == 0 {
            print!("[{}] {:?}\n", index, function);
        }
        else {
            print!("[{}] {:?} ", index, function);
        }
    }
}

fn main() {
    let mut functions_hash = HashMap::new();
    for (index, function) in Functions::iter().enumerate() {
        functions_hash.insert(index + 1, function);
    }
    utility::date();
    println!("{}", logo::TEXT);
    println!("Welcome to Rusty_OS.");
    println!("type in the Menu name to go to it and 'Back' to return here.");
    println!("If help is needed, type in 'Help()'.");
    println!("-------------------");
    utility::border("Main Menu");
    exec_loop();
    let input = utility::input("\n[*] ").trim_end().to_owned();
    // TODO: Replace if/else logic with match clause.
    if input == "" {
        println!("Please enter a valid number");
    } else if input.chars().all(|c| c.is_digit(10)) {
        let choice: usize = input.parse::<usize>().unwrap() - 1;
        println!("{}", FUNCTIONS[choice]);
    } else if input.chars().all(|c| c.is_ascii_alphabetic()) {
        println!("Alphabetic!");
    } else {
        println!("Other");
    }
}
