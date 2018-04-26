extern crate bank;
use bank::account::Account as Account;

#[test]
fn account_stores_all_transactions() {
    let mut account = Account::new();
    account.deposit(200);
    account.deposit(400);
    assert_eq!(account.history().len(), 2)
}
