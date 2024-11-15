use rand::{seq::SliceRandom, thread_rng};

// This is a derive attribute. This enhances the
// functionality of our struct.
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

/*
    ### Associated functions and methods ###
        - Associated functions are functions that are
        associated with the struct itself, not instances
        of the struct.
        - Methods are functions that are associated with
        instances of the struct.
*/

// Inherent implementation block
impl Deck {
    // We can also use '-> Deck' instead of '-> Self' in
    // the function signature.
    // 'new' is an associated function
    fn new() -> Self {
        // List of suits
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];

        // List of values
        let values = ["Ace", "Two", "Three"];

        // Variables are 'bindings' in Rust
        let mut cards = Vec::new();
        // Without 'mut' we can't reassign or change the
        // value of bindings.
        // Vec::new() is the same as vec![]

        // Double nested for loop to create a deck of cards
        // for each suit in suits
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // Implicit return - Rust automatically returns
        // the last expression in a block (without a
        // semicolon!)
        Deck { cards }
    }

    // Crate == Package
    // 'shuffle' is a method
    fn shuffle(&mut self) {
        // thread_rng() is a function that returns a random
        // number generator
        let mut rng = thread_rng();

        // shuffle() is a method that shuffles the deck
        self.cards.shuffle(&mut rng);
    }

    // 'deal' is a method
    fn deal(&mut self, num_cards_to_remove: usize) -> Vec<String> {
        let removal_idx = self.cards.len() - num_cards_to_remove;
        self.cards.split_off(removal_idx)
    }
}

fn main() {
    let mut deck = Deck::new();

    // Shuffle the deck
    deck.shuffle();

    // TODO: Need to add error handling; try using 100
    // instead of 3 => deal(100)
    let cards_via_deal = deck.deal(3);

    println!("Here's your hand: {:#?}", cards_via_deal);
    println!("Here's your deck: {:#?}", deck.cards);
}
