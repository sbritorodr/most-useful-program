use std::fs;

fn main() {
    println!("deleting myself. If you're on macos, you need to run me on a terminal!");
    fs::remove_file("most-useful-program");
}