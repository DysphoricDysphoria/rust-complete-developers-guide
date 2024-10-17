#[derive(Debug)]
enum Media {
    // AudioBook, Book, Movie etc. are variants of Media
    AudioBook { title: String },
    Book { title: String, author: String },
    Movie { title: String, director: String },
    // Podcast { episode_number: u32 },
    Podcast(u32, String), // Unnamed field - This will probably confuse others
    Placeholder,
}

// Inherent implementation
impl Media {
    fn description(&self) -> String {
        // ### Method 1 - Tedious ###
        // if let Media::Book { title, author } = self {
        //     format!("Book: {} - {}", title, author)
        // } else if let Media::Movie { title, director } = self {
        //     format!("Movie: {} - {}", title, director)
        // } else if let Media::AudioBook { title } = self {
        //     format!("AudioBook: {}", title)
        // } else {
        //     String::from("Media description")
        // }

        // ### Method 2 - Pattern matching ###
        match self {
            Media::Book { title, author } => {
                format!("Book: {} - {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} - {}", title, director)
            }
            Media::AudioBook { title } => {
                format!("AudioBook: {}", title)
            }
            // We can use any name like 'episode_number', 'episode_name' etc.
            Media::Podcast(episode_number, episode_name) => {
                format!("Podcast: ({}.) {}", episode_number, episode_name)
            }
            Media::Placeholder => {
                format!("Placeholder!")
            }
        }
    }
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
        self.items.push(media); // We are taking ownership of media here
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
}

// TODO: `'a` is a lifetime parameter. What is that?
enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable,
}

// Immutable reference of Media
fn print_media(media: &Media) {
    println!("{:#?}", media);
}

fn main() {
    let a_book = Media::Book {
        title: String::from("A book"),
        author: String::from("An author"),
    };
    let a_movie = Media::Movie {
        title: String::from("Interstellar"),
        director: String::from("A director"),
    };
    let audio_book = Media::AudioBook {
        title: String::from("Who will cry when you will die"),
    };
    let placeholder = Media::Placeholder;
    let podcast = Media::Podcast(1, String::from("A Brief History Of Nearly Everything"));

    println!("{}", a_book.description());
    println!("{}", a_movie.description());
    println!("{}", audio_book.description());

    print_media(&a_book);
    print_media(&a_movie);
    print_media(&audio_book);

    let mut catalog = Catalog::new();

    catalog.add(a_book);
    catalog.add(a_movie);
    catalog.add(audio_book);
    catalog.add(placeholder);
    catalog.add(podcast);

    println!("{:#?}", catalog);
    println!("{:#?}", catalog.items.get(4)); // Indexing into 'items' - 'Some'; 'get' is a method on vectors
    println!("{:#?}", catalog.items.get(100)); // 'None'
    println!("");

    /*
        ### Notes ###
        - Rust doesn't have null, nil, or undefined
        - We get a built-in enum called 'Option' => Has two variants - 'Some' and 'None'
        - If you want to work with Option you have to use pattern matching (the 'if let' thing) or a 'match' statement
        - Forces you to handle the case in which you have a value and the case in which you don't

    */

    match catalog.items.get(100) {
        Option::Some(value) => {
            println!("Item: {:#?}", value);
        }
        Option::None => {
            println!("Nothing at that index");
        }
    }

    println!("");

    let item_old = catalog.get_by_index(10);
    match item_old {
        MightHaveAValue::ThereIsAValue(value) => {
            println!("Item: {:#?}", value)
        }
        MightHaveAValue::NoValueAvailable => {
            println!("No value available")
        }
    }

    println!("");

    let item = catalog.get_by_index(10);
    // In the next line, we are checking the type of 'item' and then performing an operation etc.
    if let MightHaveAValue::ThereIsAValue(value) = item {
        println!("Item in pattern match {:#?}", value);
    } else {
        println!("Got no value!")
    }

    // println!("{:#?}", item);
}
