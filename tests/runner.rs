#[cfg(test)]
mod integration_tests {
    use large_squidlib::input_types::{Row, Type, Amount};
    use large_squidlib::runner::Runner;
    use large_squidlib::output_types::Output;

    #[test]
    fn test_deposit_no_overdraft() {
        let mut runner = Runner::new();
        runner.run_row(Row{
            tx_type: Type::Deposit,
            client: 1,
            tx: 1,
            amount: Amount::Some(5.0)
        });
        runner.run_row(Row{
            tx_type: Type::Withdrawal,
            client: 1,
            tx: 2,
            amount: Amount::Some(6.0)
        });
        let Output(output_rows) = runner.output();
        let available = &output_rows[0].available;
        assert_eq!(available, "5.0000");
    }

    #[test]
    fn test_withdrawal() {
        let mut runner = Runner::new();
        runner.run_row(Row{
            tx_type: Type::Deposit,
            client: 1,
            tx: 1,
            amount: Amount::Some(5.0)
        });
        runner.run_row(Row{
            tx_type: Type::Withdrawal,
            client: 1,
            tx: 2,
            amount: Amount::Some(5.0)
        });
        let Output(output_rows) = runner.output();
        let available = &output_rows[0].available;
        assert_eq!(available, "0.0000");
    }

    #[test]
    fn test_dispute() {
        let mut runner = Runner::new();
        runner.run_row(Row{
            tx_type: Type::Deposit,
            client: 1,
            tx: 1,
            amount: Amount::Some(5.0)
        });
        runner.run_row(Row{
            tx_type: Type::Dispute,
            client: 1,
            tx: 1,
            amount: Amount::None
        });
        let Output(output_rows) = runner.output();
        let available = &output_rows[0].available;
        assert_eq!(available, "0.0000");
        let held = &output_rows[0].held;
        assert_eq!(held, "5.0000");
    }

    #[test]
    fn test_dispute_resolve() {
        let mut runner = Runner::new();
        runner.run_row(Row{
            tx_type: Type::Deposit,
            client: 1,
            tx: 1,
            amount: Amount::Some(5.0)
        });
        runner.run_row(Row{
            tx_type: Type::Dispute,
            client: 1,
            tx: 1,
            amount: Amount::None
        });
        runner.run_row(Row{
            tx_type: Type::Resolve,
            client: 1,
            tx: 1,
            amount: Amount::None
        });
        runner.run_row(Row{
            tx_type: Type::Deposit,
            client: 1,
            tx: 1,
            amount: Amount::Some(3.0)
        });
        let Output(output_rows) = runner.output();
        let available = &output_rows[0].available;
        assert_eq!(available, "8.0000");
        let held = &output_rows[0].held;
        assert_eq!(held, "0.0000");
        let locked = &output_rows[0].locked;
        assert_eq!(locked, &false);
    }

    #[test]
    fn test_dispute_chargeback() {
        let mut runner = Runner::new();
        runner.run_row(Row{
            tx_type: Type::Deposit,
            client: 1,
            tx: 1,
            amount: Amount::Some(5.0)
        });
        runner.run_row(Row{
            tx_type: Type::Dispute,
            client: 1,
            tx: 1,
            amount: Amount::None
        });
        runner.run_row(Row{
            tx_type: Type::Chargeback,
            client: 1,
            tx: 1,
            amount: Amount::None
        });
        runner.run_row(Row{
            tx_type: Type::Deposit,
            client: 1,
            tx: 1,
            amount: Amount::Some(5.0)
        });
        let Output(output_rows) = runner.output();
        let available = &output_rows[0].available;
        assert_eq!(available, "0.0000");
        let held = &output_rows[0].held;
        assert_eq!(held, "0.0000");
        let locked = &output_rows[0].locked;
        assert_eq!(locked, &true);
    }
}
