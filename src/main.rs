use std::env;
use std::fs::File;
use std::{io, io::Result};

fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    env::args().skip(1).for_each(|path| {
        match File::open(path).and_then(|mut file| io::copy(&mut file, &mut handle)) {
            Err(err) => eprintln!("{}", err),
            _ => (),
        }
    });

    Ok(())
}
