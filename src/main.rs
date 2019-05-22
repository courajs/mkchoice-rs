use termion;
use std::error::Error;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let mut thing = termion::get_tty()?;
    thing.write_all(b"hey!")?;
    Ok(())
}
