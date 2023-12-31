use std::{fmt::Debug, sync::Arc};

fn compute(item: impl Debug + 'static) {
    println!("{:?}", item);
}

fn main() {
    let v: Vec<String> = vec![String::from("john"), String::from("doe")];

    let mut v_ref = Vec::<Arc<str>>::with_capacity(v.len());
    for item in &v {
        v_ref.push(Arc::from(item.as_str()));
    }

    for item in v_ref {
        compute(item);
    }
}
