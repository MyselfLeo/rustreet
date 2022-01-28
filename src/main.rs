mod api_wrapper;
mod geo;
mod map;



fn main() {
    let research = "Charnoz-sur-Ain, France";

    let mut nominatim_searcher = api_wrapper::Searcher::new();

    println!("[INFO] Requesting Nominatim data for {}", research);
    let mut base_bbox = nominatim_searcher.research(research).unwrap();
    println!("[INFO] Data received");

    // base_bbox.zoom(2.0);
    let mut overpass_data = api_wrapper::OverpassData::new();

    println!("[INFO] Requesting data from Overpass API");
    let data = overpass_data.request(base_bbox);
    println!("[INFO] Data received");

    println!("[INFO] Generating map");
    let mut map = map::Map::from(data, base_bbox);
    map.set_size(50);
    println!("[INFO] Map generated");

    let ascii_map = map.generate_ascii_map();

    print!("╔");
    for _ in 0..map.display_size * 2 {print!("═")}
    print!("╗\n");

    for x in 0..map.display_size {
        print!("║");
        for y in 0..map.display_size {
            print!("{}{}", ascii_map[(map.display_size - 1 - x) as usize][y as usize], ascii_map[(map.display_size - 1 - x) as usize][y as usize]);
        }
        print!("║\n");
    }

    print!("╚");
    for _ in 0..map.display_size * 2 {print!("═")}
    print!("╝\n");
}