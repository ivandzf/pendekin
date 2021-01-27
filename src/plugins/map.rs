use crate::plugins;
use std::collections::HashMap;

pub struct StaticMapData {
    data: HashMap<&'static str, plugins::Shortener>,
}

impl plugins::Service for StaticMapData {
    fn new(_store: plugins::Store) -> Self {
        StaticMapData {
            data: HashMap::new(),
        }
    }

    fn insert(&mut self, shortener: plugins::Shortener) {
        self.data.insert(shortener.hash, shortener);
    }

    fn get(&self, hash: &'static str) -> String {
        self.data.get(&hash).unwrap().url.to_string()
    }
}
