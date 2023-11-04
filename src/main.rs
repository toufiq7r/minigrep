use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query: &String = &args[1];
    let file: &String = &args[2];

    println!("[-] Searching: {}", query);
    println!("[-] in file: {}", file);
}
