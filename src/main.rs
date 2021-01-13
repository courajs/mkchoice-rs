
use std::error::Error;




use mkchoice::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse(std::env::args().skip(1).collect())?;
    if args.help {
        eprintln!(r#"Usage: mkchoice [-h|--help] [-v|--vanish]
            [-p|--prompt <prompt>] [-s|--selection <selection>]
            [-n|--index <selected index>]
            [args] [-- <args>]

  mkchoice prompts the user's tty to choose one of the given choices,
  and outputs the chosen one. Pass - as one of the args to also read
  line-separated options from stdin. Arguments after -- are taken as
  literal choices, not interpreted as flags. If you pass no arguments,
  mkchoice will read from stdin by default.

  You can pass the initially selected value with --selection, which
  accepts the text of an option that will appear in the list. It
  defaults to the first item if the specified one can't be found. Or,
  you can pass a zero-based index with --index.

  Change the selected option with up/down or j/k, and confirm your
  selection with space or enter.

  If the --vanish flag is given, the prompt will be erased from the
  terminal before the output is shown. Otherwise, the final state of
  the prompt will still be visible on the screen.

  Example:

  $ seq 3 | mkchoice -s master a - b -p "Which one?" -- -p -h - -- master z >some-file
  Which one?
    a
    1
    2
    3
    b
    -p
    -h
    -
    --
  > master
    z
  $ cat some-file
  master"#);
        std::process::exit(1);
    }
    let stdin = std::io::stdin();
    let c = args.into_chooser(stdin.lock())?;
    let n = c.present()?;
    if let Some((_,choice)) = n {
        println!("{}", choice);
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
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

        if result.choices.is_empty() {
            result.stdin_index = Some(0);
        }

        Ok(result)
    }
    fn into_chooser(self, stdin: impl std::io::BufRead) -> std::io::Result<Chooser> {
        let mut result = Chooser {
            vanish: self.vanish,
            prompt: self.prompt.unwrap_or_else(||String::from("Choose one:")),
            choices: self.choices,
            current_choice: 0,
        };
        if let Some(index) = self.stdin_index {
            let lines = stdin.lines().collect::<std::io::Result<Vec<String>>>()?;
            result.choices.splice(index..index, lines.into_iter());
        }
        if let Some(index) = self.selected_index {
            result.current_choice = std::cmp::min(index, result.choices.len());
        }
        if let Some(val) = self.selection {
            result.set_choice(val);
        }
        Ok(result)
    }
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
