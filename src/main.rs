mod api_wrapper;
mod geo;
mod map;

use clap::Parser;





// Args parsing using clap
#[derive(Parser)]
#[clap(author = "myselfleo", version = "0.1.0", about = "Display maps in your terminal !")]
struct Args {
    /// A string representing a place. Example: Paris, France
    search: String,

    /// Specifies the size of the outputted image. Default is 100
    #[clap(short, long)]
    size: Option<u16>,
}






fn main() {
    // Get arguments from command line
    let args = Args::parse();


    let mut nominatim_searcher = api_wrapper::Searcher::new();

    println!("[INFO] Requesting Nominatim data for {}", args.search);
    let mut base_bbox = nominatim_searcher.research(&args.search).unwrap();
    println!("[INFO] Data received");

    // base_bbox.zoom(2.0);
    let mut overpass_data = api_wrapper::OverpassData::new();

    println!("[INFO] Requesting data from Overpass API");
    let data = overpass_data.request(base_bbox);
    println!("[INFO] Data received");

    println!("[INFO] Generating map");
    let mut map = map::Map::from(data, base_bbox);

    if args.size.is_some() {
        map.set_size(args.size.unwrap());
    }

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