use crate::geo;

use std::collections::HashMap;
use json;



#[derive(Copy, Clone)]
struct Node {
    id: u64,
    lat: f64,
    lon: f64,
}



struct Way {
    id: u64,            // The id of the way, from Overpass API
    nodes: Vec<Node>,   // List of nodes of the way
}


struct Map {
    display_box: geo::BoundingBox, // Only the nodes contained in this box will be displayed
    ways: Vec<Way>, // List of ways
    lone_nodes: Vec<Node>, // List of lone nodes (not part of any way)
}

impl Map {

    /// Take the data str (as returned by OverpassData struct) and parse it
    fn from(data: String, display_box: geo::BoundingBox) -> Map {
        let mut map = Map {display_box, ways: Vec::new(), lone_nodes: Vec::new()};

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
                };
                nodes.insert(node.id, node);
            }

            // Create the Way struct and retrieve each of its nodes from the hashmap
            else if element["type"] == "way" {
                let mut way = Way {
                    id: element["id"].as_u64().unwrap(),
                    nodes: Vec::new(),
                };

                // Add this way's nodes
                for node_id in element["nodes"].members() {
                    let id_as_u64 = node_id.as_u64().unwrap();

                    if nodes.contains_key(&id_as_u64) {
                        // Remove the node from the hashmap, push it to the way's vector
                        let node = nodes.remove(&id_as_u64).unwrap();
                        way.nodes.push(node);
                    }
                }

                map.ways.push(way);
            }
        }

        // At that point, elements in the nodes hashmap are "lone nodes" (not linked to ways). Store them in map.lone_nodes
        for node_id in nodes.keys() {
            map.lone_nodes.push(nodes[&node_id]);
        }

        // Returned finished map struct
        map
    }
}