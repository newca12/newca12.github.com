use gallery_rs_utils::locate;

use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open(locate("corrupted_utf8")).unwrap();
    let stream = io::BufReader::new(file).lines();
    stream.for_each(|line| println!("{}", line.unwrap()));
}
