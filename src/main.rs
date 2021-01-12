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

use mkchoice::*;

fn main() -> Result<(), Box<dyn Error>> {
    let c = Chooser::new(&["a",
                         "blahblahblahblahblahblahblahblahblahblahblahblahblahblahblahblahblahblah",
                         "c"]);
    let v = c.present();

    // dbg!(v);

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct Args {
    help: bool,
    vanish: bool,
    prompt: Option<String>,
    selection: Option<String>,
    selected_index: Option<usize>,
    stdin_index: Option<usize>,
    choices: Vec<String>,
}
impl Args {
    fn parse(args: Vec<String>) -> Result<Self, std::num::ParseIntError> {
        let mut result = Self {
            help: false,
            vanish: false,
            prompt: None,
            selection: None,
            selected_index: None,
            stdin_index: None,
            choices: Vec::new(),
        };
        let mut args = args.into_iter().enumerate();
        while let Some((i, mut arg)) = args.next() {
            match arg.as_str() {
                "--help" | "-h" => result.help = true,
                "--vanish" | "-v" => result.vanish = true,
                "--prompt" | "-p" => {
                    if let Some((_,next)) = args.next() {
                        result.prompt = Some(next);
                    }
                },
                "--selection" | "-s" => {
                    if let Some((_,next)) = args.next() {
                        result.selection = Some(next);
                    }
                },
                "--index" | "-n" => {
                    if let Some((_,next)) = args.next() {
                        result.selected_index = Some(usize::from_str_radix(&next, 10)?);
                    }
                },
                "-" => result.stdin_index = Some(i),
                s if s.starts_with("-p=") => {
                    arg.replace_range(..3, "");
                    result.prompt = Some(arg);
                },
                s if s.starts_with("--prompt=") => {
                    arg.replace_range(.."--prompt=".len(), "");
                    result.prompt = Some(arg);
                },
                s if s.starts_with("-s=") => {
                    arg.replace_range(..3, "");
                    result.selection = Some(arg);
                },
                s if s.starts_with("--selection=") => {
                    arg.replace_range(.."--selection=".len(), "");
                    result.selection = Some(arg);
                },
                s if s.starts_with("-n=") => {
                    arg.replace_range(..3, "");
                    result.selected_index = Some(usize::from_str_radix(&arg, 10)?);
                },
                s if s.starts_with("--index=") => {
                    arg.replace_range(.."--index=".len(), "");
                    result.selected_index = Some(usize::from_str_radix(&arg, 10)?);
                },
                "--" => break,
                _ => result.choices.push(arg),
            }
        }
        result.choices.extend(args.map(|(_,s)|s));

        Ok(result)
    }
    // fn into_chooser(self, stdin: impl Read) -> Chooser {
    //     let mut result = Chooser {
    //         vanish: self.vanish,
    //         prompt: self.prompt.unwrap_or_else(||String::from("Choose one:")),
    //         choices: self.choices,
    //         curent_choice: 0,
    //     };
    //     if Some(index) = self.stdin_index {

    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg_parse() {
        let args = vec!["-s=master", "a", "-", "--vanish", "b", "-p", "Which one?", "-n", "12", "--", "-p", "-h", "-", "--", "master", "z"].into_iter().map(String::from).collect();
        assert_eq!(Args::parse(args), Ok(Args {
            help: false,
            vanish: true,
            prompt: Some("Which one?".to_string()),
            selection: Some("master".to_string()),
            selected_index: Some(12),
            stdin_index: Some(2),
            choices: vec!["a", "b", "-p", "-h", "-", "--", "master", "z"].into_iter().map(String::from).collect(),
        }));
    }

    #[test]
    fn test_index_parse_error() {
        let args = vec!["--index", "abc"].into_iter().map(String::from).collect();
        assert_eq!(Args::parse(args).unwrap_err(), usize::from_str_radix("abc", 10).unwrap_err());
    }
}
