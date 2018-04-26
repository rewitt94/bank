use transaction::*;

#[derive(Debug)]
pub struct Account {
    transactions: Vec<Transaction>,
}

impl Account {
    pub fn new() -> Self {
        Account {
            transactions: Vec::new(),
        }
    }

    pub fn withdraw(&mut self, amount: i32) {
        self.transactions.push(self::Transaction::withdrawal(amount))
    }

    pub fn deposit(&mut self, amount: i32) {
        self.transactions.push(self::Transaction::deposit(amount))
    }

    pub fn history(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}

#[cfg(test)]
mod account {
    use super::*;

    #[test]
    fn withdraw_adds_a_transaction_to_account() {
        let mut account = Account::new();
        let test_amount = 250;
        account.withdraw(test_amount);
        assert_eq!(account.history(), &vec![Transaction::withdrawal(test_amount)]);
    }

    #[test]
    fn deposit_adds_a_transaction_to_account() {
        let mut account = Account::new();
        let test_amount = 100;
        account.deposit(test_amount);
        assert_eq!(account.history(), &vec![Transaction::deposit(test_amount)]);
    }
}
