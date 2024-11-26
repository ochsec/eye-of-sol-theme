// Rust program to write and read from a file

use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::path::Path;

fn main() -> io::Result<()> {
    let file_path = "hello_world.txt";

    // Write to the file
    let mut file = File::create(file_path)?;
    file.write_all(b"Hello, World!")?;

    // Read from the file
    let mut file = OpenOptions::new().read(true).open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    println!("File content: {}", content);

    // Clean up (optional)
    std::fs::remove_file(file_path)?;

    Ok(())
}