use alloc::collections::VecDeque;
use alloc::vec::Vec;

use super::parse::{Command, Direction, Space};
use nohash_hasher::IntMap;

pub type AheuiNum = i128;

pub trait AheuiIO {
	type Num;

	fn get_num(&mut self) -> Option<Self::Num>;
	fn get_char(&mut self) -> Option<char>;

	fn put_num(&mut self, num: Self::Num);
	fn put_char(&mut self, ch: char);
}

trait AheuiMemory {
	type Data;

	fn pop(&mut self) -> Option<Self::Data>;
	fn peek(&mut self) -> Option<&Self::Data>;
	fn pop_two(&mut self) -> Option<(Self::Data, Self::Data)>;

	fn push(&mut self, data: Self::Data);
	fn duplicate(&mut self) -> Option<()>;
	fn swap(&mut self) -> Option<()>;

	fn binary(
		&mut self,
		func: impl Fn(Self::Data, Self::Data) -> Self::Data,
	) -> Option<()> {
		let (first, second) = self.pop_two()?;
		self.push(func(first, second));
		Some(())
	}
}

#[derive(Debug)]
enum Mem {
	Stack(Vec<AheuiNum>),
	Deque(VecDeque<AheuiNum>),
}

impl AheuiMemory for Mem {
	type Data = AheuiNum;

	fn push(&mut self, data: Self::Data) {
		match self {
			Self::Stack(v) => v.push(data),
			Self::Deque(v) => v.push_back(data),
		}
	}

	fn peek(&mut self) -> Option<&Self::Data> {
		match self {
			Self::Stack(v) => v.last(),
			Self::Deque(v) => v.front(),
		}
	}

	fn pop(&mut self) -> Option<Self::Data> {
		match self {
			Self::Stack(v) => v.pop(),
			Self::Deque(v) => v.pop_front(),
		}
	}

	fn duplicate(&mut self) -> Option<()> {
		let peek = *self.peek()?;

		match self {
			Self::Stack(v) => v.push(peek),
			Self::Deque(v) => v.push_front(peek),
		}
		Some(())
	}

	fn swap(&mut self) -> Option<()> {
		let (first, second) = self.pop_two()?;

		match self {
			Self::Stack(v) => {
				v.push(first);
				v.push(second);
			}
			Self::Deque(v) => {
				v.push_front(first);
				v.push_front(second);
			}
		}

		Some(())
	}

	fn pop_two(&mut self) -> Option<(Self::Data, Self::Data)> {
		let size = match self {
			Self::Stack(v) => v.len(),
			Self::Deque(v) => v.len(),
		};

		if size < 2 {
			return None;
		}

		let first = self.pop()?;
		let second = self.pop()?;

		Some((first, second))
	}
}

#[derive(Debug)]
struct Storage {
	current: u8,
	mem: IntMap<u8, Mem>,
}

impl Storage {
	fn new() -> Self {
		Storage {
			current: 0,
			mem: Default::default(),
		}
	}

	fn switch(&mut self, cursor: u8) {
		self.current = cursor;
	}

	fn get_mem_mut(&mut self, cursor: Option<u8>) -> &mut Mem {
		let cursor = cursor.unwrap_or(self.current);

		self.mem.entry(cursor).or_insert_with(|| {
			if cursor == 21 || cursor == 73 {
				Mem::Deque(Default::default())
			} else {
				Mem::Stack(Default::default())
			}
		});

		self.mem.get_mut(&cursor).unwrap()
	}
}

impl AheuiMemory for Storage {
	type Data = <Mem as AheuiMemory>::Data;

	#[inline]
	fn binary(
		&mut self,
		func: impl Fn(Self::Data, Self::Data) -> Self::Data,
	) -> Option<()> {
		self.get_mem_mut(None).binary(func)
	}

	#[inline]
	fn pop(&mut self) -> Option<Self::Data> {
		self.get_mem_mut(None).pop()
	}

	#[inline]
	fn peek(&mut self) -> Option<&Self::Data> {
		self.get_mem_mut(None).peek()
	}

	#[inline]
	fn push(&mut self, data: Self::Data) {
		self.get_mem_mut(None).push(data)
	}

	#[inline]
	fn duplicate(&mut self) -> Option<()> {
		self.get_mem_mut(None).duplicate()
	}

	#[inline]
	fn swap(&mut self) -> Option<()> {
		self.get_mem_mut(None).swap()
	}

	#[inline]
	fn pop_two(&mut self) -> Option<(Self::Data, Self::Data)> {
		self.get_mem_mut(None).pop_two()
	}
}

fn next_position(
	code: &impl Space,
	pos: (usize, usize),
	speed: (i8, i8),
) -> (usize, usize) {
	if speed.1 == 0 {
		(
			((pos.0 as isize) + (speed.0 as isize))
				.rem_euclid(code.boundary().0 as isize) as usize,
			pos.1,
		)
	} else if speed.0 == 0 {
		(
			pos.0,
			((pos.1 as isize) + (speed.1 as isize))
				.rem_euclid(code.get_row_len(pos.0).unwrap_or_default() as isize)
				as usize,
		)
	} else {
		unimplemented!()
	}
}

fn flip_speed((row, col): (i8, i8)) -> (i8, i8) {
	(-row, -col)
}

pub struct Runner<
	Code: Space<Item = (Command, Direction)>,
	IO: AheuiIO<Num = AheuiNum>,
> {
	code: Code,
	io: IO,
	position: (usize, usize),
	speed: (i8, i8),
	storage: Storage,
}

impl<Code: Space<Item = (Command, Direction)>, IO: AheuiIO<Num = AheuiNum>>
	Runner<Code, IO>
{
	pub fn new(code: Code, io: IO) -> Self {
		Self {
			code,
			io,
			position: (0, 0),
			speed: (1, 0),
			storage: Storage::new(),
		}
	}
	pub fn step(&mut self) -> Option<AheuiNum> {
		if let Some((cmd, direction)) = self.code.get(self.position) {
			self.speed = direction.to_speed(self.speed);

			let res = match cmd {
				Command::Nil => Some(()),
				Command::End => return Some(self.storage.pop().unwrap_or(0)),

				Command::Add => {
					self.storage.binary(|first, second| second + first)
				}
				Command::Mul => {
					self.storage.binary(|first, second| second * first)
				}
				Command::Sub => {
					self.storage.binary(|first, second| second - first)
				}
				Command::Div => {
					self.storage.binary(|first, second| second / first)
				}
				Command::Rem => {
					self.storage.binary(|first, second| second % first)
				}

				Command::Pop => self.storage.pop().map(|_| ()),
				Command::PutNum => {
					self.storage.pop().map(|x| self.io.put_num(x))
				}
				Command::PutChar => self
					.storage
					.pop()
					.and_then(|x| u32::try_from(x).ok())
					.and_then(char::from_u32)
					.map(|x| self.io.put_char(x)),

				Command::Push(x) => {
					self.storage.push((*x) as AheuiNum);
					Some(())
				}
				Command::GetNum => {
					self.io.get_num().map(|x| self.storage.push(x))
				}
				Command::GetChar => {
					self.io.get_char().map(|x| self.storage.push(x as AheuiNum))
				}

				Command::Dup => self.storage.duplicate(),
				Command::Swap => self.storage.swap(),

				Command::Switch(cursor) => {
					self.storage.switch(*cursor);
					Some(())
				}
				Command::StoreTo(cursor) => self
					.storage
					.pop()
					.map(|x| self.storage.get_mem_mut(Some(*cursor)).push(x)),

				Command::Cmp => {
					self.storage.binary(
						|first, second| {
							if second >= first {
								1
							} else {
								0
							}
						},
					)
				}
				Command::Test => self.storage.pop().and_then(|x| {
					if x != 0 {
						Some(())
					} else {
						None
					}
				}),
			};

			if res.is_none() {
				self.speed = flip_speed(self.speed);
			}
		}

		self.position = next_position(&self.code, self.position, self.speed);

		None
	}
}
