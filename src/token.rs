#[derive(Clone, Copy, Debug)]
pub struct Token {
	pub token: TokenType,
	pub pos: (usize, usize),
}

impl Token {
	pub fn new(token: TokenType, pos: (usize, usize)) -> Self {
		Self { token, pos }
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
	PtrRight,
	PtrLeft,
	Increment,
	Decrement,
	Output,
	Input,
	LoopStart,
	LoopEnd,
}
