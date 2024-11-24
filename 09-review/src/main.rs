mod content;

use content::catalog::{Catalog, MightHaveAValue};
use content::media::Media;

// Immutable reference of Media
fn print_media(media: &Media) {
    println!("{:#?}", media);
}

fn main() {
    // In the next line instead of
    // 'content::media::Media::AudioBook' we can also use
    // 'Media::AudioBook'
    let any_audio_book = content::media::Media::AudioBook {
        title: String::from("Who will cry when you will die?"),
    };
    let any_book = Media::Book {
        author: String::from("An author"),
        title: String::from("A book"),
    };
    let any_movie = Media::Movie {
        director: String::from("A director"),
        title: String::from("Interstellar"),
    };
    let any_podcast = Media::Podcast(101, String::from("A Brief History of Nearly Everything"));
    let any_placeholder = Media::Placeholder;

    println!("{}", any_audio_book.description());
    println!("{}", any_book.description());
    println!("{}", any_movie.description());
    println!("{}", any_podcast.description());
    println!("{}", any_placeholder.description());

    println!("");

    print_media(&any_audio_book);
    print_media(&any_book);
    print_media(&any_movie);
    print_media(&any_podcast);
    print_media(&any_placeholder);

    println!("");

    let mut catalog = Catalog::new();

    catalog.add(any_audio_book);
    catalog.add(any_book);
    catalog.add(any_movie);
    catalog.add(any_podcast);
    catalog.add(any_placeholder);

    println!("{:#?}", catalog);

    // Indexing into 'items' - 'Some'; 'get' is a method
    // on vectors
    println!("{:#?}", catalog.items.get(4));

    // 'None'
    println!("{:#?}", catalog.items.get(100));

    println!("");

    /*
        ### Notes ###
        - Rust doesn't have null, nil, or undefined
        - We get a built-in enum called 'Option' => Has
        two variants - 'Some' and 'None'
        - If you want to work with Option you have to use
        pattern matching (the 'if let' thing) or a 'match'
        statement
            - FORCES you to handle the case in which you
            have a value and the case in which you don't
    */

    let item_0 = catalog.items.get(10);
    match item_0 {
        Option::Some(value) => {
            // Here we can add another 'match' statement
            // that checks the type of media and prints
            // stuff accordingly. Kinda like what we have
            // in the 'description' method of Media
            println!("Item: {:#?}", value);
        }
        Option::None => {
            println!("Nothing at that index");
        }
    }

    println!("");

    let item_1 = catalog.get_by_index(20);
    match item_1 {
        MightHaveAValue::ThereIsAValue(value) => {
            println!("Item: {:#?}", value)
        }
        MightHaveAValue::NoValueAvailable => {
            println!("No value available")
        }
    }

    println!("");

    let item_2 = catalog.get_by_index_new(9999);
    match item_2 {
        Option::Some(value) => {
            println!("Item (match): {:#?}", value)
        }
        Option::None => {
            println!("No value available (match)")
        }
    }

    println!("");
}
