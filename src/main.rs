mod api_wrapper;
mod geo;
mod map;



fn main() {
    let mut nominatim_searcher = api_wrapper::Searcher::new();
    let base_bbox = nominatim_searcher.research("Charnoz-sur-Ain").unwrap();
    let mut overpass_data = api_wrapper::OverpassData::new();


    println!("{}", overpass_data.request(base_bbox));
}