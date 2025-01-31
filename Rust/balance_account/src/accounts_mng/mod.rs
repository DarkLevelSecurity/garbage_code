use std::io;
use crate::debug;

pub struct Account {
    pub name: String,
    pub password: String,
    pub balance: f32,
}
impl Account {
    pub fn login(&self, name: String, password: String) -> bool {
        name == self.name && password == self.password
    }

    pub fn inc(&mut self, amount: f32) {
        self.balance += amount;
    }
    pub fn dec(&mut self, amount: f32) {
        self.balance -= amount;
    }
    pub fn ret(&self) -> f32 {
        self.balance
    }
}

pub struct Login(pub String, pub String);
impl Login {
    fn default() -> (String, String) {
        ("".to_string(), "".to_string())
    }
}

pub fn get_login(header: String) -> Login {
    let mut input = Login::default();

    debug::head(header);
    debug::input("Enter your name:".to_string());
    io::stdin()
        .read_line(&mut input.0)
        .expect("Failed to read the input: ");
    debug::input("Enter your password: ".to_string());
    io::stdin()
        .read_line(&mut input.1)
        .expect("Failed to read the input: ");

    Login(input.0, input.1)
}

pub fn create_account(name_i: String, password_i: String) -> Account {
    Account {
        name: name_i,
        password: password_i,
        balance: 0.0,
    }
}

pub fn get_amount() -> f32 {
    let mut input: String = String::new();
    debug::input("Enter an amount: ".to_string());
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input: ");
    input.trim().parse().expect("wrong input")
}
