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
use std::fmt::Write as _;
use unicode_width::UnicodeWidthStr;


// struct State { choices: list of strings, position: which one we point to }
// method for State print()
// make a hardcoded State and use print() in main

fn main() -> Result<(), Box<dyn Error>> {
    let c = Chooser::new(&["a",
                         "blahblahblahblahblahblahblahblahblahblahblahblahblahblahblahblahblahblah",
                         "c"]);
    let v = c.present();

    // dbg!(v);

    Ok(())
}

enum MkchoiceError {
    NotATTY,
    Other(Box<dyn Error>),
}

pub struct Chooser<'a, T: AsRef<str>, P: AsRef<str>> {
    pub vanish: bool,
    pub prompt: P,
    pub choices: &'a [T],
    pub current_choice: usize,
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
    pub fn present(mut self) -> Result<Option<usize>, std::io::Error>  {
        let mut write = termion::get_tty()?;
        write!(write, "\r{}\n", self.prompt.as_ref());

        let mut write = write.into_raw_mode()?;
        let read = write.try_clone()?;

        write!(write, "{}", self.choice_str());

        let mut result = None;
        for e in read.keys() {
            match e? {
                Key::Up | Key::Char('k') => {
                    if self.current_choice > 0 {
                        self.current_choice -= 1;
                        write!(write, "\r{}{}", cursor::Up(self.choice_height()?), self.choice_str());
                    }
                },
                Key::Down | Key::Char('j') => {
                    if self.current_choice < self.choices.len()-1 {
                        self.current_choice += 1;
                        write!(write, "\r{}{}", cursor::Up(self.choice_height()?), self.choice_str());
                    }
                },
                Key::Char(' ') | Key::Char('\n') => {
                    result = Some(self.current_choice);
                    break;
                },
                Key::Esc | Key::Char('q') | Key::Ctrl('c') => break,
                _ => (),
            }
        }

        if self.vanish {
            write!(write, "\r{}{}", cursor::Up(self.height()?), termion::clear::AfterCursor);
        }
        return Ok(result);
    }

    fn choice_str(&self) -> String {
        let mut s = String::new();
        for (i, choice) in self.choices.iter().enumerate() {
            if i == self.current_choice {
                write!(s, "{}> {}{}\r\n", color::Fg(color::Green), choice.as_ref(), color::Fg(color::Reset));
            } else {
                write!(s, "  {}\r\n", choice.as_ref());
            }
        }
        s
    }

    // Total current height of the rendered text, taking into account
    // the prompt, each option, and line wrapping at the current tty width
    fn height(&self) -> Result<u16, std::io::Error> {
        let (t_width,_) = termion::terminal_size()?;
        let prompt_height = str_height(self.prompt.as_ref(), t_width);
        let choices_height: u16 = self.choices.iter().map(|choice| str_height(choice.as_ref(), t_width)).sum();
        Ok(prompt_height + choices_height)
    }
    fn choice_height(&self) -> Result<u16, std::io::Error> {
        let (t_width,_) = termion::terminal_size()?;
        Ok(self.choices.iter().map(|choice| str_height(choice.as_ref(), t_width)).sum())
    }
}

fn str_height(s: &str, terminal_width: u16) -> u16 {
    s.lines().map(|line| 1 + line.width() as u16 / terminal_width).sum()
}

// fn str_height(s: &str, width: u16) -> Result<u16, std::io::Error> {
// 
// }

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
