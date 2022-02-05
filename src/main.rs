mod api_wrapper;
mod geo;
mod map;
mod ascii_map;
mod style;

use clap::Parser;




// Args parsing using clap
#[derive(Parser)]
#[clap(author = "myselfleo", version = "0.1.0", about = "Display maps in your terminal !")]
struct Args {
    /// If specified, will start Rustreet in interactive mode.
    #[clap(short, long)]
    interactive: bool,

    /// A string representing a place. Example: Paris, France
    search: String,

    /// The level of zoom, should be greater than 0. Default is 1.0.
    #[clap(short, long, default_value_t = 1.0)]
    zoom: f64,

    /// Specifies the size of the outputted image. Default is 60
    #[clap(short, long, default_value_t = 60)]
    size: u16,

    /// Specifies the level of details, between 0 and 6. 0 is the most detailled and 6 the less.
    /// The default value depends on the size of the displayed area.
    #[clap(short, long)]
    details_lvl: Option<u16>,
}






fn main() {
    // Create clap App
    let mut app = clap::App::new("Rustreet");

    // Get arguments from command line
    let args = Args::parse();

    // Throw clap error if arguments are not valid
    if args.zoom <= 0.0 {
        let err = app.error(clap::ErrorKind::InvalidValue, "The zoom value must be greater than 0.");
        err.exit();
    }


    let mut nominatim_searcher = api_wrapper::Searcher::new();

    println!("[INFO] Requesting Nominatim data for {}", args.search);
    let mut base_bbox = nominatim_searcher.research(&args.search).unwrap();
    println!("[INFO] Data received");

    base_bbox.zoom(args.zoom);
    let mut overpass_data = api_wrapper::OverpassData::new();

    println!("[INFO] Requesting data from Overpass API");
    let data = overpass_data.request(base_bbox);
    println!("[INFO] Data received");

    println!("[INFO] Generating map");
    let mut map = map::MapGenerator::from(data, base_bbox);

    map.set_size(args.size);

    println!("[INFO] Map generated");

    let ascii_map = map.generate_ascii_map();

    let with_dec = ascii_map.with_decoration();
    with_dec.print();
}