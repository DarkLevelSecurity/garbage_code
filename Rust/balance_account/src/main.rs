extern crate balance_account;

use std::io;
use balance_account::debug;

struct Account {
    name: String,
    password: String,
    balance: f32
}

struct Login (String, String);

impl Login {
    fn default() -> (String, String){
        ("".to_string(), "".to_string())
    }
}

impl Account {
    fn login(&self, name: String, password: String) -> bool {
        name == self.name && password == self.password
    }

    fn inc(&mut self, amount: f32) {
        self.balance += amount;
    }
    fn dec(&mut self, amount: f32) {
        self.balance -= amount;
    }
    fn ret(&self) -> f32 {
        self.balance
    }
}

fn main() {
    let mut account: Account;
    let input = get_login("create_account".to_string());
    account = create_account(input.0, input.1);

    let input = get_login("login".to_string());
    if !account.login(input.0, input.1) {
        debug::err("Wrong Name or Password".to_string());
        panic!("");
    }

    loop {
        let mut input: String = String::new();
        debug::head("options".to_string());
        println!("0. inc\n1. dec\n2. show\n3. exit");
        io::stdin().read_line(&mut input)
            .expect("Failed to read the input: ");
        let input: i32 = input.trim().parse()
            .expect("enter a number: ");
        match input {
            0 => account.inc(get_amount()),
            1 => account.dec(get_amount()),
            2 => debug::confirm(format!("{}", account.ret())),
            3 => panic!(),
            _ => debug::err("Enter a number".to_string()),
        }
    }
}

fn create_account(name_i: String, password_i: String) -> Account {
    Account {
        name: name_i,
        password: password_i,
        balance: 0.0
    }
}

fn get_amount() -> f32 {
    let mut input: String = String::new();
    debug::input("Enter an amount: ".to_string());
    io::stdin().read_line(&mut input)
        .expect("Failed to read the input: ");
    input.trim().parse().expect("wrong input")
}

fn get_login(header: String) -> Login {
    let mut input = Login::default();

    debug::head(header);
    debug::input("Enter your name:".to_string());
    io::stdin().read_line(&mut input.0)
        .expect("Failed to read the input: ");
    debug::input("Enter your password: ".to_string());
    io::stdin().read_line(&mut input.1)
        .expect("Failed to read the input: ");

    Login(input.0, input.1)
}
