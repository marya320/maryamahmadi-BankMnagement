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

fn main() {}