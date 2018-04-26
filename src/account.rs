use transaction::*;

#[derive(Debug)]
pub struct Account {
    balance: i32,
    history: Vec<(Transaction,i32)>,
}

impl Account {
    pub fn new(balance: i32) -> Self {
        Account {
            balance,
            history: Vec::new(),
        }
    }

    pub fn withdraw(&mut self, amount: i32) {
        self.balance = self.balance - amount;
        self.history.push((self::Transaction::withdrawal(amount), self.balance))
    }

    pub fn deposit(&mut self, amount: i32) {
        self.balance = self.balance + amount;
        self.history.push((self::Transaction::deposit(amount), self.balance))
    }

    pub fn history(&self) -> &Vec<(Transaction,i32)> {
        &self.history
    }

    pub fn balance(&self) -> &i32 {
        &self.balance
    }
}

#[cfg(test)]
mod account {
    use super::*;

    #[test]
    fn withdraw_adds_a_transaction_and_corresponding_balance_to_account() {
        let mut account = Account::new(0);
        let test_amount = 250;
        account.withdraw(test_amount);
        assert_eq!(account.history(), &vec![(Transaction::withdrawal(test_amount), -250)]);
    }

    #[test]
    fn deposit_adds_a_transaction_and_corresponding_balance_to_account() {
        let mut account = Account::new(0);
        let test_amount = 100;
        account.deposit(test_amount);
        assert_eq!(account.history(), &vec![(Transaction::deposit(test_amount), 100)]);
    }

    #[test]
    fn balance_check() {
        let mut account = Account::new(100);
        account.deposit(1000);
        account.withdraw(150);
        account.withdraw(320);
        assert_eq!(&630,account.balance())
    }
}

// N.B we are currently assuming an infinite overdraft
