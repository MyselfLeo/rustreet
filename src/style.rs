const PRIMAY_HIGHWAY: [&str; 4] = ["\x1b[33m═\x1b[0m", "\x1b[33m/\x1b[0m", "\x1b[33m║\x1b[0m", "\x1b[33m\\\x1b[0m"];
const SECONDARY_HIGHWAY: [&str; 4] = ["═", "/", "║", "\\"];
const TERTIARY_HIGHWAY: [&str; 4] = ["-", "/", "|", "\\"];
const SMALL_HIGHWAY: [&str; 4] = ["\x1b[32m-\x1b[0m", "\x1b[32m/\x1b[0m", "\x1b[32m|\x1b[0m", "\x1b[32m\\\x1b[0m"];




/// Return true if n is between m1 and m2 (included)
fn is_between(n: f64, m1: f64, m2: f64) -> bool {n >= m1 && n <= m2}



/// Return the string (a character and ansi escape codes, if any) representing a node
/// of the road_type (ex: motorway, secondary, path...) with the given angle
fn get_road_repr(road_type: &str, angle: f64) {
    
    // Get the orientation of the character.
    // Example with a simple line: 0= -, 1= /, 2= |, 3= \
    let orientation: u8 = 0;
    if is_between(angle, 67.5, 112.5) || is_between(angle, 247.5, 292.5) {let orientation = 0;}
    else if is_between(angle, 112.5, 157.5) || is_between(angle, 292.5, 337.5) {let orientation = 1;}
    else if is_between(angle, 337.5, 360.0) || is_between(angle, 0.0, 22.5) {let orientation = 2;}
    else if is_between(angle, 22.5, 67.5) || is_between(angle, 202.5, 247.5) {let orientation = 3;}

    
    let value: u8 = match road_type {
        "motorway" => 10,
        "trunk" => 10,
        "primary" => 11,
        "secondary" => 12,
        "tertiary" => 13,
        "unclassified" => 13,
        "residential" => 14,
        "motorway_link" => 10,
        "trunk_link" => 10,
        "primary_link" => 11,
        "secondary_link" => 12,
        "tertiary_link" => 13,
        "living_street" => 14,
        "service" => 13,
        "pedestrian" => 15,
        "track" => 15,
        "bus_guideway" => 12,
        "escape" => 15,
        "raceway" => 15,
        "road" => 13,
        "busway" => 12,
        "footway" => 15,
        "bridleway" => 15,
        "steps" => 15,
        "corridor" => 16,
        "path" => 15,

        "river" => 20,
        "riverbank" => 20,
        "stream" => 21,
        "tidal_channel" => 21,
        "canal" => 21,
        "pressurised" => 22,
        "drain" => 23,
        "ditch" => 23,
        "fairway" => 23,
        "fish_pass" => 24,
    };


}