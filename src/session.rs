use std::{
	path::PathBuf,
	str::FromStr,
};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Event {
	Command(Command),
	Msg(Vec<u8>),
	File {
		name: String,
		compressed: bool,
		data: Vec<u8>,
	},
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Command {
	Exit,
	Msg(String),
	File(PathBuf),
}

impl FromStr for Command {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let s = s.trim();
		if s == "exit" {
			Ok(Self::Exit)
		} else if let Some(rest) = s.strip_prefix("msg") {
			let data = rest.trim();
			if data.is_empty() {
				Err("you can't send an empty message")
			} else {
				Ok(Self::Msg(data.to_string()))
			}
		} else if let Some(rest) = s.strip_prefix("file") {
			let data = rest.trim();
			if data.is_empty() {
				Err("you must specify a file to send")
			} else {
				Ok(Self::File(data.into()))
			}
		} else {
			Err("unknown command; known commands are `msg` and `file`")
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_command() {
		macro_rules! cmd {
			(msg $msg:literal) => {
				Command::Msg($msg.to_string())
			};
			(file $file:literal) => {
				Command::File($file.into())
			};
		}

		let oks = &[
			("exit", Command::Exit),
			("  exit", Command::Exit),
			("msg hello", cmd!(msg "hello")),
			("file main.rs", cmd!(file "main.rs")),
			("  msg    what's up?   ", cmd!(msg "what's up?")),
			("file\tasdf.zip\t", cmd!(file "asdf.zip")),
		];

		let errs = &[
			"msg ",
			"file ",
			"notacommand asdfsdf",
			"MSG must be lowercase",
			"ffffff",
			"",
		];

		for (text, expected) in oks {
			let got = text.parse::<Command>();
			assert_eq!(Ok(expected), got.as_ref(),);
		}

		for text in errs {
			assert!(text.parse::<Command>().is_err(), "expected an error");
		}
	}
}
