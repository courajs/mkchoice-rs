use termion::{
    self,
    raw::IntoRawMode,
    color,
    cursor,
    event::Key,
    input::{
        TermRead,
    },
};
use std::error::Error;
use std::io::Write;
use unicode_width::UnicodeWidthStr;


// struct State { choices: list of strings, position: which one we point to }
// method for State print()
// make a hardcoded State and use print() in main

fn main() -> Result<(), Box<dyn Error>> {
    let c = Chooser::new(&["a", "b", "c"]);
    let v = c.present();

    dbg!(v);

    Ok(())
}

enum MkchoiceError {
    NotATTY,
    Other(Box<dyn Error>),
}

struct Chooser<'a, T: AsRef<str>, P: AsRef<str>> {
    vanish: bool,
    prompt: P,
    choices: &'a [T],
    current_choice: usize,
}
impl<T: AsRef<str>> Chooser<'_, T, String> {
    pub fn new<'a>(options: &'a [T]) -> Chooser<'a, T, String> {
        Chooser {
            vanish: true,
            prompt: String::from("Choose one:"),
            choices: options,
            current_choice: 0,
        }
    }

}
impl<T: AsRef<str>, P: AsRef<str>> Chooser<'_, T, P> {
    pub fn present(mut self) -> Option<usize>  {
        let mut write = termion::get_tty().unwrap();
        write!(write, "\r{}\n", self.prompt.as_ref());

        let mut write = write.into_raw_mode().unwrap();
        let read = write.try_clone().unwrap();

        self.print_choices(&mut write);

        let mut result = None;
        for e in read.keys() {
            match e.unwrap() {
                Key::Up | Key::Char('k') => {
                    if self.current_choice > 0 {
                        write!(write, "\r{}", cursor::Up(self.choices.len() as u16));
                        self.current_choice -= 1;
                        self.print_choices(&mut write);
                    }
                },
                Key::Down | Key::Char('j') => {
                    if self.current_choice < self.choices.len()-1 {
                        write!(write, "\r{}", cursor::Up(self.choices.len() as u16));
                        self.current_choice += 1;
                        self.print_choices(&mut write);
                    }
                },
                Key::Char(' ') | Key::Char('\n') => {
                    result = Some(self.current_choice);
                    break;
                },
                Key::Esc | Key::Ctrl('c') => break,
                _ => (),
            }
        }

        if self.vanish {
            write!(write, "\r{}{}", cursor::Up(self.height()), termion::clear::AfterCursor);
        }
        return result;
    }

    fn height(&self) -> u16 {
        let (t_width,_) = termion::terminal_size().unwrap();
        let mut n = 0;
        for s in self.prompt.as_ref().lines() {
            n += 1 + (s.width() as u16 / t_width);
        }
        n + self.choices.len() as u16
    }

    fn print_choices(&mut self, tty: &mut impl Write) {
        for (i, choice) in self.choices.iter().enumerate() {
            if i == self.current_choice {
                write!(tty, "{}> {}{}\r\n", color::Fg(color::Green), choice.as_ref(), color::Fg(color::Reset));
            } else {
                write!(tty, "  {}\r\n", choice.as_ref());
            }
        }
    }
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
                result.push_str(&format!("{}> ", color::Fg(color::Green)));
            } else {
                result.push_str("  ");
            }
            result.push_str(choice);
            result.push_str(&format!("{}\r\n", color::Fg(color::Reset)));
        }
        result.insert_str(0,"Choose one: \r\n");
        return result
    }
}

// #[derive(Debug, Clone, PartialEq)]
// struct Args {
//     help: bool,
//     vanish: bool,
//     prompt: String,
//     selection: String,
//     stdin: Option<usize>,
//     options: Vec<String>,
// }
// 
// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     #[test]
//     fn test_arg_parse() {
//         
//     }
// }
