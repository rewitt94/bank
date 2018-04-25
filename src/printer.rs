trait Printer {

    pub fn print_statement(account: Account) {
        let statement = write_statement(account)
        println!(statement)
    }

    fn write_statement(account: Accont) -> String {

    }

    fn write_header() -> String {

    }

    fn write_all_transactions(account: Accont) -> String {

    }

    fn write_transaction(transaction: Transaction) -> String {

    }


}

// this test isn't running?
#[cfg(test)]
mod printer {
    use super::*;

    #[test]
    fn printer_test() {
        panic!("FAIL")
    }
}
