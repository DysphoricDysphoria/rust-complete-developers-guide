use super::container::Container;

#[derive(Debug)]
pub struct Basket<T> {
    item: Option<T>,
}

impl<T> Basket<T> {
    pub fn new(item: T) -> Self {
        Basket { item: Some(item) }
    }
}

impl<T> Container<T> for Basket<T> {
    // Methods here will be public by default
    fn get(&mut self) -> Option<T> {
        self.item.take()
    }

    fn put(&mut self, item: T) {
        self.item = Some(item);
    }

    fn is_empty(&self) -> bool {
        self.item.is_none()
    }
}
