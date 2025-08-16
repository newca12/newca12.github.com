use async_compression::futures::bufread::GzipDecoder;
use futures::{AsyncBufReadExt, StreamExt, io::BufReader};

use async_fs::File;
use gallery_rs_utils::locate;

#[tokio::main]
async fn main() {
    let file = File::open(locate("123.gz")).await.unwrap();
    let decoder = BufReader::new(file);

    //this is an iterator
    let lines = BufReader::new(GzipDecoder::new(decoder)).lines();

    // let res = lines.for_each(|l| {
    //     println!("{:?}", l);
    //     future::ready(())
    // });
    // res.await
    let sum = lines.fold(0, |acc, n| async move {
        acc + n.unwrap().trim().parse::<u32>().unwrap()
    });
    assert_eq!(sum.await, 6);
}
