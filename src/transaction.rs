enum TransactionType {
    Credit,
    Debit,
}

pub struct Transaction {
    amount: i32,
    action: TransactionType,
}

impl Transaction {
    pub fn withdrawal(amount: i32) -> Transaction {
        Transaction {
            amount,
            action: TransactionType::Credit,
        }
    }

    pub fn deposit(amount: i32) -> Transaction {
        Transaction {
            amount,
            action: TransactionType::Debit,
        }
    }
}

#[cfg(test)]
mod transaction {
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
        match withdrawal.action {
            TransactionType::Debit => panic!("Expected Credit, but received Debit"),
            TransactionType::Credit => return,
        }
    }

    #[test]
    fn deposit_has_action_deposit() {
        let deposit = Transaction::deposit(250);
        match deposit.action {
            TransactionType::Debit => return,
            TransactionType::Credit => panic!("Expected Dedit, but received Credit"),
        }
    }
}
