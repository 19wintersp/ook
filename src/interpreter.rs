use crate::error::Error;
use crate::{ Token, TokenType };

use std::io::{ self, Read };

pub fn interpret(tokens: Vec<Token>, source: String) -> Result<(), Error> {
	let len = tokens.len();

	let mut mem = [ 0u8; 65536 ];
	let mut ptr = 0usize;

	let mut ip = 0usize;

	let mut loops = Vec::<usize>::new();

	loop {
		if ip >= len { break }

		match tokens[ip].token {
			TokenType::PtrLeft => ptr -= 1,
			TokenType::PtrRight => ptr += 1,
			TokenType::Increment => mem[ptr] += 1,
			TokenType::Decrement => mem[ptr] -= 1,
			TokenType::Output => print!("{}", mem[ptr] as char),
			TokenType::Input => mem[ptr] = io::stdin().bytes().next().unwrap().unwrap(),
			TokenType::LoopStart => {
				if mem[ptr] == 0 {
					let start = tokens[ip].pos;

					loop {
						ip += 1;

						if ip >= len {
							return Err(Error::new("Error: Unmatched loop start".into(), source, start))
						}

						if tokens[ip].token == TokenType::LoopEnd {
							break
						}
					}
				} else {
					loops.push(ip);
				}
			},
			TokenType::LoopEnd => {
				if let Some(pos) = loops.last() {
					if mem[ptr] != 0 {
						ip = pos + 1;
						continue
					}
				} else {
					return Err(Error::new("Error: Unmatched loop end".into(), source, tokens[ip].pos))
				}
			},
		}

		ip += 1;
	}

	Ok(())
}
