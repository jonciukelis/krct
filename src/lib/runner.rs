use crate::input_types::{Type, Amount, Row};
use crate::output_types::{Output, OutputRow};
use std::collections::HashMap;


pub struct Runner(HashMap<u16, Account>);

struct Account {
    available: f64,
    held: f64,
    locked: bool,
    transactions: HashMap<u32, Transaction>
}

struct Transaction {
    tx_types: Vec<Type>,
    amount: f64
}

impl Runner {
    pub fn new() -> Self {
        Runner(HashMap::new())
    }

    pub fn run_row(&mut self, row: Row) {
        let Runner(hashmap) = self;
        let client = row.client;
        // Set up empty client if client doesn't exist.
        if !hashmap.contains_key(&client) {
            hashmap.insert(client, Account{
                available: 0.0,
                held: 0.0,
                locked: false,
                transactions: HashMap::new()
            });
        }
        let tx = row.tx;
        // Set up empty transaction if transaction doesn't exist.
        if !hashmap[&client].transactions.contains_key(&tx){
            let account = hashmap.get_mut(&client).unwrap();
            account.transactions.insert(tx, Transaction {
                tx_types: vec![],
                amount: 0.0
            });
        }
        // Some variables and mutables.
        let available = hashmap[&client].available;
        let held = hashmap[&client].held;
        let transaction_amount = hashmap[&client].transactions[&tx].amount;
        let tx_types = hashmap[&client].transactions[&tx].tx_types.clone();
        let account = hashmap.get_mut(&client).unwrap();
        let transaction = account.transactions.get_mut(&tx).unwrap();
        // Runner functions depending on type.
        match row.tx_type {
            Type::Deposit => {
                if let Amount::Some(amount) = row.amount {
                    account.available = available + amount;
                    transaction.tx_types.push(Type::Deposit);
                    transaction.amount = amount;
                } else {
                    panic!("Deposit without amount")
                }
            },
            Type::Withdrawal => {
                if let Amount::Some(amount) = row.amount {
                    account.available = available - amount;
                    transaction.tx_types.push(Type::Deposit);
                    transaction.amount = amount;
                } else {
                    panic!("Withdrawal without amount")
                }
            },
            Type::Dispute => {
                if tx_types.contains(&Type::Deposit) {
                    account.available = available - transaction_amount;
                    account.held = held + transaction_amount;
                    transaction.tx_types.push(Type::Dispute);
                }
            },
            Type::Resolve => {
                if tx_types.contains(&Type::Deposit) && tx_types.contains(&Type::Dispute) {
                    account.available = available + transaction_amount;
                    account.held = held - transaction_amount;
                    transaction.tx_types.push(Type::Resolve);
                }
            },
            Type::Chargeback => {
                if tx_types.contains(&Type::Deposit) && tx_types.contains(&Type::Dispute) {
                    account.held = held - transaction_amount;
                    account.locked = true;
                    transaction.tx_types.push(Type::Chargeback);
                }
            },
        }

    }
    //Generate output!
    pub fn output(&self) -> Output {
        let Runner(hashmap) = self;
        let mut rows = vec![];
        for account in hashmap {
            let (client, account_details) = account;
            let available = &account_details.available;
            let held = &account_details.held;
            let total = available + held;
            let locked = &account_details.locked;
            rows.push(OutputRow{
                client: *client,
                available: format!("{:.4}", available),
                held: format!("{:.4}", held),
                total: format!("{:.4}", total),
                locked: *locked
            });
        }
        Output(rows)
    }
}