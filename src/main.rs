use std::io;

enum AccountStatus {
    Active,
    Frozen,
}

enum TransactionKind {
    Deposit,
    Withdraw,
    TransferIn,
    TransferOut,
}

enum BankError {
    AccountNotFound,
    InsufficientFunds,
    InvalidAmount,
    AccountFrozen,
    SameAccount,
}

struct Account {
    id: u64,
    owner_name: String,
    balance: i64,
    status: AccountStatus,
}

struct Transaction {
    account_id: u64,
    kind: TransactionKind,
    amount: i64,
}

fn status_to_str(status: &AccountStatus) -> &str {
    match status {
        AccountStatus::Active => "Active",
        AccountStatus::Frozen => "Frozen",
    }
}

fn kind_to_str(kind: &TransactionKind) -> &str {
    match kind {
        TransactionKind::Deposit => "Deposit",
        TransactionKind::Withdraw => "Withdraw",
        TransactionKind::TransferIn => "TransferIn",
        TransactionKind::TransferOut => "TransferOut",
    }
}

fn print_error(err: BankError) {
    match err {
        BankError::AccountNotFound => println!("Error: Account not found."),
        BankError::InsufficientFunds => println!("Error: Insufficient funds."),
        BankError::InvalidAmount => println!("Error: Invalid amount."),
        BankError::AccountFrozen => println!("Error: Account is frozen."),
        BankError::SameAccount => println!("Error: Source and destination accounts cannot be the same."),
    }
}

fn read_string() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(_) => String::new(),
    }
}

fn read_u64() -> Option<u64> {
    let input = read_string();
    match input.parse::<u64>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

fn read_i64() -> Option<i64> {
    let input = read_string();
    match input.parse::<i64>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}


fn main() {}