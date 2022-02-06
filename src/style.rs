const PRIMAY_HIGHWAY: [&str; 4] = ["\x1b[93m═\x1b[0m", "\x1b[93m⇗\x1b[0m", "\x1b[93m║\x1b[0m", "\x1b[93m⇖\x1b[0m"];
const SECONDARY_HIGHWAY: [&str; 4] = ["═", "⇗", "║", "⇖"];
const TERTIARY_HIGHWAY: [&str; 4] = ["\x1b[33m-\x1b[0m", "\x1b[33m/\x1b[0m", "\x1b[33m|\x1b[0m", "\x1b[33m\\\x1b[0m"];
const QUATERNARY_HIGHWAY: [&str; 4] = ["-", "/", "|", "\\"];
const SMALL_HIGHWAY: [&str; 4] = ["\x1b[90m-\x1b[0m", "\x1b[90m/\x1b[0m", "\x1b[90m|\x1b[0m", "\x1b[90m\\\x1b[0m"];
const VERY_SMALL_HIGHWAY: [&str; 4] = ["\x1b[32m⋯\x1b[0m", "\x1b[32m⋰\x1b[0m", "\x1b[32m⋮\x1b[0m", "\x1b[32m⋱\x1b[0m"];

const RIVER: [&str; 4] = ["\x1b[34m═\x1b[0m", "\x1b[34m⇗\x1b[0m", "\x1b[34m║\x1b[0m", "\x1b[34m⇖\x1b[0m"];
const STREAM: [&str; 4] = ["\x1b[34m-\x1b[0m", "\x1b[34m/\x1b[0m", "\x1b[34m|\x1b[0m", "\x1b[34m\\\x1b[0m"];



const WAY_TYPES: [&str; 36] = [
    "motorway",
    "trunk",
    "primary",
    "secondary",
    "tertiary",
    "unclassified",
    "residential",
    "motorway_link",
    "trunk_link",
    "primary_link",
    "secondary_link",
    "tertiary_link",
    "living_street",
    "service",
    "pedestrian",
    "track",
    "bus_guideway",
    "escape",
    "raceway",
    "road",
    "busway",
    "footway",
    "bridleway",
    "steps",
    "corridor",
    "path",
    "river",
    "riverbank",
    "stream",
    "tidal_channel",
    "canal",
    "pressurised",
    "drain",
    "ditch",
    "fairway",
    "fish_pass",
];



/// Return a Vector of way types (motorway, canal, primary, etc.)
/// corresponding to the given detail level
pub fn get_way_types(detail_lvl: u8) -> Vec<&'static str> {

    // Base list of way types.
    let mut res: Vec<&str> = Vec::from(["motorway", "motorway_link", "trunk", "trunk_link", "river"]);

    if detail_lvl > 0 {
        res.append(&mut Vec::from(["primary", "primary_link", "riverbank", "canal"]));
    }
    if detail_lvl > 1 {
        res.append(&mut Vec::from(["secondary", "secondary_link", "stream"]));
    }
    if detail_lvl > 2 {
        res.append(&mut Vec::from(["tertiary", "tertiary_link"]));
    }
    if detail_lvl > 3 {
        res.append(&mut Vec::from(["unclassified", "bus_guideway", "busway"]))
    }
    if detail_lvl > 4 {
        res.append(&mut Vec::from(["residential", "living_street", "service", "road", "pressurised", "drain", "ditch", "fairway"]));
    }
    if detail_lvl > 5 {
        res.append(&mut Vec::from(["pedestrian", "track", "escape", "raceway", "footway", "steps", "corridor", "path", "fish_pass"]));
    }

    res
}





/// Return the index of the given way_type, or none if does not exists
pub fn get_way_index(value: &str) -> Option<usize> {
    for i in 0..36 {
        if WAY_TYPES[i] == value {return Some(i);}
    }
    Option::None
}



/// Return true if n is between m1 and m2 (included)
fn is_between(n: f64, m1: f64, m2: f64) -> bool {n >= m1 && n <= m2}



/// Return the string (a character and ansi escape codes, if any) representing a node
/// of the road_type (ex: motorway, secondary, path...) with the given angle
pub fn get_road_repr(way_type_index: usize, angle: f64) -> String {
    
    // Get the orientation of the character.
    // Example with a simple line: 0= -, 1= /, 2= |, 3= \
    let mut orientation: usize = 0;
    if is_between(angle, 67.5, 112.5) || is_between(angle, 247.5, 292.5) {orientation = 0;}
    else if is_between(angle, 112.5, 157.5) || is_between(angle, 292.5, 337.5) {orientation = 1;}
    else if is_between(angle, 337.5, 360.0) || is_between(angle, 0.0, 22.5) {orientation = 2;}
    else if is_between(angle, 22.5, 67.5) || is_between(angle, 202.5, 247.5) {orientation = 3;}
    
    match way_type_index {
        0 => String::from(PRIMAY_HIGHWAY[orientation]),
        1 => String::from(PRIMAY_HIGHWAY[orientation]),
        2 => String::from(SECONDARY_HIGHWAY[orientation]),
        3 => String::from(TERTIARY_HIGHWAY[orientation]),
        4 => String::from(QUATERNARY_HIGHWAY[orientation]),
        5 => String::from(QUATERNARY_HIGHWAY[orientation]),
        6 => String::from(SMALL_HIGHWAY[orientation]),
        7 => String::from(PRIMAY_HIGHWAY[orientation]),
        8 => String::from(PRIMAY_HIGHWAY[orientation]),
        9 => String::from(SECONDARY_HIGHWAY[orientation]),
        10 => String::from(TERTIARY_HIGHWAY[orientation]),
        11 => String::from(QUATERNARY_HIGHWAY[orientation]),
        12 => String::from(SMALL_HIGHWAY[orientation]),
        13 => String::from(QUATERNARY_HIGHWAY[orientation]),
        14 => String::from(VERY_SMALL_HIGHWAY[orientation]),
        15 => String::from(VERY_SMALL_HIGHWAY[orientation]),
        16 => String::from(TERTIARY_HIGHWAY[orientation]),
        17 => String::from(VERY_SMALL_HIGHWAY[orientation]),
        18 => String::from(VERY_SMALL_HIGHWAY[orientation]),
        19 => String::from(QUATERNARY_HIGHWAY[orientation]),
        20 => String::from(TERTIARY_HIGHWAY[orientation]),
        21 => String::from(VERY_SMALL_HIGHWAY[orientation]),
        22 => String::from(VERY_SMALL_HIGHWAY[orientation]),
        23 => String::from(VERY_SMALL_HIGHWAY[orientation]),
        24 => String::from(VERY_SMALL_HIGHWAY[orientation]),
        25 => String::from(VERY_SMALL_HIGHWAY[orientation]),

        26 => String::from(RIVER[orientation]),
        27 => String::from(RIVER[orientation]),
        28 => String::from(STREAM[orientation]),
        29 => String::from(STREAM[orientation]),
        30 => String::from(STREAM[orientation]),
        31 => String::from(STREAM[orientation]),
        32 => String::from(STREAM[orientation]),
        33 => String::from(STREAM[orientation]),
        34 => String::from(STREAM[orientation]),
        35 => String::from(STREAM[orientation]),

        _ => String::from(" "),
    }
}