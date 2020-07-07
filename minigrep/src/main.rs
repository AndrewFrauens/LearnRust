use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[0];

    println!("Searching for : {}", query);
    println!("File : {}", filename);
}