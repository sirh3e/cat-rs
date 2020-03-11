use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let mut bytes = vec![];

    let args: Vec<String> = env::args().collect();
    let file_paths = Vec::from(&args[1..]);

    for file_path in file_paths.iter() {
        let mut buffer: Vec<u8> = Vec::new();

        File::open(file_path)?.read_to_end(&mut buffer)?;
        bytes.append(&mut buffer)
    }

    if let Ok(_) = std::io::stdout().write_all(&bytes) {}
    Ok(())
}
