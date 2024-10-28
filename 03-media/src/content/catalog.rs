use super::media::Media; // 'super' is the reference to the parent module

// TODO: `'a` is a lifetime parameter. What is that?
pub enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable,
}

#[derive(Debug)]
pub struct Catalog {
    pub items: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { items: Vec::new() }
    }

    pub fn add(&mut self, media: Media) {
        self.items.push(media); // We are taking ownership of 'media' here
    }

    pub fn get_by_index(&self, index: usize) -> MightHaveAValue {
        if self.items.len() > index {
            // Something to return
            MightHaveAValue::ThereIsAValue(&self.items[index]) // We don't want to transfer ownership, hence the use of '&'
        } else {
            // Nothing to return
            MightHaveAValue::NoValueAvailable
        }
    }

    pub fn get_by_index_new(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        }
    }
}
