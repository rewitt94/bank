#[derive(Debug,PartialEq)]
pub enum TransactionType {
    Credit,
    Debit,
}

#[derive(Debug,PartialEq)]
pub struct Transaction {
    pub amount: i32,
    pub action: TransactionType,
}

impl Transaction {
    pub fn withdrawal(amount: i32) -> Self {
        Transaction {
            amount,
            action: TransactionType::Credit,
        }
    }

    pub fn deposit(amount: i32) -> Self {
        Transaction {
            amount,
            action: TransactionType::Debit,
        }
    }
}

#[cfg(test)]
mod transaction_tests {
    use super::*;

    #[test]
    fn withdrawal_stores_amount() {
        let withdrawal = Transaction::withdrawal(100);
        assert_eq!(withdrawal.amount, 100);
    }

    #[test]
    fn deposit_stores_amount() {
        let deposit = Transaction::deposit(250);
        assert_eq!(deposit.amount, 250);
    }

    #[test]
    fn withdrawal_has_action_withdrawal() {
        let withdrawal = Transaction::withdrawal(100);
        assert_eq!(withdrawal.action, TransactionType::Credit)
    }

    #[test]
    fn deposit_has_action_deposit() {
        let deposit = Transaction::deposit(250);
        assert_eq!(deposit.action, TransactionType::Debit)
    }
}

// dates yet to be added
