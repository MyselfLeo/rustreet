mod nominatim;
mod overpass;



fn main() {
    let mut searcher = nominatim::Searcher::new();
    let mut map_data = overpass::MapData::new();

    let bounding_box = searcher.research("Villieu, France");

    let result = map_data.request(bounding_box);

    println!("{}", result);
}