use A1Q1::phonebook::*;
use std::io::*;

fn main() {
    let mut contact_list = PhoneBook::new();
    contact_list.load_records("contacts.txt");
    display_separator();
    display_intro();
    display_separator();
    menu(&mut contact_list);
    display_separator();
    contact_list.save_records("contacts2.txt");
}

fn display_intro() {
    let message = String::from("A Phone Book Management Program");
    println!("{}", message);
}

fn get_user_input(start: u32, end: u32) -> Option<u32> {
    print!("Enter your choice: ");
    match stdout().flush() {
        Ok(_) => (),
        Err(_) => panic!("Failed to print!")
    };
    let mut input = String::new();
    let mut num: u32;
    loop {
        match stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("RL_Invalid menu option.");
                print!("Please try again: ");
                match stdout().flush() {
                    Ok(_) => (),
                    Err(_) => panic!("Failed to print!")
                };
                continue;
            }
        };
        num = match input.parse::<u32>() {
            Ok(val) => val,
            Err(_) => {
                println!("PR_Invalid menu option.");
                print!("Please try again: ");
                match stdout().flush() {
                    Ok(_) => (),
                    Err(_) => panic!("Failed to print!")
                };
                continue;
            }
        };
        if num < start || num > end {
            println!("BND_Invalid menu option.");
            print!("Please try again: ");
            match stdout().flush() {
                Ok(_) => (),
                Err(_) => panic!("Failed to print!")
            };
            continue;
        }
        else {
            break;
        }
    };
    Some(num)
}

fn display_menu() {
    println!("1. Look up a contact");
    println!("2. Add a new contact");
    println!("3. Change an existing contact");
    println!("4. Delete a contact");
    println!("5. Display all contacts");
    println!("6. Quit a program");
}

fn display_separator() {
    let lines = "-".repeat(48);
    println!("{}", lines);
}

fn menu(contact_list: &mut PhoneBook) {
    display_menu();
    display_separator();
    let mut selection = get_user_input(1, 6).unwrap();
    // let mut selection = 5;
    while selection != 6 {
        if selection == 1 {
            display_separator();
            contact_list.search_a_record();
        } else if selection == 2 {
            display_separator();
            contact_list.add_a_record();
        } else if selection == 3 {
            display_separator();
            contact_list.update_a_record();
        } else if selection == 4 {
            display_separator();
            contact_list.delete_a_record();
        } else {
            display_separator();
            println!("List of Contacts:");
            contact_list.display_records();
        }
        display_separator();
        display_menu();
        display_separator();
        selection = get_user_input(1, 6).unwrap();
        // selection = 6;
    }
    display_separator();
    println!("Thank you!")
}