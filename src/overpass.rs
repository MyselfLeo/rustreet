use std::collections::HashMap;
use reqwest;
use json;


static API_URL: &str = "https://lz4.overpass-api.de/api/interpreter";
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));



/// Struct used to store overpass cache and to make requests to overpass
pub struct MapData {
    cache: HashMap<String, Vec<String>>,
    client: reqwest::blocking::Client,
}

// TODO: add a function to request data from a given bounding box