use termion;
use termion::{clear, cursor};
use std::error::Error;
use std::io::Write;


// struct State { choices: list of strings, position: which one we point to }
// method for State print()
// make a hardcoded State and use print() in main

fn main() -> Result<(), Box<dyn Error>> {
    let mut tty = termion::get_tty()?;
        tty.write_all(b"\n\n")?;
        tty.write_all(b"world!\n")?;
        
        let msg = format!("{}hello...\n\n", cursor::Up(2));
        tty.write_all(msg.as_bytes())?;

        let o = Options {choices: vec!["one", "two"], position: 1};
        let s = o.print();
        tty.write_all(s.as_bytes())?;

        Ok(())
}

struct Options<'a> {
    choices: Vec<&'a str>,
    position: usize
}

impl<'a> Options<'a> {
    fn print(&self) -> String {
        let mut result = String::new();
        for (i, choice) in self.choices.iter().enumerate() {
            if i == self.position {
                result.push_str("> ");
            } else {
                result.push_str("  ");
            }
            result.push_str(choice);
            result.push('\n');
        }
        result.insert_str(0,"Choose one: \n");
        return result
    }
}


