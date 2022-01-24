mod nominatim;

/*
fn test_overpass() {
    // ovm stands for Overpass (https://wiki.openstreetmap.org/wiki/Overpass_API)

    let response = reqwest::blocking::get("https://lz4.overpass-api.de/api/interpreter?data=(way(45.7073666,4.7718134,45.8082628,4.8983774)[highway=\"motorway\"];<;); out;").unwrap(); // can panic
    let response_text = response.text().unwrap(); // can panic

    println!("{}", response_text);
}
*/

fn test_nominatim() {
    let mut searcher = nominatim::Searcher::new();
    let result = searcher.research("Paris, France");

    println!("{:?}", result);
}


fn main() {
    test_nominatim();
}