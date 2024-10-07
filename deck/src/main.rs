use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)] // This is a derive attribute. This enhances the functionality of our struct.
struct Deck {
    cards: Vec<String>,
}

/*
    ### Associated functions and Methods ###
    - Associated functions are functions that are associated with the struct itself, not instances of the struct.
    - Methods are functions that are associated with instances of the struct.
*/

// Inherent implementation block
impl Deck {
    // We can also use '-> Deck' instead of '-> Self' in the function signature
    fn new() -> Self {
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"]; // List of 'suits' - 'hearts', 'diamonds', 'clubs', 'spades'
        let values = ["Ace", "Two", "Three"]; // List of 'values' - 'ace', 'two' etc.

        // Variables are 'bindings' in Rust
        let mut cards = Vec::new();
        // Without 'mut' we can't reassign or change the value of bindings
        // Vec::new() is the same as vec![]

        // Double nested for loop to create a deck of cards
        // for each suit in suits
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // Implicit return - rust automatically returns the last expression in a block (without a semicolon!)
        Deck { cards }
    }

    // Crate == Package
    fn shuffle(&mut self) {
        let mut rng = thread_rng(); // thread_rng() is a function that returns a random number generator
        self.cards.shuffle(&mut rng); // shuffle() is a method that shuffles the deck
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        let cards_to_keep = self.cards.len() - num_cards;
        self.cards.split_off(cards_to_keep)
    }
}

fn main() {
    let mut deck: Deck = Deck::new();

    // Shuffle the deck
    deck.shuffle();

    // TODO: Need to add error handling; try using 100 instead of 3 => deal(100)
    let cards = deck.deal(3);

    println!("Here's your hand: {:#?}", cards);
    println!("Here's your deck: {:#?}", deck.cards);
}
