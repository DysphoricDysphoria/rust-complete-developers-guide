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

    fn summary(&self) -> String {
        format!(
            "{} (AccID: {}) has balance {}",
            self.holder, self.id, self.balance
        )
    }

    // TODO: What if 'amount' is negative?
    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
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
            accounts: Vec::new(), // Can also use 'vec![]'
        }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        // ### Method 1 ###
        // let mut total = 0;
        // for acc in &self.accounts {
        //     total += acc.balance;
        // }
        // total

        // ### Method 2 ###
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        // ### Method 1 ###
        // let mut summary = Vec::new();
        // for acc in &self.accounts {
        //     summary.push(acc.summary());
        // }
        // summary

        // ### Method 2 ###
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
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

// NOTE: Try using &Account and return &account => !Error
fn make_and_print_account() -> Account {
    let account: Account = Account::new(1, String::from("John Davis"));

    println!("{:#?}", account);

    // !Error - Can't return a reference to account which
    // is owned by the current function
    // &account

    account
}

fn ref_demo_0() {
    let account = Account::new(1, String::from("Johnny Wick"));

    // Multiple immutable references can co-exist for a
    // value
    let account_ref_1 = &account;
    let account_ref_2 = &account;

    // !Error: Can't use immutable reference to update
    // value
    // account_ref_1.balance = 100;

    // !Error: Can't move a value while reference(s) to
    // the value exist (Caveat: Can move if the reference
    // isn't used later on)
    // let other_account = account;

    print_account(account_ref_1);
    print_account(account_ref_2);

    println!("{:#?}", account);
}

fn ref_demo_1() {
    let bank = Bank::new();
    let account = Account::new(1, String::from("Jamie Bravo"));

    // Immutable reference
    let account_ref = &account;

    print_account(account_ref);

    // Immutable reference
    let bank_accounts_ref = &bank.accounts;

    print_account_list_with_ref(bank_accounts_ref);

    /*
        ### QUESTION ###
            The rule is "You can't move a value while a ref
            to the value exists."; Reference to
            bank.accounts exist & still we are able to move
            it?

            ### Answer ###
            Rust allows you to borrow part of a struct
            (like a field) and later move the entire field
            out of the struct, as long as you DON'T USE THE
            REFERENCE AFTER THE MOVE.
    */
    print_account_list_without_ref(bank.accounts);

    // !Error: Can't use the reference once a value
    // (bank.accounts) has been moved
    // print_account_list_with_ref(bank_accounts_ref);

    // !Error: borrow of partially moved value: `bank`
    // println!("{:#?}", bank);
}

fn ref_demo_2() {
    let mut account = Account::new(1, String::from("Betty Davis"));

    // !Error: We can't have a read-only (immutable)
    // reference if we have a mutable reference and
    // vice-versa; depends on what is initialized
    // and used first
    // let account_ref_read_only = &account;

    // Mutable reference
    let account_ref_mutable_1 = &mut account;

    // !Error: Can't modify a binding via its owner if we
    // have a reference (mutable or immutable)
    // account.balance = 100;

    // !Error: We can't have another mutable reference if
    // we already have one
    // let account_ref_mutable_2 = &mut account;

    change_account(account_ref_mutable_1);

    // Here we are using a immutable reference although
    // we already have a mutable reference
    // println!("{:#?}", account_ref_read_only.holder);
}

fn ref_demo_3() {
    let num = 5;

    // Ownership isn't transferred from num to other_num
    let other_num = num;
    println!("{} {}", num, other_num);
}

fn lifetimes_demo_0() {
    let _account = make_and_print_account();
}

fn demos() {
    ref_demo_0();

    ref_demo_1();

    ref_demo_2();

    ref_demo_3();

    lifetimes_demo_0();
}

fn main() {
    demos();

    let mut bank = Bank::new();

    // Accounts
    let mut account_1 = Account::new(1, String::from("John Wick"));
    let mut account_2 = Account::new(2, String::from("John McClane"));
    let mut account_3 = Account::new(3, String::from("Frank Castle"));

    // Ops on account_1
    account_1.deposit(100);
    account_1.withdraw(80);
    println!("{}\n", account_1.summary());

    // Deposit to account_2
    account_2.deposit(10);

    // Deposit to account_3
    account_3.deposit(30);

    // Add accounts
    bank.add_account(account_1);
    bank.add_account(account_2);
    bank.add_account(account_3);

    println!("Total balance in bank: {}\n", bank.total_balance());
    println!("Account summaries: {:#?}\n", bank.summary());

    println!("{:#?}", bank);
}

/*
    ### ### ### ### #
    ### Ownership ###
    ### ### ### ### #
    - The goal of ownership is to limit the ways you can
    reference and change data.
        > Lesson 1: Multiple things can refer to a value
        at the same time, but they are all read-only
        > Lesson 2: A value can only be updated when there
        are no read-only/write references to it
*/
