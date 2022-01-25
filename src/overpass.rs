use std::collections::HashMap;
use reqwest;


static API_URL: &str = "https://lz4.overpass-api.de/api/interpreter";
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

static HIGHWAY_LEVELS: [&str; 7] = ["residential", "unclassified", "tertiary", "secondary", "primary", "trunk", "motorway"];
static WATERWAY_LEVELS: [&str; 6] = ["ditch", "drain", "stream", "river", "canal", "riverbank"];




/// Struct used to generate the text to pass to Overpass API
pub struct RequestBuilder {
    details_level: u8, // Betwenn 0 and 6. 0 is the most detailed.
    get_building: bool, // If true, the request will ONLY return buildings (the center of their geometry
    bounding_box: [String; 4],
    timeout: u32,
}

impl RequestBuilder {

    /// Create a new RequestBuilder. The bounding_box parameter is the bounding box of the research
    pub fn new(bounding_box: [String; 4]) -> RequestBuilder {
        RequestBuilder {
            details_level: 0,
            get_building: false,
            bounding_box: bounding_box,
            timeout: 30,
        }
    }

    /// Set whether to get the buildings or not.
    pub fn get_building(&mut self, value: bool) {
        self.get_building = value;
    }

    /// Set the details level of the request.
    pub fn set_details_level(&mut self, value: u8) {
        if value > 6 {
            panic!("The details level must be between 0 and 6");
        }
        self.details_level = value;
    }

    /// Build the request text to pass to Overpass API
    pub fn get_request_txt(&self, with_newline: Option<bool>) -> String {
        let mut request: String = format!("[out:json][timeout:{}];\n", self.timeout); // Base parameters for the request

        if self.get_building {
            request.push_str(format!("way[building]({}); out center;\n", self.bounding_box.join(",")).as_str());
        }

        else {
            // List every level of detail required
            request.push_str("((");

            for x in self.details_level..6 {
                request.push_str(format!("way[highway={}]({});\n", HIGHWAY_LEVELS[x as usize], self.bounding_box.join(",")).as_str());
                request.push_str(format!("way[waterway={}]({});\n", WATERWAY_LEVELS[x as usize], self.bounding_box.join(",")).as_str());
            }
            request.push_str(format!("way[highway={}]({});\n", HIGHWAY_LEVELS[6], self.bounding_box.join(",")).as_str());
            request.push_str(");\nnode(w);\n);\nout;\n");
        }

        let keep_newline: bool = match with_newline {
            Some(value) => value,
            None => false,
        };

        if keep_newline {request}
        else {request.replace('\n', "")}
    }
}










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
            let data = format!("((way({})[highway=primary];way({})[waterway=river];);node(w););out;", key, key);

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