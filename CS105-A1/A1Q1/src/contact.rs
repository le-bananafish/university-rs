use serde_derive::{Serialize, Deserialize};

use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contact {
    name: String,
    phone: String,
    email: String,
}

impl Contact {
    pub fn new(name: &str, phone: &str, email: &str) -> Self {
        Contact {
            name: name.to_string(),
            phone: phone.to_owned(),
            email: email.to_owned(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned()
    }

    pub fn get_phone(&self) -> String {
        self.phone.to_owned()
    }

    pub fn set_phone(&mut self, phone: &str) {
        self.phone = phone.to_owned()
    }

    pub fn get_email(&self) -> String {
        self.email.to_owned()
    }

    pub fn set_email(&mut self, email: &str) {
        self.email = email.to_owned()
    }
}

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}\nPhone: {}\nEmail: {}", self.name, self.phone, self.email)
    }
}

mod tests {
    use super::*;

    #[test]
    pub fn test_print_contact() {
        let c = Contact::new("Sebastian", "02102678959", "sebastianlau25@gmail.com");
        println!("{}", c);
    }
}