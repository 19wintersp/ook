use std::process;

#[derive(Clone, Debug)]
pub struct Error {
	message: String,
	pos: (usize, usize),
	line: String,
}

impl Error {
	pub fn new(message: String, source: String, pos: (usize, usize)) -> Self {
		Self {
			message,
			pos,
			line: source.split("\n").nth(pos.1).unwrap().into(),
		}
	}

	pub fn exit(self) -> ! {
		self.display();
		process::exit(2);
	}

	pub fn display(self) {
		let numl = ((self.pos.1 + 1) as f32).log10() as usize + 1;

		eprintln!("{}\n", self.message);
		eprintln!(" {} | {}", self.pos.1 + 1, self.line);
		eprintln!("{}^ here", "-".repeat(numl + self.pos.0 + 4));
	}
}
