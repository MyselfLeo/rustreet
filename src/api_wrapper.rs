use crate::geo;

use std::collections::HashMap;
use std::io;
use reqwest;
use json;

static NOMINATIM_API_URL: &str = "https://nominatim.openstreetmap.org/search";
static OVERPASS_API_URL: &str = "https://lz4.overpass-api.de/api/interpreter";
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));


static HIGHWAY_LEVELS: [&str; 7] = ["residential", "unclassified", "tertiary", "secondary", "primary", "trunk", "motorway"];
static WATERWAY_LEVELS: [&str; 6] = ["ditch", "drain", "stream", "river", "canal", "riverbank"];













/// Struct used to generate the text to pass to Overpass API
pub struct RequestBuilder {
    get_building: bool, // If true, the request will ONLY return buildings (the center of their geometry
    timeout: u32,
    bounding_box: geo::BoundingBox,
}

impl RequestBuilder {

    /// Take the scale of the map (i.e the width of the displayed map, in km) and return the corresponding level of details
    pub fn get_lvl_details(scale: f64) -> u8 {
        // TODO: add proper algorithm
        return 0;
    }



    /// Create a new RequestBuilder. The bounding_box parameter is the bounding box of the research
    pub fn new(bounding_box: geo::BoundingBox) -> RequestBuilder {
        RequestBuilder {
            get_building: false,
            bounding_box: bounding_box,
            timeout: 30,
        }
    }



    /// Set whether to get the buildings or not.
    pub fn get_building(&mut self, value: bool) {
        self.get_building = value;
    }



    /// Build the request text to pass to Overpass API
    pub fn get_request_txt(&self, with_newline: Option<bool>) -> String {

        // Base parameters for the request
        let mut request: String = format!("[out:json][timeout:{}];\n", self.timeout);

        // Create the bounding box string
        let bbox_str = format!("{},{},{},{}", self.bounding_box.coo[0], self.bounding_box.coo[1], self.bounding_box.coo[2], self.bounding_box.coo[3]);


        if self.get_building {
            request.push_str(format!("way[building]({});+out+center;\n", bbox_str).as_str());
        }

        else {
            // List every level of detail required
            request.push_str("(\n(\n");

            for x in RequestBuilder::get_lvl_details(self.bounding_box.dim_km[0])..6 {
                request.push_str(format!("way[highway={}]({});\n", HIGHWAY_LEVELS[x as usize], bbox_str).as_str());
                request.push_str(format!("way[waterway={}]({});\n", WATERWAY_LEVELS[x as usize], bbox_str).as_str());
            }
            request.push_str(format!("way[highway={}]({});\n", HIGHWAY_LEVELS[6], bbox_str).as_str());

            // request.push_str(format!(");\nnode(w)({});\n);\nout;\n", bbox_str).as_str());
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














/// Struct used to store nominatim cache and to make requests to nominatim
pub struct Searcher {
    cache: HashMap<String, String>,
    client: reqwest::blocking::Client,
}

impl Searcher {

    /// Return a new Searcher
    pub fn new() -> Searcher {
        let client = reqwest::blocking::Client::builder().user_agent(APP_USER_AGENT).build().unwrap();
        Searcher {
            cache: HashMap::new(),
            client: client,
        }
    }

    /// Takes a search string and returns the OSM id of the first returned area.
    pub fn research(&mut self, search: &str) -> Result<geo::BoundingBox, io::Error> {

        let result: String;

        // Check if the search string is already in the cache
        if self.cache.contains_key(&search.to_string()) {
            result = match self.cache.get(&search.to_string()) {
                Some(value) => value.clone(),
                None => "".to_string(),
            };
        }
        // Make a request to the Nominatim API and cache it.
        else {
            // create the URL from search string and api url
            let request_url = format!("{}?q={}&format=json&limit=1", NOMINATIM_API_URL, search.replace(" ", "+"));

            // request and store result
            let response = self.client.get(request_url).send().unwrap();
            result = response.text().unwrap();

            // cache the result for future use
            self.cache.insert(search.to_string(), result.clone());
        }

        // Parse the result
        let first_json_value: &json::JsonValue = &json::parse(&result).unwrap()[0];
        
        // Get bounding box returned by Nominatim
        let bbox_of_jsonvalue = &first_json_value["boundingbox"];

        // Transform the returned string values as f64
        let mut bbox_of_f64: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        for i in 0..4 {
            bbox_of_f64[i] = match bbox_of_jsonvalue[i].as_str().unwrap().parse::<f64>() {
                Ok(value) => value,
                _ => {
                    // Raise error if the value is not a valid f64
                    println!("[ERROR]: The bounding box returned by Nominatim is not valid.");
                    return Err(io::Error::new(io::ErrorKind::Other, "[ERROR]: The bounding box returned by Nominatim is not valid."));
                },
            }
        }

        // Build the BoundingBox and return it
        // (we're using the minlat minlon maxlat maxlon order, instead of the maxlat maxlon minlat minlon order given by Nominatim)
        Ok(geo::BoundingBox::new(bbox_of_f64[0], bbox_of_f64[2], bbox_of_f64[1], bbox_of_f64[3]))
    }
}








/// Struct used to store overpass cache and to make requests to overpass
pub struct OverpassData {
    cache: HashMap<String, String>,
    client: reqwest::blocking::Client,
}

impl OverpassData {

    /// Return a newly created OverpassData struct
    pub fn new() -> OverpassData {
        let client = reqwest::blocking::Client::builder().user_agent(APP_USER_AGENT).build().unwrap();
        OverpassData {
            cache: HashMap::new(),
            client: client,
        }
    }




    /// Takes a bounding box as parameter (min lat, min long, max lat, max long) and returns the result of the Overpass API.
    /// The result is cached for future use.
    pub fn request(&mut self, bounding_box: geo::BoundingBox) -> String {
        let result: String;

        // Format the key used in the cache hashmap
        let key = format!("{},{},{},{}", bounding_box.coo[0], bounding_box.coo[1], bounding_box.coo[2], bounding_box.coo[3]);

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
            let request_builder = RequestBuilder::new(bounding_box);
            let request_data = request_builder.get_request_txt(None);

            // Create the URL of the request
            let request_url = format!("{}?data={}", OVERPASS_API_URL, request_data);

            // request and store result
            let response = self.client.get(request_url).send().unwrap();
            result = response.text().unwrap();

            // cache the result for future use
            self.cache.insert(key, result.clone());
        }

        result
    }
}