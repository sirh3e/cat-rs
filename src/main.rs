use std::env;
use std::fs::File;
use std::io::{stdout, Read, Result, Write};

fn main() -> Result<()> {
    let mut buffer = vec![];

    env::args().skip(1).for_each(|path| {
        File::open(path).map(|mut file| file.read_to_end(&mut buffer));
    });

    stdout().write_all(&buffer)?;
    Ok(())
}
