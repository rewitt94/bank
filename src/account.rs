use transaction::*;

// pub trait ITransaction {
//     fn withdrawal(amount: i32) -> Self;
//     fn deposit(amount: i32) -> Self;
// }
//
// impl ITransaction for Transaction {
//     fn withdrawal(amount: i32) -> Self {
//         Transaction {
//             amount,
//             action: TransactionType::Credit,
//         }
//     }
//
//     fn deposit(amount: i32) -> Self {
//         Transaction {
//             amount,
//             action: TransactionType::Debit,
//         }
//     }
// }

#[derive(Debug)]
pub struct Account {
    pub transactions: Vec<Transaction>,
}

impl Account {
    pub fn new() -> Self {
        Account {
            transactions: Vec::new(),
        }
    }

    pub fn withdraw(&mut self, amount: i32) {
        self.transactions.push(self::ITransaction::withdrawal(amount))
    }

    pub fn deposit(&mut self, amount: i32) {
        self.transactions.push(self::ITransaction::deposit(amount))
    }
}

#[cfg(test)]
mod account {
    use super::*;

    #[test]
    fn account_test() {

    }
}
