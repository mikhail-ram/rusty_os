use chrono::{DateTime, Local};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

fn input(string: &str) -> String {
    let mut input = String::new();
    if string != "" {
        println!("{}", string);
    }
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

macro_rules! input {
    ($s: expr) => {
        input($s);
    };
    () => {
        input("")
    };
}

fn border(string: &str) {
    let border_vertical: String = format!(" {} ", std::iter::repeat("-").take(string.len() + 2).collect::<String>());
    println!("{}", border_vertical);
    println!("| {} |", string);
    println!("{}", border_vertical);
}

fn exec_loop() {
    /*
    for (index, function) in FUNCTIONS.iter().enumerate() {
        println!("[{}] {}", index + 1, function);
    }
    */
    for (index, function) in Functions::iter().enumerate() {
        println!("[{}] {:?}", index + 1, function);
    }
}

fn is_string_numeric(string: &String) -> bool {
    let list: Vec<bool> = string.chars().map(|c| c.is_numeric()).collect();
    let list = &list[..list.len()-2];
    list.iter().all(|v| *v == true)
}

fn main() {
    const TEXT: &str = "|||||  |||||  |||||  || ||  ||||        |||||    |||||
|| ||  || ||  || ||   |||   || ||       || ||    ||
||||   |||||  ||||     |    || ||       || ||     ||
|| ||  || ||  ||       |    ||||        |||||  |||||";

    let now: DateTime<Local> = Local::now();
    println!("{}", now.format("%a, %d %b %Y %H:%M."));
    println!("{}", TEXT);
    println!("Welcome to Rapyd_OS.");
    println!("type in the Menu name to go to it and 'Back' to return here. If help is needed, type in 'Help()'");
    println!("-------------------");
    border("Main Menu");
    exec_loop();
    let input = input!("Pick a choice:");
    if is_string_numeric(&input) {
        let choice: usize = input[..input.len()-2].parse::<usize>().unwrap() - 1;
        println!("{}", FUNCTIONS[choice])
    } else {
    }
}
