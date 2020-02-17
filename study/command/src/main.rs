use std::process::Command;
use std::io::{self, Write};
use std::env;

fn main() {
    call_node()
}

fn call_node() {
    let mut node = Command::new("node");
    node.arg("node/node.js").arg("rust");
    node.current_dir(env::current_dir().unwrap());

    let output = node.output().expect("failed to connect node!");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stdout().write_all(&output.stderr).unwrap();
}