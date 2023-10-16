use async_compression::tokio::bufread::GzipDecoder;

use futures::StreamExt;
use gallery_rs_utils::locate;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};
use tokio_stream::wrappers::LinesStream;

#[tokio::main]
async fn main() {
    let file = File::open(locate("123.gz")).await.unwrap();
    let decoder = BufReader::new(file);

    //this is not an iterator
    let lines = BufReader::new(GzipDecoder::new(decoder)).lines();
    //but we can wrap it to be one
    let stream = LinesStream::new(lines);

    let sum = stream.fold(0, |acc, n| async move {
        acc + n.unwrap().trim().parse::<u32>().unwrap()
    });
    assert_eq!(sum.await, 6);
}
