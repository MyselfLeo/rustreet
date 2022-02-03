const PRIMAY_HIGHWAY: [&str; 4] = ["\x1b[33m═\x1b[0m", "\x1b[33m/\x1b[0m", "\x1b[33m║\x1b[0m", "\x1b[33m\\\x1b[0m"];
const SECONDARY_HIGHWAY: [&str; 4] = ["═", "/", "║", "\\"];
const TERTIARY_HIGHWAY: [&str; 4] = ["\x1b[33m-\x1b[0m", "\x1b[33m/\x1b[0m", "\x1b[33m|\x1b[0m", "\x1b[33m\\\x1b[0m"];
const QUATERNARY_HIGHWAY: [&str; 4] = ["-", "/", "|", "\\"];
const SMALL_HIGHWAY: [&str; 4] = ["\x1b[32m-\x1b[0m", "\x1b[32m/\x1b[0m", "\x1b[32m|\x1b[0m", "\x1b[32m\\\x1b[0m"];
const VERY_SMALL_HIGHWAY: [&str; 4] = ["⋯", "⋰", "⋮", "⋱"];




/// Return true if n is between m1 and m2 (included)
fn is_between(n: f64, m1: f64, m2: f64) -> bool {n >= m1 && n <= m2}



/// Return the string (a character and ansi escape codes, if any) representing a node
/// of the road_type (ex: motorway, secondary, path...) with the given angle
fn get_road_repr(road_type: &str, angle: f64) {
    
    // Get the orientation of the character.
    // Example with a simple line: 0= -, 1= /, 2= |, 3= \
    let orientation: usize = 0;
    if is_between(angle, 67.5, 112.5) || is_between(angle, 247.5, 292.5) {let orientation = 0;}
    else if is_between(angle, 112.5, 157.5) || is_between(angle, 292.5, 337.5) {let orientation = 1;}
    else if is_between(angle, 337.5, 360.0) || is_between(angle, 0.0, 22.5) {let orientation = 2;}
    else if is_between(angle, 22.5, 67.5) || is_between(angle, 202.5, 247.5) {let orientation = 3;}

    
    match road_type {
        "motorway" => String::from(PRIMAY_HIGHWAY[orientation]),
        "trunk" => String::from(PRIMAY_HIGHWAY[orientation]),
        "primary" => String::from(SECONDARY_HIGHWAY[orientation]),
        "secondary" => String::from(TERTIARY_HIGHWAY[orientation]),
        "tertiary" => String::from(QUATERNARY_HIGHWAY[orientation]),
        "unclassified" => String::from(QUATERNARY_HIGHWAY[orientation]),
        "residential" => String::from(SMALL_HIGHWAY[orientation]),
        "motorway_link" => String::from(PRIMAY_HIGHWAY[orientation]),
        "trunk_link" => String::from(PRIMAY_HIGHWAY[orientation]),
        "primary_link" => String::from(SECONDARY_HIGHWAY[orientation]),
        "secondary_link" => String::from(TERTIARY_HIGHWAY[orientation]),
        "tertiary_link" => String::from(QUATERNARY_HIGHWAY[orientation]),
        "living_street" => String::from(SMALL_HIGHWAY[orientation]),
        "service" => String::from(QUATERNARY_HIGHWAY[orientation]),
        "pedestrian" => String::from(VERY_SMALL_HIGHWAY[orientation]),
        "track" => String::from(VERY_SMALL_HIGHWAY[orientation]),
        "bus_guideway" => String::from(TERTIARY_HIGHWAY[orientation]),
        "escape" => String::from(VERY_SMALL_HIGHWAY[orientation]),
        "raceway" => String::from(VERY_SMALL_HIGHWAY[orientation]),
        "road" => String::from(QUATERNARY_HIGHWAY[orientation]),
        "busway" => String::from(TERTIARY_HIGHWAY[orientation]),
        "footway" => String::from(VERY_SMALL_HIGHWAY[orientation]),
        "bridleway" => String::from(VERY_SMALL_HIGHWAY[orientation]),
        "steps" => String::from(VERY_SMALL_HIGHWAY[orientation]),
        "corridor" => String::from(VERY_SMALL_HIGHWAY[orientation]),
        "path" => String::from(VERY_SMALL_HIGHWAY[orientation]),

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