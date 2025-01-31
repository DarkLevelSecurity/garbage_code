extern crate balance_account;

use balance_account::debug;
use balance_account::accounts_mng;
use std::io;

fn main() {
    let mut account: accounts_mng::Account;
    let input = accounts_mng::get_login("create_account".to_string());
    account = accounts_mng::create_account(input.0, input.1);

    let input = accounts_mng::get_login("login".to_string());
    if !account.login(input.0, input.1) {
        debug::err("Wrong Name or Password".to_string());
        panic!("");
    }

    loop {
        let mut input: String = String::new();
        debug::head("options".to_string());
        println!("0. inc\n1. dec\n2. show\n3. exit");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input: ");
        let input: i32 = input.trim().parse().expect("enter a number: ");
        match input {
            0 => account.inc(accounts_mng::get_amount()),
            1 => account.dec(accounts_mng::get_amount()),
            2 => debug::confirm(format!("{}", account.ret())),
            3 => panic!(),
            _ => debug::err("Enter a number".to_string()),
        }
    }
}
