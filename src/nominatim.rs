use std::collections::HashMap;
use reqwest;
use json;



static API_URL: &str = "https://nominatim.openstreetmap.org/search";
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));



/// Struct used to store nominatim cache and to make requests to nominatim
pub struct Searcher {
    cache: HashMap<String, Vec<String>>,
    client: reqwest::blocking::Client,
}

impl Searcher {

    pub fn new() -> Searcher {
        let client = reqwest::blocking::Client::builder().user_agent(APP_USER_AGENT).build().unwrap();
        Searcher {
            cache: HashMap::new(),
            client: client,
        }
    }

    /// Takes a search string and returns the bounding box of the search result, as returned by Nominatim but with the order
    /// of the coordinates modified to fit the Overpass API format.
    pub fn research(&mut self, search: &str) -> [String; 4] {

        let result: String;

        // Check if the search string is already in the cache
        if self.cache.contains_key(&search.to_string()) {
            result = match self.cache.get(&search.to_string()) {
                Some(value) => value[0].clone(),
                None => "".to_string(),
            };
        }
        // Make a request to the Nominatim API and cache it.
        else {
            // create the URL from search string and api url
            let request_url = format!("{}?q={}&format=json&limit=1", API_URL, search.replace(" ", "+"));

            // request and store result
            let response = self.client.get(request_url).send().unwrap();
            result = response.text().unwrap();

            // cache the result for future use
            self.cache.insert(search.to_string(), vec![result.clone()]);
        }


        // Parse the result
        let first_json_value: &json::JsonValue = &json::parse(&result).unwrap()[0];
        let bounding_box = &first_json_value["boundingbox"];

        // Bounding box in the format [minlat, minlon, maxlat, maxlon] (or South, West, North, East, like Overpass API)
        [
            bounding_box[0].to_string(),
            bounding_box[2].to_string(),
            bounding_box[1].to_string(),
            bounding_box[3].to_string(),
        ]
    }
}