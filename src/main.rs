use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("\n{:?}\n",args);

    let query = &args[1];
    let filename = &args[2];

    println!("\nSearching for {}",query);
    println!("In file {}\n",filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("With text: \n{}", contents);
}
