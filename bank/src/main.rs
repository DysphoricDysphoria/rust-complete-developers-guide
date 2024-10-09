#[derive(Debug)]
struct Account {
    balance: i32,
    holder: String,
    id: u32,
}

// Inherent implementation
impl Account {
    fn new(id: u32, holder: String) -> Self {
        // Implicit return
        Account {
            balance: 0,
            holder,
            id,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

// Inherent implementation
impl Bank {
    fn new() -> Self {
        Bank {
            accounts: Vec::new(), // vec![]
        }
    }
}

fn print_account(account: Account) -> Account {
    println!("{:#?}", account);
    account
}

fn main() {
    let mut account = Account::new(1, String::from("John Wick"));

    account = print_account(account);
    account = print_account(account);

    println!("{:#?}", account);
}

/*
    ### Ownership ###
    - The goal of ownership is to limit the ways you can reference and change data.
        > Lesson 1: Multiple things can refer to a value at the same time, but they are all read-only
        > Lesson 2: A value can only be updated when there are no read-only references to it
*/

/*
    ### Rules ###
    1. Every value is 'owned' by a single variable (binding), struct, vector, etc. at a time
    2. Reassigning the value to a variable (binding), passing it to a function, putting it into a vector, etc. moves the value. The old owner can't be used to access the value anymore!
*/
