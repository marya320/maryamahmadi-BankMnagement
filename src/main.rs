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

fn create_account(
    accounts: &mut Vec<Account>,
    next_id: &mut u64,
    owner_name: String,
    initial_balance: i64,
) -> Result<u64, BankError> {
    if initial_balance < 0 {
        return Err(BankError::InvalidAmount);
    }

    let id = *next_id;
    *next_id += 1;

    let new_account = Account {
        id,
        owner_name,
        balance: initial_balance,
        status: AccountStatus::Active,
    };

    accounts.push(new_account);
    Ok(id)
}

fn list_accounts(accounts: &Vec<Account>) {
    if accounts.is_empty() {
        println!("No accounts found.");
        return;
    }

    for account in accounts {
        println!(
            "ID: {} | Owner: {} | Balance: {} Rial | Status: {}",
            account.id,
            account.owner_name,
            account.balance,
            status_to_str(&account.status)
        );
    }
}

fn find_account(accounts: &Vec<Account>, id: u64) -> Option<&Account> {
    for account in accounts {
        if account.id == id {
            return Some(account);
        }
    }
    None
}

fn deposit(
    accounts: &mut Vec<Account>,
    transactions: &mut Vec<Transaction>,
    id: u64,
    amount: i64,
) -> Result<(), BankError> {
    if amount <= 0 {
        return Err(BankError::InvalidAmount);
    }

    for account in accounts {
        if account.id == id {
            account.balance += amount;
            transactions.push(Transaction {
                account_id: id,
                kind: TransactionKind::Deposit,
                amount,
            });
            return Ok(());
        }
    }

    Err(BankError::AccountNotFound)
}

fn withdraw(
    accounts: &mut Vec<Account>,
    transactions: &mut Vec<Transaction>,
    id: u64,
    amount: i64,
) -> Result<(), BankError> {
    if amount <= 0 {
        return Err(BankError::InvalidAmount);
    }

    for account in accounts {
        if account.id == id {
            match account.status {
                AccountStatus::Frozen => return Err(BankError::AccountFrozen),
                AccountStatus::Active => {}
            }

            if account.balance < amount {
                return Err(BankError::InsufficientFunds);
            }

            account.balance -= amount;
            transactions.push(Transaction {
                account_id: id,
                kind: TransactionKind::Withdraw,
                amount,
            });
            return Ok(());
        }
    }

    Err(BankError::AccountNotFound)
}

fn transfer(
    accounts: &mut Vec<Account>,
    transactions: &mut Vec<Transaction>,
    from_id: u64,
    to_id: u64,
    amount: i64,
) -> Result<(), BankError> {
    if amount <= 0 {
        return Err(BankError::InvalidAmount);
    }

    if from_id == to_id {
        return Err(BankError::SameAccount);
    }

    let mut from_index: Option<usize> = None;
    let mut to_index: Option<usize> = None;

    for i in 0..accounts.len() {
        if accounts[i].id == from_id {
            from_index = Some(i);
        }
        if accounts[i].id == to_id {
            to_index = Some(i);
        }
    }

    let from_idx = match from_index {
        Some(i) => i,
        None => return Err(BankError::AccountNotFound),
    };

    let to_idx = match to_index {
        Some(i) => i,
        None => return Err(BankError::AccountNotFound),
    };

    match accounts[from_idx].status {
        AccountStatus::Frozen => return Err(BankError::AccountFrozen),
        AccountStatus::Active => {}
    }

    if accounts[from_idx].balance < amount {
        return Err(BankError::InsufficientFunds);
    }

    accounts[from_idx].balance -= amount;
    accounts[to_idx].balance += amount;

    transactions.push(Transaction {
        account_id: from_id,
        kind: TransactionKind::TransferOut,
        amount,
    });

    transactions.push(Transaction {
        account_id: to_id,
        kind: TransactionKind::TransferIn,
        amount,
    });

    Ok(())
}

fn toggle_freeze(accounts: &mut Vec<Account>, id: u64) -> Result<&AccountStatus, BankError> {
    for account in accounts {
        if account.id == id {
            match account.status {
                AccountStatus::Active => account.status = AccountStatus::Frozen,
                AccountStatus::Frozen => account.status = AccountStatus::Active,
            }
            return Ok(&account.status);
        }
    }

    Err(BankError::AccountNotFound)
}

fn show_history(accounts: &Vec<Account>, transactions: &Vec<Transaction>, id: u64) {
    match find_account(accounts, id) {
        None => {
            println!("Account not found.");
            return;
        }
        Some(_) => {}
    }

    let mut found = false;
    for tx in transactions {
        if tx.account_id == id {
            println!("Type: {} | Amount: {} Rial", kind_to_str(&tx.kind), tx.amount);
            found = true;
        }
    }

    if !found {
        println!("No transactions found for this account.");
    }
}

fn show_statistics(accounts: &Vec<Account>, transactions: &Vec<Transaction>) {
    let total_accounts = accounts.len();
    let mut active_accounts = 0;
    let mut frozen_accounts = 0;
    let mut total_balance: i64 = 0;

    for account in accounts {
        match account.status {
            AccountStatus::Active => active_accounts += 1,
            AccountStatus::Frozen => frozen_accounts += 1,
        }
        total_balance += account.balance;
    }

    let total_transactions = transactions.len();

    println!("Total accounts: {}", total_accounts);
    println!("Active accounts: {}", active_accounts);
    println!("Frozen accounts: {}", frozen_accounts);
    println!("Total balance: {} Rial", total_balance);
    println!("Total transactions: {}", total_transactions);
}


fn main() {
    let mut accounts: Vec<Account> = Vec::new();
    let mut transactions: Vec<Transaction> = Vec::new();
    let mut next_id: u64 = 1;

    loop {
        println!("\n--- Bank Account Management ---");
        println!("1. Create Account");
        println!("2. List Accounts");
        println!("3. Find Account by ID");
        println!("4. Deposit");
        println!("5. Withdraw");
        println!("6. Transfer");
        println!("7. Freeze / Unfreeze Account");
        println!("8. Transaction History");
        println!("9. Bank Statistics");
        println!("10. Exit");
        println!("Enter your choice:");

        let choice = read_string();

        match choice.as_str() {
            "1" => {
                println!("Enter owner name:");
                let owner_name = read_string();
                if owner_name.is_empty() {
                    println!("Owner name cannot be empty.");
                    continue;
                }

                println!("Enter initial balance (Rial):");
                let initial_balance = match read_i64() {
                    Some(val) => val,
                    None => {
                        println!("Invalid balance.");
                        continue;
                    }
                };

                match create_account(&mut accounts, &mut next_id, owner_name, initial_balance) {
                    Ok(id) => println!("Account created successfully with ID: {}", id),
                    Err(err) => print_error(err),
                }
            }
            "2" => {
                list_accounts(&accounts);
            }
            "3" => {
                println!("Enter account ID:");
                let id = match read_u64() {
                    Some(val) => val,
                    None => {
                        println!("Invalid account ID.");
                        continue;
                    }
                };

                match find_account(&accounts, id) {
                    Some(account) => {
                        println!(
                            "ID: {} | Owner: {} | Balance: {} Rial | Status: {}",
                            account.id,
                            account.owner_name,
                            account.balance,
                            status_to_str(&account.status)
                        );
                    }
                    None => println!("Account not found."),
                }
            }
            "4" => {
                println!("Enter account ID:");
                let id = match read_u64() {
                    Some(val) => val,
                    None => {
                        println!("Invalid account ID.");
                        continue;
                    }
                };

                println!("Enter deposit amount (Rial):");
                let amount = match read_i64() {
                    Some(val) => val,
                    None => {
                        println!("Invalid amount.");
                        continue;
                    }
                };

                match deposit(&mut accounts, &mut transactions, id, amount) {
                    Ok(()) => println!("Deposit successful."),
                    Err(err) => print_error(err),
                }
            }
            "5" => {
                println!("Enter account ID:");
                let id = match read_u64() {
                    Some(val) => val,
                    None => {
                        println!("Invalid account ID.");
                        continue;
                    }
                };

                println!("Enter withdraw amount (Rial):");
                let amount = match read_i64() {
                    Some(val) => val,
                    None => {
                        println!("Invalid amount.");
                        continue;
                    }
                };

                match withdraw(&mut accounts, &mut transactions, id, amount) {
                    Ok(()) => println!("Withdrawal successful."),
                    Err(err) => print_error(err),
                }
            }
            "6" => {
                println!("Enter source account ID:");
                let from_id = match read_u64() {
                    Some(val) => val,
                    None => {
                        println!("Invalid source account ID.");
                        continue;
                    }
                };

                println!("Enter destination account ID:");
                let to_id = match read_u64() {
                    Some(val) => val,
                    None => {
                        println!("Invalid destination account ID.");
                        continue;
                    }
                };

                println!("Enter transfer amount (Rial):");
                let amount = match read_i64() {
                    Some(val) => val,
                    None => {
                        println!("Invalid amount.");
                        continue;
                    }
                };

                match transfer(&mut accounts, &mut transactions, from_id, to_id, amount) {
                    Ok(()) => println!("Transfer successful."),
                    Err(err) => print_error(err),
                }
            }
            "7" => {
                println!("Enter account ID:");
                let id = match read_u64() {
                    Some(val) => val,
                    None => {
                        println!("Invalid account ID.");
                        continue;
                    }
                };

                match toggle_freeze(&mut accounts, id) {
                    Ok(new_status) => println!("Account status changed to: {}", status_to_str(new_status)),
                    Err(err) => print_error(err),
                }
            }
            "8" => {
                println!("Enter account ID:");
                let id = match read_u64() {
                    Some(val) => val,
                    None => {
                        println!("Invalid account ID.");
                        continue;
                    }
                };

                show_history(&accounts, &transactions, id);
            }
            "9" => {
                show_statistics(&accounts, &transactions);
            }
            "10" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 10.");
            }
        }
    }
}