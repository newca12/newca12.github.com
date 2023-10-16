use gallery_rs_utils::locate;

use async_fs::File;
use futures::{io::BufReader, pin_mut, stream, AsyncBufReadExt, Stream, StreamExt};

//lock reproduced
#[tokio::main]
async fn main() {
    let file = File::open(locate("corrupted_utf8")).await.unwrap();

    let stream = BufReader::new(file).lines().flat_map(stream::iter);

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
