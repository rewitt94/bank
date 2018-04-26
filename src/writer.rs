use std::io::Write;

use account::*;
use transaction::*;

pub fn output_statement<B>(account: &Account, mut writer: B)
where
    B: Write,
{
    let statement = write_statement(account);
        writeln!(&mut writer,"{}", statement);
}

fn write_statement(account: &Account) -> String {
    let mut statement = String::from("");
    statement.push_str(&write_header());
    statement.push_str(&write_all_transactions(account));
    statement
}

fn write_header() -> String {
    String::from("Debit || Credit || Balance\n")
}

fn write_all_transactions(account: &Account) -> String {
    let his = account.history();
    let mut formatted_transactions = String::new();
    for i in 0..his.len() {
        formatted_transactions.push_str(&write_transaction(&his[i].0,&his[i].1))
    }
    formatted_transactions
}

fn write_transaction(transaction: &Transaction, balance: &i32) -> String {
    let formatted_transaction;
    match transaction.action {
        TransactionType::Debit => formatted_transaction = format!("{} || || {}\n", transaction.amount, balance),
        TransactionType::Credit => formatted_transaction = format!("|| {} || {}\n", transaction.amount, balance),
    }
    formatted_transaction
}

#[cfg(test)]
mod printer {
    use super::*;

    #[test]
    fn header_formatted_correctly() {
        let header = write_header();
        assert_eq!(&header, "Debit || Credit || Balance\n")
    }

    #[test]
    fn transaction_formatted_correctly() {
        let deposit = Transaction::deposit(100);
        let formatted_transaction = write_transaction(&deposit, &400);
        assert_eq!(formatted_transaction, "100 || || 400\n")
    }

}
