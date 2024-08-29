use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("Hello, world!");

    let file = "ascii.txt";
    let encoded_file = "ascii_encoded.txt";

    let content = fs::read_to_string(file).expect("Unable to read file");
    println!("content: {}", content);
    let cookie = cookie::Cookie::new("test", &content);
    let encoded: String = match cookie::Cookie::parse_encoded(cookie.to_string()).unwrap() {
        cookie => {
            println!("parsed: {}", cookie.to_string());
            cookie::Cookie::encoded(&cookie).to_string()
        },
        _ => {
            println!("failed");
            String::new()
        },
    };
    println!("encoded: {}", encoded);

    let mut f = File::create(encoded_file).expect("Unable to create file");
    f.write_all(encoded.as_bytes())
        .expect("Unable to write file");
}
