extern crate serde;
extern crate serde_json;

use crate::contact::Contact;
use std::{
    fs::{File, OpenOptions},
    fmt,
    io::{prelude::*, BufReader},
};

#[derive(Debug)]
pub struct PhoneBook {
    records: Vec<Contact>,
}

impl PhoneBook {
    pub fn new() -> Self {
        PhoneBook {
            records: Vec::with_capacity(12),
        }
    }

    pub fn size(&self) -> usize {
        self.records.len()
    }

    pub fn load_records(&mut self, filename: &str) {
        let f = match File::open(filename) {
            Ok(f) => f,
            Err(_) => panic!("Failed to open file!")
        };
        let f = BufReader::new(f);
        for line in f.lines() {
            match line {
                Ok(line) => {
                    let ds: Contact = serde_json::from_str(&line).unwrap();
                    self.records.push(ds)
                },
                Err(_) => println!("Failed to derserialise Contact!")
            }
        }
    }

    pub fn save_records(&mut self, filename: &str) {
        let mut f = match OpenOptions::new().append(true).open(filename) {
            Ok(f) => f,
            Err(_) => panic!("Failed to open file!")
        };
        let mut to_write = String::new();
        for item in self.records.iter() {
            let serialised = serde_json::to_string(&item).unwrap();
            to_write.push_str(&serialised);
        }
        match f.write_all(to_write.as_bytes()) {
            Ok(_) => (),
            Err(_) => println!("Failed to write file!")
        }
    }

    pub fn find_a_record(&mut self, name: &str) -> Option<&mut Contact> {
        for item in self.records.iter_mut() {
            if item.get_name() == name {
                return Some(item)
            }
        };
        return None
    }

    pub fn display_a_record(self, item: &Contact) {
        println!("{}", item);
    }

    pub fn search_a_record(&mut self) {
        print!("Enter a name: ");
        let mut name = String::new();
        match std::io::stdin().read_line(&mut name) {
            Ok(_) => (),
            Err(_) => panic!("Failed to read line!")
        };
        let result = self.find_a_record(&name);
        match result {
            Some(a) => println!("{}", a),
            None => println!("{} is not found in the phone book.", name),
        }
    }

    pub fn add_a_record(&mut self) {
        print!("Enter a name: ");
        std::io::stdout().flush();
        let mut name = String::new();
        match std::io::stdin().read_line(&mut name) {
            Ok(_) => (),
            Err(_) => panic!("Failed to read line!")
        };
        let result = self.find_a_record(&name);
        match result {
            Some(_) => println!("{} is already in the phone book.", name),
            None => {
                print!("Enter phone number: ");
                let mut phone = String::new();
                match std::io::stdin().read_line(&mut phone) {
                    Ok(_) => (),
                    Err(_) => panic!("Failed to read line!")
                };
                print!("Enter email address: ");
                let mut email = String::new();
                match std::io::stdin().read_line(&mut email) {
                    Ok(_) => (),
                    Err(_) => panic!("Failed to read line!")
                };
                let new_contact = Contact::new(&name, &phone, &email);
                self.records.push(new_contact);
                println!("{} is added to the phone book.", &name);
            }
        }
    }

    pub fn update_a_record(&mut self) {
        unimplemented!();
    }

    pub fn delete_a_record(&mut self) {
        unimplemented!();
    }

    pub fn display_records(&mut self) {
        for (i, item) in self.records.iter_mut().enumerate() {
            println!("Record #{}:", i+1);
            println!("{}", item);
        }
    }
}

impl fmt::Display for PhoneBook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, item) in self.records.iter().enumerate() {
            write!(f, "Record: {}", item)
        }
        // write!("{}", "{}", self.records)
    }
}

// impl fmt::Display for std::vec::Vec<contact::Contact> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for (i, item) in self.iter().enumerate() {
//             write!("{}", item)
//         }
//     }
// }

mod test {
    // use crate::phonebook::PhoneBook;
    use super::PhoneBook;

    #[test]
    fn test_load_records() {
        let mut t = PhoneBook::new();
        t.load_records("contacts.txt");
    }

    #[test]
    fn test_save_records() {
        let mut t = PhoneBook::new();
        t.save_records("contacts2.txt");
    }

    #[test]
    fn test_add_a_record() {
        let mut t = PhoneBook::new();
        t.add_a_record();
    }

    #[test]
    fn test_print_phonebook() {
        let mut t = PhoneBook::new();
        t.add_a_record();
        t.display_records();
    }
}