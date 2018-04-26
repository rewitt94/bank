extern crate bank;
use bank::account::Account as Account;
use bank::writer::output_statement as output_statement;

#[derive(Debug,PartialEq)]
struct Statement {
    bytes: Vec<u8>,
}

#[test]
fn account_stores_all_transactions() {
    let mut account = Account::new(0);
    account.deposit(200);
    account.deposit(400);
    assert_eq!(account.history().len(), 2)
}


#[test]
fn writing_statement_one_to_buffer() {
    // we will write to this buffer
    let mut buffer: Vec<u8> = Vec::new();

    // load file as bytes
    let mut statement_as_bytes = include_bytes!("examples/statement_one.txt").to_vec();
    // Add an extra line feed
    statement_as_bytes.push(10);
    // Store it in a struct to use Debug & PartialEq
    let statement = Statement { bytes: statement_as_bytes };

    // Run code
    let mut account = Account::new(200);
    account.deposit(300);
    account.withdraw(250);
    account.deposit(100);

    output_statement(&account,&mut buffer);

    assert_eq!(&buffer,&statement.bytes)
}

#[test]
fn writing_statement_two_to_buffer() {
    // we will write to this buffer
    let mut buffer: Vec<u8> = Vec::new();

    // load file as bytes
    let mut statement_as_bytes = include_bytes!("examples/statement_two.txt").to_vec();
    // Add an extra line feed
    statement_as_bytes.push(10);
    // Store it in a struct to use Debug & PartialEq
    let statement = Statement { bytes: statement_as_bytes };

    // Run code
    let mut account_two = Account::new(1000);
    account_two.deposit(2000);
    account_two.withdraw(2999);

    output_statement(&account_two,&mut buffer);

    assert_eq!(&buffer,&statement.bytes)
}
