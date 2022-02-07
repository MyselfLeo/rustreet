mod api_wrapper;
mod geo;
mod map;
mod ascii_map;
mod style;

use clap::Parser;






/// Throw clap error if arguments are not valid
fn check_arguments(args: &Args) {

    // Create clap App
    let mut app = clap::App::new("Rustreet");

    if args.zoom <= 0.0 {
        let err = app.error(clap::ErrorKind::InvalidValue, "The zoom value must be greater than 0.");
        err.exit();
    }


    // TODO: REMOVE WHEN INTERACTIVE MODE IS IMPLEMENTED
    if args.interactive {
        let err = app.error(clap::ErrorKind::ArgumentConflict, "Interactive mode not implemented yet.");
        err.exit();
    }
}




/// Process the request of the user one time, print the generated map and exit the program
fn one_shot(args: &Args) {

    // Request bounding box from Nominatim
    if args.info {println!("[INFO] Requesting data from Nominatim API")}
    let mut searcher = api_wrapper::Searcher::new();
    let mut bbox = searcher.research(&args.search).unwrap();
    if args.info {println!("[INFO] Nominatim data received. Bounding box: {}, {}, {}, {} (S/W/N/E)", bbox.coo[0], bbox.coo[1], bbox.coo[2], bbox.coo[3])}

    // Apply zoom
    bbox.zoom(args.zoom);
    if args.info && args.zoom != 0.0 {println!("[INFO] Applied a x{} zoom. New bounding box: {}, {}, {}, {} (S/W/N/E)", args.zoom, bbox.coo[0], bbox.coo[1], bbox.coo[2], bbox.coo[3])}

    // Request map data from the Overpass API
    if args.info {println!("[INFO] Requesting map data from Overpass API")}
    let mut overpass_data = api_wrapper::OverpassData::new();
    let data = overpass_data.request(bbox, args.timeout);
    if args.info {println!("[INFO] Data received")}

    // Generate the map
    if args.info {println!("[INFO] Generating map of size {}", args.size)}
    let map = map::MetaMap::from(data, bbox, args.size);
    if args.info {println!("[INFO] Map generated")}

    // Display map and exit
    let ascii_map = map.generate_ascii_map().with_decoration();
    ascii_map.print();
}










// Args parsing using clap
#[derive(Parser)]
#[clap(author = "myselfleo", version = "0.1.0", about = "Display maps in your terminal !")]
struct Args {
    /// Specifies if Rustreet must start in one-shot or interactive mode.
    #[clap(short, long)]
    interactive: bool,

    /// A string representing a place. Example: Paris, France
    search: String,

    /// The level of zoom, should be greater than 0.
    #[clap(short, long, default_value_t = 1.0)]
    zoom: f64,

    /// Specifies the size of the outputted image.
    #[clap(short, long, default_value_t = 60)]
    size: u16,

    /// Specifies the level of details, between 0 and 6. 0 is the most detailled and 6 the less.
    /// The default value depends on the size of the displayed area.
    #[clap(short, long)]
    details_lvl: Option<u16>,

    /// Set the timeout for requests to Overpass API.
    #[clap(short, long, default_value_t = 30)]
    timeout: u32,

    /// If specified, will display information messages. Don't work in interactive mode.
    #[clap(long)]
    info: bool,
}





fn main() {
    // Get arguments from command line
    let args = Args::parse();
    check_arguments(&args);

    
    // Run Rustreet in interactive or one-shot mode
    if args.interactive {

    }

    else {
        one_shot(&args);
    }
}