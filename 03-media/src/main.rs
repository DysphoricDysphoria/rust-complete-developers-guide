mod content;

use content::catalog::Catalog;
use content::catalog::MightHaveAValue;
use content::employee::Employee;
use content::media::Media;
use content::task::Task;

// Immutable reference of Media
fn print_media(media: &Media) {
    println!("{:#?}", media);
}

fn main() {
    // In the next line instead of 'content::media::Media::AudioBook' we can also use 'Media::AudioBook'
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
    catalog.add(any_placeholder);
    catalog.add(any_podcast);

    println!("{:#?}", catalog);
    println!("{:#?}", catalog.items.get(4)); // Indexing into 'items' - 'Some'; 'get' is a method on vectors
    println!("{:#?}", catalog.items.get(100)); // 'None'
    println!("");

    /*
        ### Notes ###
        - Rust doesn't have null, nil, or undefined
        - We get a built-in enum called 'Option' => Has two variants - 'Some' and 'None'
        - If you want to work with Option you have to use pattern matching (the 'if let' thing) or a 'match' statement
            - FORCES you to handle the case in which you have a value and the case in which you don't
    */

    let item_0 = catalog.items.get(10);
    match item_0 {
        Option::Some(value) => {
            // Here we can add another 'match' statement that checks the
            // type of media and prints stuff accordingly. Kinda like what
            // we have in the 'description' method of Media
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

    let item_3 = catalog.get_by_index(30);
    /*
        ### if let ###
        - Type Assertion: Rust does not have explicit type assertions like some other languages (e.g., TypeScript). Instead, it uses pattern matching to ensure that the type and structure of the data match the expected pattern.

        - Comparison: This is not a comparison in the traditional sense (like ==). It's a pattern match that checks if 'item_2' is of a specific enum variant and, if so, extracts the contained value.
    */
    if let MightHaveAValue::ThereIsAValue(value) = item_3 {
        println!("Item in pattern match: {:#?}", value);
    } else {
        println!("Got no value!");
    }

    println!("");

    let item_4 = catalog.get_by_index_new(10);
    if let Some(value) = item_4 {
        println!("Item in pattern match (if let): {:#?}", value)
    } else {
        println!("Got no value (if let)!");
    }

    println!("");

    /*
        ### Handling options ###
        - unwrap
            - If 'item' is a Some, returns the value in the Some
            - If 'item' is a None, panics!
            - Use for quick debugging examples
        - expect
            - If 'item' is a Some, returns the value in the Some
            - If 'item' is a None, prints the provided debug message
            and panics!
            - Use when we want to crash if there is no value
        - unwrap_or
            - If 'item' is a Some, returns the value in the Some
            - If 'item' is a None, returns the provided default value
            - Use when it makes sense to provide a fallback value
        - Documentation
            - https://doc.rust-lang.org/std/option/enum.Option.html
    */
    let item_for_unwrap = catalog.get_by_index_new(0);
    let placeholder = Media::Placeholder;

    // println!("{:#?}", item_for_unwrap.unwrap())
    // println!("{:#?}", item_for_unwrap.expect("no item found!"))
    println!("{:#?}", item_for_unwrap.unwrap_or(&placeholder));

    println!("");

    let employee1 = Employee {
        name: String::from("Mob Boss"),
    };

    let t_1 = Task {
        assigned_to: Some(employee1),
    };

    let t_2 = Task { assigned_to: None };

    println!("t_1: {:#?}", t_1);

    println!("");

    println!(
        "t_1.assigned_to.unwrap().name: {:#?}",
        t_1.assigned_to.unwrap().name
    );

    println!("");

    println!("t_2.assigned_to: {:#?}", t_2.assigned_to);

    // Exercise link: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=8cab45161489fe0a2ad027d5222cb3fa
}
