use core::prelude::*;

#[derive(Debug, Serialize)]
pub struct CsvRecord {
    id: String,
    osm_node: Option<u64>,
    created: u64,
    version: u64,
    title: String,
    description: String,
    lat: f64,
    lng: f64,
    street: Option<String>,
    zip: Option<String>,
    city: Option<String>,
    country: Option<String>,
    homepage: Option<String>,
    categories: Vec<String>,
    tags: Vec<String>,
    license: Option<String>,
}

impl From<Entry> for CsvRecord {
    fn from(e: Entry) -> Self {
        //TODO:
        unimplemented!();
    }
}
