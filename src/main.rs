extern crate bank;
use bank::account::Account as Account;

fn main() {
    let mut account = Account::new();
    account.deposit(200);
    account.withdraw(150);
}
