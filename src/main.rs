mod api_wrapper;
mod geo;



fn main() {
    let mut nominatim_searcher = api_wrapper::Searcher::new();
    let base_bbox = nominatim_searcher.research("Charnoz-sur-Ain").unwrap();
    let mut map_data = api_wrapper::MapData::new();


    println!("{}", map_data.request(base_bbox));
}