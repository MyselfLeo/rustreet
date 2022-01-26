use crate::geo;

use std::collections::HashMap;




struct Node {
    id: u64,
    lat: f64,
    lon: f64,
}



struct Way {
    id: u64,            // The id of the way, from Overpass API
    nodes: Vec<Node>,   // List of nodes of the way
    character: char,    // The character representing the way
}



struct Map {
    bounding_box: geo::BoundingBox,
    ways: Vec<Way>, // List of ways
}