pub mod map;
pub mod redis;

pub enum Store {
    Unknown,
    Redis,
    StaticMap(map::StaticMapData),
}

pub struct StoreFactory {
    store: Store,
}

#[derive(Clone)]
pub struct Shortener {
    hash: &'static str,
    url: String,
    lifetime: u64,
}

pub trait Service {
    fn new(store: Store) -> Self;
    fn insert(&mut self, shortener: Shortener);
    fn get(&self, hash: &'static str) -> String;
}

impl Service for StoreFactory {
    fn new(store: Store) -> Self {
        let mut new_store = Store::Unknown;
        match store {
            Store::Unknown => new_store = Store::Unknown,
            Store::Redis => new_store = Store::Redis,
            Store::StaticMap(_state) => {
                let static_data = map::StaticMapData::new(new_store);
                new_store = Store::StaticMap(static_data);
            }
        }

        StoreFactory { store: new_store }
    }

    fn insert(&mut self, shortener: Shortener) {}

    fn get(&self, hash: &'static str) -> String {
        "".to_string()
    }
}
