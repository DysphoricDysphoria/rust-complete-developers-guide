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
    // We can also use 'Deck' instead of 'Self' in the function signature
    fn new() -> Self {
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"]; // List of 'suits' - 'hearts', 'diamonds', 'clubs', 'spades'
        let values = ["Ace", "Two", "Three"]; // List of 'values' - 'ace', 'two' etc.

        let mut cards = vec![]; // Without 'mut' we can't reassign or change the value of bindings

        // Double nested for loop to create a deck of cards
        // for each suit in suits
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // Variables are 'bindings' in Rust
        let deck: Deck = Deck { cards }; // Vec::new() is the same as vec![]

        return deck;
    }
}

fn main() {
    let deck = Deck::new();

    println!("Here's your deck: {:#?}", deck);
}
