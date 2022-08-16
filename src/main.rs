// use std::env;
// use std::fs;

// fn load_model_file(model_data: &str) {
//     let model_contents = fs::read_to_string(model_data)
//         .expect("Should be able to open model data file");
// }

fn parse_coef_table() -> &'static str {
    let coef_table = include_str!("coef.dat");
    return coef_table;
}

fn main() {
    println!("{}", parse_coef_table());

}
