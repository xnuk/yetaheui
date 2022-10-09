#![cfg(target_arch = "x86_64")]
use std::env;
use std::fs;
use std::io::Write;
use std::io::{self, Read};
use std::process;
use yetaheui::parse::parse;
use yetaheui::runner::{AheuiIO, AheuiNum, Runner};

struct StdinFromFileIO {
	reader: utf8_read::Reader<io::Stdin>,
	peek: Option<char>,
}

impl StdinFromFileIO {
	fn new() -> Self {
		Self {
			reader: utf8_read::Reader::new(io::stdin()),
			peek: None,
		}
	}

	fn take_char(&mut self) -> Option<char> {
		match self.reader.next_char() {
			Ok(utf8_read::Char::Char(c)) => Some(c),
			_ => None,
		}
	}

	fn pop(&mut self) -> Option<char> {
		self.peek.take().or_else(|| self.take_char())
	}

	fn peek(&mut self) -> Option<&char> {
		if self.peek.is_none() {
			self.peek = self.take_char();
		}
		self.peek.as_ref()
	}

	fn pop_while(&mut self, func: impl Fn(char) -> bool) -> Vec<char> {
		let mut ret = Vec::new();

		while let Some(peek) = self.peek() {
			if func(*peek) {
				if let Some(c) = self.pop() {
					ret.push(c);
					continue;
				}
			}
			break;
		}

		ret
	}
}

impl AheuiIO for StdinFromFileIO {
	type Num = AheuiNum;

	fn get_num(&mut self) -> Option<Self::Num> {
		// removing whitespace
		self.pop_while(|c| c.is_whitespace());

		// sign
		let sign: AheuiNum = match self.peek() {
			Some('+') => {
				self.pop();
				1
			}
			Some('-') => {
				self.pop();
				-1
			}
			_ => 1,
		};

		// numbers
		self.pop_while(|c| c.is_ascii_digit())
			.into_iter()
			.collect::<String>()
			.parse()
			.ok()
			.map(|v: AheuiNum| v * sign)
	}

	fn get_char(&mut self) -> Option<char> {
		self.pop()
	}

	fn put_num(&mut self, num: Self::Num) {
		print!("{num}");
	}

	fn put_char(&mut self, ch: char) {
		let mut stdout = io::stdout();
		stdout.write_all(ch.to_string().as_bytes()).ok();
		stdout.flush().ok();
	}
}

fn main() -> io::Result<()> {
	let code = {
		let path = env::args_os().nth(1).expect("No aheui file given");
		let mut file = fs::File::open(path)?;
		let mut reader = io::BufReader::new(&mut file);
		let mut buf = String::new();
		reader.read_to_string(&mut buf)?;
		parse(&buf)
	};

	let mut runner = Runner::new(code, StdinFromFileIO::new());

	loop {
		if let Some(code) = runner.step() {
			process::exit(code as i32);
		}
	}
}
