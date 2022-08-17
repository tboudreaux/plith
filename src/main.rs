use csv;
use std::process;
use std::error::Error;
use serde::Deserialize;
// use std::env;
// use std::fs;

// fn load_model_file(model_data: &str) {
//     let model_contents = fs::read_to_string(model_data)
//         .expect("Should be able to open model data file");
// }
//
static COEF_FILE: &[u8] = include_bytes!("coef.dat");

#[derive(Debug, Deserialize)]
struct Coef {
    Param: String,
    a: f64,
    b: f64,
    c: f64
}

fn parse_string_as_csv(bytes: &[u8]) -> Result<(), Box<dyn Error>>{
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(bytes);
    for result in rdr.deserialize() {
        let record: Coef = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    parse_string_as_csv(COEF_FILE);

}
