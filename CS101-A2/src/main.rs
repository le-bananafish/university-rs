use std::io;

fn get_number(num: &str, spaces: u32) -> String {
    " ".repeat(spaces as usize - num.len()) + num
}

fn get_word(words: &str, spaces: u32) -> String {
    let length = words.len();
    if length > spaces as usize {
        words.to_owned()
    }
    else {
        let blanks = (spaces as usize - length) / 2;
        if blanks != 0 {
            " ".repeat(blanks + 1) + words + &" ".repeat(blanks)
        }
        else {
            " ".repeat(blanks) + words + &" ".repeat(blanks)
        }
    }
}

fn get_row(num: u32, size: u32, spaces: u32) -> String {
    if num == 0 {
        let mut row = get_number("", spaces);
        for i in 0..size {
            row = row + &get_number(&(i + 1).to_string(), spaces)
        }
        row
    }
    else {
        let mut row = get_number(&(num.to_string() + " |").to_string(), spaces);
        for i in 0..size {
            row = row + &get_number(&(num * (i + 1)).to_string(), spaces)
        }
        row
    }
}

fn display_separator(size: u32, spaces: u32) {
    println!("{}", "-".repeat(((size + 1) * spaces) as usize))
}

fn display_table(size: u32, spaces: u32) {
    for num in 0..size {
        println!("{}", get_row(num + 1, size, spaces))
    }
}

fn get_user_input_worker() -> u32 {
    let mut size = String::new();
    io::stdin().read_line(&mut size).expect("Failed to read line.");
    let size = size.trim().parse::<u32>().expect("Failed to convert String to u32");
    size
}

fn get_user_input() -> u32 {
    let mut size = get_user_input_worker();
    while !(size >= 1 && size <= 20) {
        print!("Table size should be between 1 and 20");
        size = get_user_input_worker();
    }
    size
}

fn display_heading(size: u32, spaces: u32) {
    display_separator(size, spaces);
    let title = size.to_string() + "x" + &size.to_string() + " Times Table";
    println!("{}", get_word(&title, (size + 1) * spaces));
    println!("{}", get_row(0, size, spaces));
    display_separator(size, spaces);
}

fn main() {
    let size: u32 = get_user_input();
    let spaces = ((size * size).to_string().len() + 2) as u32;
    display_heading(size,spaces);
    display_table(size,spaces);
    display_separator(size,spaces);
}