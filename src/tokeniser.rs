use crate::error::Error;
use crate::{ Token, TokenType };

pub struct Config {
	pub tokens: (String, String, String),
	pub strict: bool,
	pub insensitive: bool,
}

pub fn tokenise(code: String, mut config: Config) -> Result<Vec<Token>, Error> {
	let mut chars = code.chars().peekable();
	let mut pos = (0usize, 0usize);
	let mut ipos = pos;
	let mut token = String::new();
	let mut tt = 0;
	let mut tog = false;
	let mut tokens = Vec::new();

	if config.insensitive {
		config.tokens = (
			config.tokens.0.to_lowercase(),
			config.tokens.1.to_lowercase(),
			config.tokens.2.to_lowercase(),
		);
	}

	loop {
		let tpos = pos;
		if !tog {
			ipos = pos;
		}

		for ch in &mut chars {
			pos.0 += 1;

			if ch.is_whitespace() {
				if ch == '\n' {
					pos.0 = 0;
					pos.1 += 1;
				}

				break
			}

			token.push(ch);
		}

		if config.insensitive {
			token = token.to_lowercase();
		}

		if token == config.tokens.0 {
			tt += 1;
		} else if token == config.tokens.1 {
			tt += 2;
		} else if token == config.tokens.2 {
			tt += 3;
		} else if config.strict {
			let error = format!("Error: Unknown token '{}' not permitted in strict mode", token);
			return Err(Error::new(error, code, tpos))
		} else {
			token = String::new();

			if chars.peek().is_none() {
				if tog {
					return Err(Error::new("Error: Unfinished token combination".into(), code, pos))
				}

				break
			} else {
				continue
			}
		}

		if tog {
			let ttype = match tt {
				5 => TokenType::Increment,
				6 => TokenType::Input,
				7 => TokenType::PtrRight,
				9 => TokenType::Output,
				10 => TokenType::Decrement,
				11 => TokenType::LoopStart,
				13 => TokenType::PtrLeft,
				14 => TokenType::LoopEnd,
				_ => return Err(Error::new("Error: Unknown token combination".into(), code, ipos)),
			};

			tokens.push(Token::new(ttype, ipos));

			tt = 0;
		} else {
			tt <<= 2;
		}

		tog = !tog;

		token = String::new();

		if chars.peek().is_none() {
			if tog {
				return Err(Error::new("Error: Unfinished token combination".into(), code, pos))
			}

			break
		}
	}

	Ok(tokens)
}
