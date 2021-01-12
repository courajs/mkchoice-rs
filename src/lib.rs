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

use std::io::Write;
use std::fmt::Write as _;
use unicode_width::UnicodeWidthStr;

pub struct Chooser {
    pub vanish: bool,
    pub prompt: String,
    pub choices: Vec<String>,
    pub current_choice: usize,
}
impl Chooser {
    pub fn new(options: &[impl ToString]) -> Chooser {
        Chooser {
            vanish: true,
            prompt: String::from("Choose one:"),
            choices: options.iter().map(|s|s.to_string()).collect(),
            current_choice: 0,
        }
    }
}
impl Chooser {
    pub fn set_choice(&mut self, val: impl PartialEq<String>) -> bool {
        for i in 0..self.choices.len() {
            if val == self.choices[i] {
                self.current_choice = i;
                return true;
            }
        }
        return false;
    }
    pub fn present(mut self) -> Result<Option<(usize, String)>, std::io::Error>  {
        let mut write = termion::get_tty()?;
        write!(write, "\r{}\n", self.prompt)?;

        eprintln!("x");
        let read = write.try_clone()?;
        let raw = write.try_clone()?;
        let _raw = raw.into_raw_mode()?;
        eprintln!("y");

        write!(write, "{}", self.choice_str())?;

        let mut result = None;
        for e in read.keys() {
            match e? {
                Key::Up | Key::Char('k') => {
                    if self.current_choice > 0 {
                        self.current_choice -= 1;
                        write!(write, "\r{}{}", cursor::Up(self.choice_height()?), self.choice_str())?;
                    }
                },
                Key::Down | Key::Char('j') => {
                    if self.current_choice < self.choices.len()-1 {
                        self.current_choice += 1;
                        write!(write, "\r{}{}", cursor::Up(self.choice_height()?), self.choice_str())?;
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
            write!(write, "\r{}{}", cursor::Up(self.height()?), termion::clear::AfterCursor)?;
        }
        return Ok(result.map(|n| (n, self.choices.remove(n))));
    }

    fn choice_str(&self) -> String {
        let mut s = String::new();
        for (i, choice) in self.choices.iter().enumerate() {
            if i == self.current_choice {
                write!(s, "{}> {}{}\r\n", color::Fg(color::Green), choice, color::Fg(color::Reset)).unwrap();
            } else {
                write!(s, "  {}\r\n", choice).unwrap();
            }
        }
        s
    }

    // Total current height of the rendered text, taking into account
    // the prompt, each option, and line wrapping at the current tty width
    fn height(&self) -> Result<u16, std::io::Error> {
        eprintln!("a");
        let (t_width,_) = termion::terminal_size()?;
        eprintln!("b");
        let prompt_height = str_height(self.prompt.as_ref(), t_width);
        let choices_height: u16 = self.choices.iter().map(|choice| str_height(choice.as_ref(), t_width)).sum();
        Ok(prompt_height + choices_height)
    }
    fn choice_height(&self) -> Result<u16, std::io::Error> {
        eprintln!("a");
        let (t_width,_) = termion::terminal_size()?;
        eprintln!("b");
        Ok(self.choices.iter().map(|choice| str_height(choice.as_ref(), t_width)).sum())
    }
}

fn str_height(s: &str, terminal_width: u16) -> u16 {
    s.lines().map(|line| 1 + line.width() as u16 / terminal_width).sum()
}

