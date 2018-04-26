extern crate bank;

use bank::account::Account as Account;
use bank::writer::output_statement as output_statement;

fn main() {
    let mut buffer = std::io::stdout();
    let mut account = Account::new(200);
    account.deposit(400);
    account.withdraw(80);
    account.withdraw(200);
    output_statement(&account,&mut buffer)
}
