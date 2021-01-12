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
    stdin_index: Option<usize>,
    choices: Vec<String>,
}
impl Args {
    fn new(args: Vec<String>) -> Self {
        let mut result = Self {
            help: false,
            vanish: false,
            prompt: None,
            selection: None,
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
                "--" => break,
                _ => result.choices.push(arg),
            }
        }
        result.choices.extend(args.map(|(_,s)|s));

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg_parse() {
        let args = vec!["-s=master", "a", "-", "--vanish", "b", "-p", "Which one?", "--", "-p", "-h", "-", "--", "master", "z"].into_iter().map(String::from).collect();
        assert_eq!(Args::new(args), Args {
            help: false,
            vanish: true,
            prompt: Some("Which one?".to_string()),
            selection: Some("master".to_string()),
            stdin_index: Some(2),
            choices: vec!["a", "b", "-p", "-h", "-", "--", "master", "z"].into_iter().map(String::from).collect(),
        });
    }
}
