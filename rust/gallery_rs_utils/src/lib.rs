use std::path::PathBuf;
use std::process::Command;

//#[allow(dead_code)]

pub fn run_example(example_name: &str) {
    println!("running example: {example_name}");
    let output = Command::new("cargo")
        .arg("run")
        .arg("--example")
        .arg(example_name)
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if !output.status.success() {
        let s = String::from_utf8_lossy(&output.stderr);
        panic!("{example_name} failed and stderr was:\n{}", s);
    }
}

//Trick to allow both cargo run & cargo test to locate a common resource
pub fn locate(file: &str) -> PathBuf {
    let mut assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assets_dir.push("assets");
    assets_dir.push(file);
    assets_dir
}
