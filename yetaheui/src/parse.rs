use alloc::vec::Vec;
use unicode_segmentation::UnicodeSegmentation;

const BATCHIM_COUNTS: [u8; 138] = {
	let ㄱ: u8 = 2;
	let ㄴ: u8 = 2;
	let ㄷ: u8 = 3;
	let ㄹ: u8 = 5;
	let ㅁ: u8 = 4;
	let ㅂ: u8 = 4;
	let ㅅ: u8 = 2;
	let ㅇ: u8 = 1;
	let ㅈ: u8 = 3;
	let ㅊ: u8 = 4;
	let ㅋ: u8 = 3;
	let ㅌ: u8 = 4;
	let ㅍ: u8 = 4;
	let ㅎ: u8 = 3;
	let 반시옷: u8 = 3;
	let 여린히읗: u8 = 2;
	let 옛이응: u8 = 2;

	[
		0,
		ㄱ,
		ㄱ + ㄱ,
		ㄱ + ㅅ,
		ㄴ,
		ㄴ + ㅈ,
		ㄴ + ㅎ,
		ㄷ,
		ㄹ,
		ㄹ + ㄱ,
		ㄹ + ㅁ,
		ㄹ + ㅂ,
		ㄹ + ㅅ,
		ㄹ + ㅌ,
		ㄹ + ㅍ,
		ㄹ + ㅎ,
		ㅁ,
		ㅂ,
		ㅂ + ㅅ,
		ㅅ,
		ㅅ + ㅅ,
		ㅇ,
		ㅈ,
		ㅊ,
		ㅋ,
		ㅌ,
		ㅍ,
		ㅎ,
		//
		// ===================
		//
		ㄱ + ㄹ,
		ㄱ + ㅅ + ㄱ,
		ㄴ + ㄱ,
		ㄴ + ㄷ,
		ㄴ + ㅅ,
		ㄴ + 반시옷,
		ㄴ + ㅌ,
		ㄷ + ㄱ,
		ㄷ + ㄹ,
		ㄹ + ㄱ + ㅅ,
		ㄹ + ㄴ,
		ㄹ + ㄷ,
		ㄹ + ㄷ + ㅎ,
		ㄹ + ㄹ,
		ㄹ + ㅁ + ㄱ,
		ㄹ + ㅁ + ㅅ,
		ㄹ + ㅂ + ㅅ,
		ㄹ + ㅂ + ㅎ,
		ㄹ + ㅂ + ㅇ,
		ㄹ + ㅅ + ㅅ,
		ㄹ + 반시옷,
		ㄹ + ㅋ,
		ㄹ + 여린히읗,
		ㅁ + ㄱ,
		ㅁ + ㄹ,
		ㅁ + ㅂ,
		ㅁ + ㅅ,
		ㅁ + ㅅ + ㅅ,
		ㅁ + 반시옷,
		ㅁ + ㅊ,
		ㅁ + ㅎ,
		ㅁ + ㅇ,
		ㅂ + ㄹ,
		ㅂ + ㅍ,
		ㅂ + ㅎ,
		ㅂ + ㅇ,
		ㅅ + ㄱ,
		ㅅ + ㄷ,
		ㅅ + ㄹ,
		ㅅ + ㅂ,
		반시옷,
		ㅇ + ㄱ,
		ㅇ + ㄱ + ㄱ,
		ㅇ + ㅇ,
		ㅇ + ㅋ,
		옛이응,
		옛이응 + ㅅ,
		옛이응 + 반시옷,
		ㅍ + ㅂ,
		ㅍ + ㅇ,
		ㅎ + ㄴ,
		ㅎ + ㄹ,
		ㅎ + ㅁ,
		ㅎ + ㅂ,
		여린히읗,
		ㄱ + ㄴ,
		ㄱ + ㅂ,
		ㄱ + ㅊ,
		ㄱ + ㅋ,
		ㄱ + ㅎ,
		ㄴ + ㄴ,
		//
		// ===================
		//
		ㄴ + ㄹ,
		ㄴ + ㅊ,
		ㄷ + ㄷ,
		ㄷ + ㄷ + ㅂ,
		ㄷ + ㅂ,
		ㄷ + ㅅ,
		ㄷ + ㅅ + ㄱ,
		ㄷ + ㅈ,
		ㄷ + ㅊ,
		ㄷ + ㅌ,
		ㄹ + ㄱ + ㄱ,
		ㄹ + ㄱ + ㅎ,
		ㄹ + ㄹ + ㅋ,
		ㄹ + ㅁ + ㅎ,
		ㄹ + ㅂ + ㄷ,
		ㄹ + ㅂ + ㅍ,
		ㄹ + 옛이응,
		ㄹ + 여린히읗 + ㅎ,
		ㄹ + ㅇ,
		ㅁ + ㄴ,
		ㅁ + ㄴ + ㄴ,
		ㅁ + ㅁ,
		ㅁ + ㅂ + ㅅ,
		ㅁ + ㅈ,
		ㅂ + ㄷ,
		ㅂ + ㄹ + ㅍ,
		ㅂ + ㅁ,
		ㅂ + ㅂ,
		ㅂ + ㅅ + ㄷ,
		ㅂ + ㅈ,
		ㅂ + ㅊ,
		ㅅ + ㅁ,
		ㅅ + ㅂ + ㅇ,
		ㅅ + ㅅ + ㄱ,
		ㅅ + ㅅ + ㄷ,
		ㅅ + 반시옷,
		ㅅ + ㅈ,
		ㅅ + ㅊ,
		ㅅ + ㅌ,
		ㅅ + ㅎ,
		반시옷 + ㅂ,
		반시옷 + ㅂ + ㅇ,
		옛이응 + ㅁ,
		옛이응 + ㅎ,
		ㅈ + ㅂ,
		ㅈ + ㅂ + ㅂ,
		ㅈ + ㅈ,
		ㅍ + ㅅ,
		ㅍ + ㅌ,
	]
};

enum BatchimNum {
	Const(u8),
	Num,
	Char,
}

impl BatchimNum {
	fn from_batchim(x: u8) -> Self {
		// 이응, 옛이응
		if x == 21 || x == 73 {
			return Self::Num;
		}

		// 히읗, 여린히읗
		if x == 27 || x == 82 {
			return Self::Char;
		}

		Self::Const(BATCHIM_COUNTS[x as usize])
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
	Nil,
	End,

	Add,
	Mul,
	Sub,
	Div,
	Rem,

	Pop,
	PutNum,
	PutChar,

	Push(u8),
	GetNum,
	GetChar,

	Dup,
	Swap,

	Switch(u8),
	StoreTo(u8),
	Cmp,
	Test,
}

impl Command {
	fn from_syllable(syllable: (u8, u8, u8)) -> (Self, Direction) {
		let (a, b, c) = syllable;
		let direction = Direction::from_moeum(b);
		let batchim = BatchimNum::from_batchim(c);
		let cmd = match a {
			2 => Self::Div,
			3 => Self::Add,
			4 => Self::Mul,
			5 => Self::Rem,
			6 => match batchim {
				BatchimNum::Const(_) => Self::Pop,
				BatchimNum::Char => Self::PutChar,
				BatchimNum::Num => Self::PutNum,
			},

			//ᄇ
			7 => match batchim {
				BatchimNum::Const(x) => Self::Push(x),
				BatchimNum::Char => Self::GetChar,
				BatchimNum::Num => Self::GetNum,
			},
			//ᄈ
			8 => Self::Dup,

			//ᄉ
			9 => Self::Switch(c),
			//ᄊ
			10 => Self::StoreTo(c),
			//ᄌ
			12 => Self::Cmp,
			//ᄎ
			14 => Self::Test,
			//ᄏ

			//ᄐ
			16 => Self::Sub,
			//ᄑ
			17 => Self::Swap,
			//ᄒ
			18 => Self::End,
			_ => Self::Nil,
		};
		(cmd, direction)
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
	Left,
	Up,
	Right,
	Down,
	LeftTwice,
	UpTwice,
	RightTwice,
	DownTwice,
	FlipUpDown,
	FlipLeftRight,
	FlipAll,
	Nil,
}

impl Direction {
	fn from_moeum(x: u8) -> Self {
		match x {
			1 => Self::Right,
			3 => Self::RightTwice,
			5 => Self::Left,
			7 => Self::LeftTwice,
			9 => Self::Up,
			13 => Self::UpTwice,
			14 => Self::Down,
			18 => Self::DownTwice,
			19 => Self::FlipUpDown,
			21 => Self::FlipLeftRight,

			20 | 60 => Self::FlipAll, // U+119C라는 멋진 문명을 아시나요?

			_ => Self::Nil,
		}
	}

	pub fn to_speed(&self, (row_speed, col_speed): (i8, i8)) -> (i8, i8) {
		match self {
			Self::Left => (0, -1),
			Self::Up => (-1, 0),
			Self::Right => (0, 1),
			Self::Down => (1, 0),
			Self::LeftTwice => (0, -2),
			Self::UpTwice => (-2, 0),
			Self::RightTwice => (0, 2),
			Self::DownTwice => (2, 0),
			Self::FlipUpDown => (-row_speed, col_speed),
			Self::FlipLeftRight => (row_speed, -col_speed),
			Self::FlipAll => (-row_speed, -col_speed),
			Self::Nil => (row_speed, col_speed),
		}
	}
}

fn syllable(ch: u32) -> Option<(u8, u8, u8)> {
	if !(0xAC00..=0xD7A3).contains(&ch) {
		return None;
	}

	let x = ch - 0xAC00;
	let (x, end) = (x / 28, (x % 28) as u8);
	let (x, mid) = (x / 21, (x % 21) as u8 + 1);
	let start = x as u8;
	Some((start, mid, end))
}

struct SIRange<const N: usize>([(u32, u32); N]);

impl<const N: usize> SIRange<N> {
	// const fn start(&self) -> u32 {
	// 	self.0[0].0
	// }

	// const fn end(&self) -> u32 {
	// 	let data = &self.0;
	// 	data[data.len() - 1].0
	// }

	fn get_index(&self, item: u32) -> Option<u32> {
		let mut sum = 0;

		for (a, b) in &self.0 {
			if *a <= item && item <= *b {
				return Some(sum + (item - *a));
			}
			sum += b - a + 1;
		}

		None
	}
}

const JAMO_INITIAL: SIRange<2> = SIRange([
	// U+1100-U+1112 : 현대 한글 초성
	// U+1113-U+115F : 옛한글 초성
	(0x1100, 0x115F),
	//  옛한글 초성 확장
	(0xA960, 0xA97C),
]);

const JAMO_MEDIAL: SIRange<2> = SIRange([
	// U+1160-U+1175 : 현대 한글 중성
	// U+1176-U+11A7 : 옛한글 중성
	(0x1160, 0x11A7),
	// 옛한글 중성 확장
	(0xD7B0, 0xD7C6),
]);

const JAMO_FINAL: SIRange<2> = SIRange([
	// U+11A8-U+11C2 : 현대 한글 받침
	// U+11C3-U+11FF : 옛한글 받침
	(0x11A8, 0x11FF),
	// 옛한글 받침 확장
	(0xD7CB, 0xD7FB),
]);

// const TONE_MARKS: SIRange<1> = SIRange([
// 	// U+302E HANGUL SINGLE DOT TONE MARK
// 	// U+302F HANGUL DOUBLE DOT TONE MARK
// 	(0x302E, 0x302F),
// ]);

enum HangulChar {
	PartialInitial(u8),
	PartialMedial(u8),
	PartialFinal(u8),
	Syllable(u8, u8, u8),
}

impl HangulChar {
	fn from_char(ch: char) -> Option<Self> {
		let ch = ch as u32;

		if let Some(x) = JAMO_INITIAL.get_index(ch) {
			Some(Self::PartialInitial(x as u8))
		} else if let Some(x) = JAMO_MEDIAL.get_index(ch) {
			Some(Self::PartialMedial(x as u8))
		} else if let Some(x) = JAMO_FINAL.get_index(ch) {
			Some(Self::PartialFinal(x as u8 + 1)) // counting non-final as 0
		} else if let Some((a, b, c)) = syllable(ch) {
			Some(Self::Syllable(a, b, c))
		} else {
			None
		}
	}
}

#[derive(Default, Debug)]
pub struct CodeMatrix {
	boundary: (usize, usize),
	code: Vec<Vec<(Command, Direction)>>,
}

pub trait Space {
	type Item;
	fn boundary(&self) -> (usize, usize);
	fn get(&self, row_col: (usize, usize)) -> Option<&Self::Item>;
	fn get_row_len(&self, index: usize) -> Option<usize>;
}

trait SpaceBuilder {
	type Item;
	type Space: Space;

	fn push(&mut self, x: Self::Item);
	fn break_line(&mut self);
	fn build(self) -> Self::Space;
}

impl SpaceBuilder for CodeMatrix {
	type Item = (Command, Direction);
	type Space = Self;

	fn push(&mut self, x: Self::Item) {
		if self.code.is_empty() {
			self.break_line();
		}

		if let Some(v) = self.code.last_mut() {
			v.push(x)
		}
	}

	fn break_line(&mut self) {
		self.code.push(Vec::new())
	}

	fn build(mut self) -> Self {
		while self.code.last().map(|v| v.is_empty()).unwrap_or(false) {
			self.code.pop();
		}

		self.boundary.0 = self.code.len();
		self.boundary.1 =
			self.code.iter().map(|v| v.len()).max().unwrap_or_default();

		self
	}
}

impl Space for CodeMatrix {
	type Item = (Command, Direction);

	fn boundary(&self) -> (usize, usize) {
		self.boundary
	}

	fn get(&self, (row, col): (usize, usize)) -> Option<&Self::Item> {
		self.code.get(row).and_then(|v| v.get(col))
	}

	fn get_row_len(&self, index: usize) -> Option<usize> {
		self.code.get(index).map(|v| v.len())
	}
}

pub fn parse(code: &str) -> CodeMatrix {
	let mut matrix = CodeMatrix::default();
	for gh in code.graphemes(true) {
		if gh.contains('\n') {
			matrix.break_line();
			continue;
		}

		let mut syl = (0, 0, 0);
		for c in gh.chars() {
			if let Some(x) = HangulChar::from_char(c) {
				match x {
					HangulChar::PartialInitial(c) => syl.0 = c,
					HangulChar::PartialMedial(c) => syl.1 = c,
					HangulChar::PartialFinal(c) => syl.2 = c,
					HangulChar::Syllable(a, b, c) => syl = (a, b, c),
				}
			}
		}
		matrix.push(Command::from_syllable(syl));
	}

	matrix.build()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_syllable_separation() {
		assert_eq!(syllable('객' as u32), Some((0, 2, 1)));
		assert_eq!(syllable('개' as u32), Some((0, 2, 0)));
		assert_eq!(syllable('갱' as u32), Some((0, 2, 21)));
		assert_eq!(syllable('갷' as u32), Some((0, 2, 27)));
	}

	fn syl_to_cmd(ch: char) -> Option<(Command, Direction)> {
		syllable(ch as u32).map(Command::from_syllable)
	}

	#[test]
	fn test_command_syl() {
		assert_eq!(
			syl_to_cmd('밥').unwrap(),
			(Command::Push(4), Direction::Right)
		);
		assert_eq!(
			syl_to_cmd('삭').unwrap(),
			(Command::Switch(1), Direction::Right)
		);
		assert_eq!(
			syl_to_cmd('시').unwrap(),
			(Command::Switch(0), Direction::FlipLeftRight)
		);
	}

	#[test]
	fn test_sirange() {
		let sirange = SIRange([(0, 3), (6, 8), (9, 12)]);
		assert_eq!(sirange.get_index(0), Some(0));
		assert_eq!(sirange.get_index(1), Some(1));
		assert_eq!(sirange.get_index(2), Some(2));
		assert_eq!(sirange.get_index(3), Some(3));
		assert_eq!(sirange.get_index(6), Some(4));
		assert_eq!(sirange.get_index(7), Some(5));
		assert_eq!(sirange.get_index(8), Some(6));
		assert_eq!(sirange.get_index(9), Some(7));
		assert_eq!(sirange.get_index(10), Some(8));
		assert_eq!(sirange.get_index(11), Some(9));
	}
}
