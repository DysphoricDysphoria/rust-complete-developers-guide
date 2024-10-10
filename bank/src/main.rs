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

// This func accepts a reference
fn print_account(account: &Account) {
    println!("{:#?}", account);
}

// This func accepts a reference
fn print_account_list_with_ref(accounts: &Vec<Account>) {
    println!("{:#?}", accounts);
}

// This func accepts a value
fn print_account_list_without_ref(accounts: Vec<Account>) {
    println!("{:#?}", accounts);
}

// This func accepts a mutable reference
fn change_account(account: &mut Account) {
    account.balance = 10;
}

fn ref_demo_0() {
    let account = Account::new(1, String::from("John Wick 1"));

    // Multiple immutable references can co-exist for a value
    let account_ref_1 = &account;
    let account_ref_2 = &account;

    // account_ref_1.balance = 100; // !Error: Can't use immutable reference to update value

    // let other_account = account; // !Error: Can't move a value while reference(s) to the value exist (Caveat: Can move if the reference isn't used later on)

    print_account(account_ref_1);
    print_account(account_ref_2);

    println!("{:#?}", account);
}

fn ref_demo_1() {
    let bank = Bank::new();
    let account = Account::new(1, String::from("John Wick 2"));

    let account_ref = &account;

    print_account(account_ref);

    let bank_accounts_ref = &bank.accounts;

    print_account_list_with_ref(bank_accounts_ref);

    /*
        >>> QUESTION: The rule is "You can't move a value while a ref to the value exists."; Reference to bank.accounts exist still we are able to move it?
            Answer: Rust allows you to borrow part of a struct (like a field) and later move the entire field out of the struct, as long as you DON'T USE THE REFERENCE AFTER THE MOVE.
    */
    print_account_list_without_ref(bank.accounts);

    // print_account_list_with_ref(bank_accounts_ref); // !Error: Can't use the reference once a value (bank.accounts) has been moved

    // println!("{:#?}", bank); // !Error: borrow of partially move value: `bank`
}

fn ref_demo_2() {
    let mut account = Account::new(1, String::from("John Wick 3"));

    // let account_ref_read_only = &account; // !Error: We can't have a read-only (immutable) reference if we have a mutable reference

    // Mutable reference
    let account_ref_mutable_1 = &mut account;

    // account.balance = 100; // !Error: Can't modify a binding via its owner if we have a reference (mutable or immutable)

    // let account_ref_mutable_2 = &mut account; // !Error: We can't have another mutable reference if we already have one

    change_account(account_ref_mutable_1);

    // println!("{:#?}", account_ref_read_only.holder); // Here we are using a immutable reference although we already have a mutable reference
}

fn main() {
    ref_demo_0();

    ref_demo_1();

    ref_demo_2();
}

/*
    ### Ownership ###
    - The goal of ownership is to limit the ways you can reference and change data.
        > Lesson 1: Multiple things can refer to a value at the same time, but they are all read-only
        > Lesson 2: A value can only be updated when there are no read-only references to it
*/

/*
    ### ### ### ### ### ### #
    ### Rules - Ownership ###
    ### ### ### ### ### ### #
        1. Every value is 'owned' by a single variable (binding), struct, vector, etc. at a time
        2. Reassigning the value to a variable (binding), passing it to a function, putting it into a vector, etc. moves the value. The old owner can't be used to access the value anymore!

    ### ### ### ### ### ### #
    ### Rules - Borrowing ###
    ### ### ### ### ### ### #
        3. You can create many read-only (immutable) references to a value. These refs can all exist at the same time.
        4. You can't move a value while a ref to the value exists. (Caveat: Can move if the reference isn't used later on)

        5. You can make a writeable (mutable) reference to a value only if there are no read-only references currently in use. One mutable ref to a value can exist at a time (although we can have numerous read-only references).
        6. You can't mutate a value through the owner when any ref (mutable or immutable) to the value exists.
*/
