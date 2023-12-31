use std::sync::Arc;

use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

async fn compute(item: Arc<str>) {
    println!("{}", item)
}

#[tokio::main]
async fn main() {
    let v: Vec<String> = vec![String::from("john"), String::from("doe")];

    let mut v_ref = Vec::<Arc<str>>::with_capacity(v.len());
    for item in &v {
        v_ref.push(Arc::from(item.as_str()));
    }

    let mut join_set = JoinSet::new();
    for item in v_ref {
        join_set.spawn(async { compute(item).await });
    }
    sleep(Duration::from_millis(100)).await;
}
