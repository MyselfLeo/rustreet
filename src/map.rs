use crate::geo;
use crate::style::get_road_repr;
use crate::style::get_way_index;
use crate::ascii_map::AsciiMap;

use std::f64::consts::PI;
use std::collections::HashMap;
use std::cmp::Ordering;
use json;








#[derive(Copy, Clone)]
struct Node {
    id: u64,
    lat: f64,
    lon: f64,

    previous_lat: Option<f64>,
    previous_lon: Option<f64>,
    next_lat: Option<f64>,
    next_lon: Option<f64>,

    way_type: Option<usize>,
}


impl Node {
    /// Return the angle (in degrees) of this Node relative to the East-West direction.
    /// 
    /// The angle is determined relative to the previous node coordinates (if any) and
    /// the next node coordinates (if any).
    /// If the node is alone (no previous nor next nodes), the function will return 0.
    fn get_angle(&self) -> f64 {

        // If nor previous_lat or next_lat are specified, return 0
        if self.previous_lat.is_none() && self.next_lat.is_none() {return 0.0;}


        let d_lat: f64;
        let d_lon: f64;

        // Choose the 2 points used to compute the angle
        if self.previous_lat.is_some() && self.next_lat.is_some() {
            d_lat = self.next_lat.unwrap() - self.previous_lat.unwrap();
            d_lon = self.next_lon.unwrap() - self.previous_lon.unwrap();
        }
        else if self.previous_lat.is_some() {
            d_lat = self.lat - self.previous_lat.unwrap();
            d_lon = self.lon - self.previous_lon.unwrap();
        }
        else {
            d_lat = self.next_lat.unwrap() - self.lat;
            d_lon = self.next_lon.unwrap() - self.lon;
        }

        // Compute the angle and return it in degrees
        ((d_lat / d_lon).atan() * 180.0 / PI) + 90.0
    }


    fn get_string_rep(&self) -> String {
        if self.way_type.is_none() {String::from(" ")}
        else {get_road_repr(self.way_type.unwrap(), self.get_angle())}
    }
}





#[derive(Clone)]
struct Way {
    id: u64,                        // The id of the way, from Overpass API
    nodes: Vec<Node>,               // List of nodes of the way
    tags: HashMap<String, String>,  // Tags of this way (like "highway", "lanes", "max_speed", etc.)
}


impl Way {

    fn default_str() -> String {
        String::from(" ")
    }



    /// Return the way type of that way
    fn way_type(&self) -> &str {
        if self.tags.contains_key("highway") {self.tags["highway"].as_str();}
        else if self.tags.contains_key("waterway") {self.tags["waterway"].as_str();}

        ""
    }



    /// Return the way type index of this way
    fn way_index(&self) -> Option<usize> {
        if self.way_type() == "" {Option::None}
        else {get_way_index(self.way_type())}
    }



    /// Compare the importance value of both ways. Used to sort the way vector
    fn compare(&self, other: &Way) -> Result<Ordering, &'static str> {
        if self.way_index() == other.way_index() {Ok(Ordering::Equal)}
        else if self.way_index() < other.way_index() {Ok(Ordering::Less)}
        else if self.way_index() > other.way_index() {Ok(Ordering::Greater)}

        else {Err("Can't compare way indexes.")}
    }




    /// Append a node to this way. Modify the node to change previous_lat/lon and next_lat/lon values
    fn add_node(&mut self, mut node: Node) {
        if self.nodes.len() > 0 {
            let last_node_id = self.nodes.len() - 1;

            self.nodes[last_node_id].next_lat = Some(node.lat);
            self.nodes[last_node_id].next_lon = Some(node.lon);

            node.previous_lat = Some(self.nodes[last_node_id].lat);
            node.previous_lon = Some(self.nodes[last_node_id].lon);
        }

        self.nodes.push(node);
    }




    /// Create n new Node objects to between each Nodes of this way to add more information about the path.
    /// This will prevent the way to be displayed fragmented.
    /// NOTE: The nodes MUST BE CORRECTLY SORTED for this to work
    fn interpolate_nodes(&mut self, n: u32) {

        // Iterate over each node until the last one
        let mut i: usize = 0;

        // Prevent attempts to subtract with overflow
        if self.nodes.len() == 0 {return;}

        while i < (self.nodes.len() - 1) {

            let current_node = self.nodes[i];
            let next_node = self.nodes[i+1];

            let delta_lon = next_node.lon - current_node.lon;
            let delta_lat = next_node.lat - current_node.lat;

            let step_lon = delta_lon / (n + 1) as f64;
            let step_lat = delta_lat / (n + 1) as f64;

            // Add n nodes between current_node and next_node. Modify i accordingly
            for _ in 0..n {

                // Create a new node object
                let new_node = Node {
                    id: 0,
                    lat: self.nodes[i].lat + step_lat,
                    lon: self.nodes[i].lon + step_lon,
                    
                    previous_lat: Some(self.nodes[i].lat),
                    previous_lon: Some(self.nodes[i].lon),
                    next_lat: Option::None,
                    next_lon: Option::None,

                    way_type: self.nodes[i].way_type,
                };

                i += 1;
                self.nodes.insert(i, new_node);

                // Add to the previous node the coordinates of its next node
                self.nodes[i - 1].next_lat = Some(self.nodes[i].lat);
                self.nodes[i - 1].next_lon = Some(self.nodes[i].lon);
            }

            i += 1;
        }
    }
}





/// Structure used to generate a ascii map struct
pub struct MapGenerator {
    display_box: geo::BoundingBox,         // Only the nodes contained in this box will be displayed
    ways: Vec<Way>,                        // List of ways
    lone_nodes: Vec<Node>,                 // List of nodes (not part of any way)

    pub display_height: u16,               // height of the ASCII Map, in characters. Width = display. don't take the borders into account
}


impl MapGenerator {


    /// Take the data str (as returned by OverpassData struct) and parse it
    pub fn from(data: String, display_box: geo::BoundingBox) -> MapGenerator {
        let mut map = MapGenerator {display_box, ways: Vec::new(), lone_nodes: Vec::new(), display_height: 60};

        let json_data: json::JsonValue = json::parse(&data).unwrap();


        // List every nodes in this temporary hashmap.
        // The JSON must have every Nodes in a Way declared BEFORE that way, or nodes won't be properly linked
        let mut nodes: HashMap<u64, Node> = HashMap::new();

        for element in json_data["elements"].members() {

            // Add the node to the nodes temporary hashmap
            if element["type"] == "node" {

                let node = Node {
                    id: element["id"].as_u64().unwrap(),
                    lat: element["lat"].as_f64().unwrap(),
                    lon: element["lon"].as_f64().unwrap(),

                    previous_lat: Option::None,
                    previous_lon: Option::None,
                    next_lat: Option::None,
                    next_lon: Option::None,

                    way_type: Option::None, // Will be changed in
                };
                
                nodes.insert(node.id, node);
            }

            // Create the Way struct and retrieve each of its nodes from the hashmap
            else if element["type"] == "way" {

                // List the tags to be added to the way struct
                let mut tags: HashMap<String, String> = HashMap::new();
                for entry in element["tags"].entries() {
                    tags.insert(entry.0.to_string(), entry.1.to_string());
                }

                let mut way = Way {
                    id: element["id"].as_u64().unwrap(),
                    nodes: Vec::new(),
                    tags: tags.clone(),
                };

                // Add this way's nodes
                for node_id in element["nodes"].members() {
                    let id_as_u64 = node_id.as_u64().unwrap();

                    if nodes.contains_key(&id_as_u64) {

                        // Remove the node from the hashmap
                        let mut node = nodes.remove(&id_as_u64).unwrap();

                        // Add the way's type to the node
                        node.way_type = way.way_index();

                        // Push the node to the vector of nodes from the way
                        way.add_node(node);
                    }
                }

                // Interpolate the nodes of that way
                way.interpolate_nodes(map.display_height as u32);

                map.ways.push(way);
            }
        }

        // At that point, elements in the nodes hashmap are "lone nodes" (not linked to ways). Store them in map.lone_nodes
        for node_id in nodes.keys() {
            map.lone_nodes.push(nodes[&node_id]);
        }

        //map.ways.sort_by(|a, b| a.compare(b).unwrap());

        // Debug
        println!("[INFO] Number of ways: {}", map.ways.len());

        // Returned finished map struct
        map
    }




    /// Set the size of the displayed ascii map (in characters)
    pub fn set_size(&mut self, size: u16) {
        self.display_height = size;
    }



    pub fn generate_ascii_map(&self) -> AsciiMap {

        // Initialise map
        let mut data: Vec<Vec<String>> = Vec::new();
        for x in 0..self.display_height {
            data.push(Vec::new());

            for _ in 0..self.display_height {
                data[x as usize].push(Way::default_str());
            }
        }


        // For each node of each way, we get its coordinate in the asciimap and put the character representing it
        for way in &self.ways {

            for node in &way.nodes {

                // Get the relative coordinates of the node compared to the display box
                let rel_lat = node.lat - self.display_box.coo[0]; // lat - min_lat
                let rel_lon = node.lon - self.display_box.coo[1]; // lon - min_lon
                
                // Skip this node if it's not contained in the display box
                if rel_lat < 0.0 || rel_lon < 0.0 {continue;}
                if rel_lat > self.display_box.dim_deg[0] || rel_lon > self.display_box.dim_deg[1] {continue;}

                // Get the character coordinates
                let char_x = rel_lat / self.display_box.dim_deg[0] * self.display_height as f64;
                let char_x = char_x.floor() as usize;

                let char_y = rel_lon / self.display_box.dim_deg[1] * self.display_height as f64;
                let char_y = char_y.floor() as usize;

                // If the point is out of bounds, skip without adding it to the data
                if char_x >= self.display_height as usize || char_y >= self.display_height as usize {continue;}

                // Add the way character to the ascii map
                data[char_x][char_y] = String::from(node.get_string_rep());
            }

        }


        // Return the AsciiMap
        let mut ascii_map = AsciiMap::from(data);
        ascii_map.double();
        ascii_map
    }
}