mod error;
mod interpreter;
mod token;
mod tokeniser;

use interpreter::interpret;
use token::{ Token, TokenType };
use tokeniser::{ Config, tokenise };

use std::fs;

use clap::{ clap_app, Error };

static TOKEN_A: &str = "Ook.";
static TOKEN_B: &str = "Ook!";
static TOKEN_C: &str = "Ook?";

fn main() {
	let matches = clap_app!(app =>
		(name: env!("CARGO_PKG_NAME"))
		(version: env!("CARGO_PKG_VERSION"))
		(author: env!("CARGO_PKG_AUTHORS"))
		(about: env!("CARGO_PKG_DESCRIPTION"))
		(@arg csin: -i --("case-insensitive") "Ignores case when matching tokens")
		(@arg strict: -s --strict "Enables strict mode: only tokens are permitted")
		(@arg token_a: -a --("token-a") +takes_value "Specify custom A token")
		(@arg token_b: -b --("token-b") +takes_value "Specify custom B token")
		(@arg token_c: -c --("token-c") +takes_value "Specify custom C token")
		(@arg eval: -e --eval +takes_value "Specify script to execute instead of FILE")
		(@arg FILE: +takes_value required_unless[eval] "File to execute")
	).get_matches();

	let strict = matches.is_present("strict");
	let insensitive = matches.is_present("csin");

	let token_a = matches.value_of("token_a").unwrap_or(TOKEN_A).into();
	let token_b = matches.value_of("token_b").unwrap_or(TOKEN_B).into();
	let token_c = matches.value_of("token_c").unwrap_or(TOKEN_C).into();

	let code = if let Some(file) = matches.value_of("FILE") {
		match fs::read_to_string(file) {
			Ok(data) => data,
			Err(err) => {
				let err: Error = err.into();
				err.exit();
			},
		}
	} else {
		matches.value_of("eval").unwrap().into()
	};

	let config = Config {
		tokens: (token_a, token_b, token_c),
		strict,
		insensitive,
	};

	let tokens = tokenise(code.clone(), config)
		.map_err(|err| err.exit())
		.unwrap();
	let _ = interpret(tokens, code)
		.map_err(|err| err.exit());
}
