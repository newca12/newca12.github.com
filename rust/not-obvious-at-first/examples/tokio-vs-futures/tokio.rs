use async_compression::tokio::bufread::GzipDecoder;

use gallery_rs_utils::locate;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

#[tokio::main]
async fn main() {
    let file = File::open(locate("123.gz")).await.unwrap();
    let decoder = BufReader::new(file);

    //this is not an iterator
    let mut lines = BufReader::new(GzipDecoder::new(decoder)).lines();

    assert_eq!(lines.next_line().await.unwrap().unwrap(), "1");
    assert_eq!(lines.next_line().await.unwrap().unwrap(), "2");
    assert_eq!(lines.next_line().await.unwrap().unwrap(), "3");
}
