#[derive(Debug)]
enum Media {
    // AudioBook, Book, Movie etc. are variants of Media
    AudioBook { title: String },
    Book { author: String, title: String },
    Movie { director: String, title: String },
    // Podcast { episode_number: u32, episode_name: String },
    Podcast(u32, String), // Unnamed fields - This will probably confuse others
    Placeholder,
}

// Inherent implementation
impl Media {
    fn description(&self) -> String {
        // ### Method 1 - Tedious ###
        // if let Media::AudioBook { title } = self {
        //     format!("AudioBook: {}", title)
        // } else if let Media::Book { author, title } = self {
        //     format!("Book: {} - {}", title, author)
        // } else if let Media::Movie { director, title } = self {
        //     format!("Movie: {} - {}", title, director)
        // } else if let Media::Podcast(episode_number, episode_name) = self {
        //     format!("Podcast: {} - {}", episode_number, episode_name)
        // } else {
        //     String::from("Media description")
        // }

        // ### Method 2 - Pattern matching ###
        match self {
            Media::AudioBook { title } => {
                format!("AudioBook: {}", title)
            }
            Media::Book { author, title } => {
                format!("Book: {} - {}", title, author)
            }
            Media::Movie { director, title } => {
                format!("Movie: {} - {}", title, director)
            }
            // We can use any name instead of 'episode_number' and 'episode_name' since we are using unnamed fields in 'Podcast' variant of Media
            Media::Podcast(episode_number, episode_name) => {
                format!("Podcast: ({}.) {}", episode_number, episode_name)
            }
            Media::Placeholder => {
                format!("Placeholder!")
            }
        }
    }
}

// TODO: `'a` is a lifetime parameter. What is that?
enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable,
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: Vec::new() }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media); // We are taking ownership of 'media' here
    }

    fn get_by_index(&self, index: usize) -> MightHaveAValue {
        if self.items.len() > index {
            // Something to return
            MightHaveAValue::ThereIsAValue(&self.items[index]) // We don't want to transfer ownership, hence the use of '&'
        } else {
            // Nothing to return
            MightHaveAValue::NoValueAvailable
        }
    }

    fn get_by_index_new(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        }
    }
}

// Immutable reference of Media
fn print_media(media: &Media) {
    println!("{:#?}", media);
}

#[derive(Debug)]
struct Employee {
    name: String,
}

#[derive(Debug)]
struct Task {
    assigned_to: Option<Employee>,
}

fn main() {
    let any_audio_book = Media::AudioBook {
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
        name: String::from("Who Cares"),
    };

    let task_1 = Task {
        assigned_to: Some(employee1),
    };

    let task_2 = Task { assigned_to: None };

    println!("Task 1: {:#?}", task_1);

    println!("");

    println!("Task 2: {:#?}", task_2);

    // Exercise link: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=8cab45161489fe0a2ad027d5222cb3fa
}
