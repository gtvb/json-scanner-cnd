mod token;
mod scan;

use std::fs;
use scan::Scanner;

fn main() {
    let content = fs::read_to_string("test.json").unwrap();
    let mut scanner = Scanner::new(&content);
    println!("Scanning {}", content);

    let tokens = scanner.scan();
    for t in tokens {
        println!("{:?} {:?}", t.t_type, t.t_value);
    }
}
