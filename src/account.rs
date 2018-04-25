// use transaction::Transaction;
// use double::mock::Mock;



struct Account {
    transactions: Vec<Transaction>,
}

impl Account {
    fn new() -> Account {
        Account {
            transactions: Vec::new(),
        }
    }

    fn withdraw(&mut self, amount: i32) {
        self.transactions.push(Transaction::withdrawal(amount))
    }

    fn deposit(&mut self, amount: i32) {
        self.transactions.push(Transaction::deposit(amount))
    }
}

#[cfg(test)]
mod account {
    use super::*;

    #[test]
    fn account_test() {

    }
}
