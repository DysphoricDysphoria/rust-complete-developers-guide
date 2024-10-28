#[derive(Debug)]
pub enum Media {
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
    pub fn description(&self) -> String {
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
