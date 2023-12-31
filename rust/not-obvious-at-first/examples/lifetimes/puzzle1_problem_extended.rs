use async_executor::Executor;

async fn compute<'a>(item: impl std::fmt::Debug + 'a) {
    println!("{:?}", item)
}

fn main() {
    let ex: Executor = Executor::new();
    let v: Vec<String> = vec![String::from("john"), String::from("doe")];

    let mut v_ref: Vec<&str> = Vec::<&str>::with_capacity(v.len());
    for item in &v {
        v_ref.push(item);
    }

    for item in v_ref {
        let _task = ex.spawn(async {
            compute(item).await;
        });
    }
}
