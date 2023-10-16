use gallery_rs_utils::locate;

use async_compression::tokio::bufread::GzipDecoder;
use futures::{pin_mut, stream, Stream, StreamExt};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::LinesStream;

//lock not reproduced
#[tokio::main]
async fn main() {
    let file = File::open(locate("corrupted_utf8.gz")).await.unwrap();

    let decoder = BufReader::new(file);
    let lines = BufReader::new(GzipDecoder::new(decoder)).lines();
    let stream = LinesStream::new(lines).flat_map(stream::iter);

    process(stream).await;
}

pub async fn process(stream: impl Stream<Item = String>) {
    pin_mut!(stream);
    let mut total = 0;
    while let Some(line) = tokio_stream::StreamExt::next(&mut stream).await {
        println!("{} {}", total, line);
        total += 1;
    }
}
