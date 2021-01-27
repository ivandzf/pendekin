mod plugins;

pub fn new_store_factory(store_type: String) -> plugins::StoreFactory {
    let mut store = plugins::Store::Unknown;
    if store_type == "map" {
        let static_data = plugins::map::StaticMapData::new(store);
        store = plugins::Store::StaticMap(static_data);
    } else if store_type == "redis" {
        store = plugins::Store::Redis;
    }

    plugins::StoreFactory { store }
}
