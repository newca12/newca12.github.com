use async_compression::tokio::bufread::GzipDecoder;
use gallery_rs_utils::locate;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

//lock not reproduced
#[tokio::main]
async fn main() {
    let file = File::open(locate("corrupted_utf8.gz")).await.unwrap();

    let decoder = BufReader::new(file);

    //this is not an iterator
    let mut lines = BufReader::new(GzipDecoder::new(decoder)).lines();

    while let Some(line) = lines.next_line().await.unwrap() {
        println!("length = {}", line.len())
    }
}
