mod nominatim;
mod overpass;



fn main() {
    /*
    let mut searcher = nominatim::Searcher::new();
    let mut map_data = overpass::MapData::new();

    let bounding_box = searcher.research("Villieu, France");

    let result = map_data.request(bounding_box);

    println!("{}", result);
    */
    let mut searcher = nominatim::Searcher::new();
    let bounding_box = searcher.research("Chazey-sur-Ain, France");

    let mut request_builder = overpass::RequestBuilder::new(bounding_box);
    request_builder.set_details_level(2);
    println!("{}", request_builder.get_request_txt(Some(true)));

}