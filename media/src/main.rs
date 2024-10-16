#[derive(Debug)]
enum Media {
    // Book, Movie etc. are variants of Media
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
    // Podcast { episode_number: u32 },
    Podcast(u32), // Unnamed field - This will probably confuse people
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
            Media::Podcast(episode_number) => {
                format!("Podcast: {}", episode_number)
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
        self.items.push(media);
    }
}

fn print_media(media: &Media) {
    println!("{:#?}", media);
}

fn main() {
    let audio_book = Media::AudioBook {
        title: String::from("Who will cry when you will die?"),
    };
    let a_movie = Media::Movie {
        title: String::from("Interstellar"),
        director: String::from("A director"),
    };
    let a_book = Media::Book {
        title: String::from("A book"),
        author: String::from("An author"),
    };
    let podcast = Media::Podcast(1);
    let placeholder = Media::Placeholder;

    println!("{}", a_movie.description());
    println!("{}", a_book.description());
    println!("{}", audio_book.description());

    print_media(&a_movie);
    print_media(&a_book);
    print_media(&audio_book);

    let mut catalog = Catalog::new();

    catalog.add(audio_book);
    catalog.add(a_movie);
    catalog.add(a_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    println!("{:#?}", catalog);
}
