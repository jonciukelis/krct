# Large octopus rust coding test 

Project takes a list of transaction CSV file.

Each transaction has the following contents:
- Type
- Client - u16 
- TX - u32
- Amount - f64

Supported transaction types: 
- Deposit
- Withdrawal
- Dispute
- Resolve
- Chargeback

Dispute, Resolve and Chargeback do not have and amount specified.

Output format is:
- Client
- Available Funds (Undisputed)
- Held Funds (Disputed)
- Total Funds
- Locked (For a rightful Dispute)
## Prerequisites

- Rust with cargo

## Getting Started

- Start the app:
    ```
    $ cargo run -- <filename.csv>
    ```
- Linting:
    ```
    $ cargo clippy
    ```

## To get to production

- Unit and Integration tests are missing.
- Async the runner
- CI/CD is missing
- Logging. There's a lot of soft failures happening unreported.
 
