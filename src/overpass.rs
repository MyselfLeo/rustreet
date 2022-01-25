use std::collections::HashMap;
use reqwest;


static API_URL: &str = "https://lz4.overpass-api.de/api/interpreter";
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));



/// Struct used to store overpass cache and to make requests to overpass
pub struct MapData {
    cache: HashMap<String, String>,
    client: reqwest::blocking::Client,
}

impl MapData {
    pub fn new() -> MapData {
        let client = reqwest::blocking::Client::builder().user_agent(APP_USER_AGENT).build().unwrap();
        MapData {
            cache: HashMap::new(),
            client: client,
        }
    }

    /// Takes a bounding box as parameter (min lat, min long, max lat, max long) and returns the result of the Overpass API.
    /// The result is cached for future use.
    pub fn request(&mut self, bouding_box: [String; 4]) -> String {
        let result: String;

        // Format the key used in the cache hashmap
        let key = format!("{},{},{},{}", bouding_box[0], bouding_box[1], bouding_box[2], bouding_box[3]);

        // Check if the result is already in the cache
        if self.cache.contains_key(&key) {
            result = match self.cache.get(&key.to_string()) {
                Some(value) => value.clone(),
                None => "".to_string(),
            };
        }

        // If not cached, make a request to the Overpass API and cache the result
        else {
            // Generate request data
            let data = format!("((way({})[highway];way({})[waterway];);node(w););out;", key, key);

            // Create the URL of the request
            let request_url = format!("{}?data={}", API_URL, data);

            // request and store result
            let response = self.client.get(request_url).send().unwrap();
            result = response.text().unwrap();

            // cache the result for future use
            self.cache.insert(key, result.clone());
        }

        result
    }
}