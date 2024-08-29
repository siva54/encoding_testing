use cookie::Cookie;
use std::fs::File;
use std::io::BufReader;

fn main() {
    println!("Hello, world!");

    let file = "ascii.txt";
    let encoded_file = "ascii_encoded.txt";

    let mut f = File::create(encoded_file).expect("Unable to create file");
    let lines = BufReader::new(File::open(file).expect("Unable to read file"))
        .lines()
        .map(|l| l.expect("Unable to read line"))
        .map(|l| Cookie::parse_encoded(l.as_str()).map(|c| c.encoded().to_string()))
        .filter_map(|l| l.ok());

    for line in lines {
        writeln!(f, "{}", line).expect("Unable to write line");
    }
}
