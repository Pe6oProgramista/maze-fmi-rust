//#![allow(dead_code)]
//#![allow(unused_macros)]
//#![allow(unused_imports)]
//#![allow(unused_variables)]
#![allow(arithmetic_overflow)]

// external crates
extern crate bmp;
#[macro_use]
extern crate lazy_static;
extern crate bit_vec;

// used dependences
use bit_vec::BitVec;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
use std::cell::{RefCell};
use std::hash::{Hash};

// self modules
pub use error::{MazeError, MazeErrorKind, MazeResult};


// consts
const MAX_DIST: u32 = u32::MAX;
// const key_width: u32 = 20;
// const key_height: u32 = 20;
	
const WALL_COLOR: Color = Color{ r: 0, g: 0, b: 0 };
const START_COLOR: Color = Color{ r: 195, g: 195, b: 196 };
const END_COLOR: Color = Color{ r: 126, g: 127, b: 127 };
const PATH_COLOR: Color = Color{ r: 255, g: 0, b: 0 };
lazy_static! {
	static ref START_KEY_COMB: KeyComb = KeyComb::from_bytes(&[0b00000000]);
	static ref DIRS: [Coord; 4] = [Coord::new(u32::MAX, 0), Coord::new(0, u32::MAX), Coord::new(0, 1), Coord::new(1, 0)];
}

type KeyComb = BitVec;
type KeyCombRef = Rc<KeyComb>; // only rc because I dont need internal mutability
type PixelRef = Rc<RefCell<Pixel>>;

fn is_grey(color: &Color) -> bool {
	color.r == color.g && color.g == color.b
}

fn keycomb_set(key_comb: &KeyComb, pos: u32) -> KeyComb {
	let mut tmp = key_comb.clone();
	if (pos as usize) >= tmp.len() {
		tmp.grow(pos as usize - tmp.len() + 1, false);
	}
	tmp.set(pos as usize, true);
	tmp
}

fn keycomb_unset(key_comb: &KeyComb, pos: u32) -> KeyComb {
	let mut tmp = key_comb.clone();
	if (pos as usize) < tmp.len() {
		tmp.set(pos as usize, false);
	}
	tmp
}

fn keycomb_eq(key_comb1: &KeyComb, key_comb2: &KeyComb) -> bool {
	match key_comb1.len() < key_comb2.len() {
		true => {
			let mut tmp = key_comb1.clone();
			tmp.grow(key_comb2.len() - key_comb1.len(), false);
			
			return tmp == *key_comb2;
		}
		false => {
			let mut tmp = key_comb2.clone();
			tmp.grow(key_comb1.len() - key_comb2.len(), false);
			
			return tmp == *key_comb1;
		}
	};
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Color {
	r: u8,
	g: u8,
	b: u8
}

impl From<bmp::Pixel> for Color {
    fn from(pxl: bmp::Pixel) -> Self {
        Color {
			r: pxl.r,
			g: pxl.g,
			b: pxl.b,
		}
    }
}

impl Into<bmp::Pixel> for Color {
    fn into(self) -> bmp::Pixel {
        bmp::Pixel {
			r: self.r,
			g: self.g,
			b: self.b
		}
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Coord {
	row: u32,
	col: u32,
}

impl Coord {
	fn new(row: u32, col: u32) -> Self {
		Coord {
			row: row,
			col: col,
		}
	}
}

impl Default for Coord {
    fn default() -> Self {
        Coord {
			row: u32::MAX,
			col: u32::MAX,
        }
    }
}

impl AsRef<Coord> for Coord {
    fn as_ref(&self) -> &Coord {
        return self
    }
}

impl std::ops::Add<&Coord> for &Coord {
    type Output = Coord;

    fn add(self, other: &Coord) -> Coord {
        Coord {row: self.row.wrapping_add(other.row), col: self.col.wrapping_add(other.col)}
    }
}



#[derive(Clone, PartialEq, Eq, Debug)]
enum PixelType {
	UNSET,
	WALL,
	FREE,
	KEY,
	ZONE,
	START,
	END,
}

#[derive(Clone)]
struct Pixel {
	color: Color,
	p_type: PixelType,
	key_dists: HashMap<KeyCombRef, u32>
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel {
			color: Color {r: 0, g: 0, b: 0},
			p_type: PixelType::UNSET,
			key_dists: HashMap::<KeyCombRef, u32>::default(),
        }
    }
}


// maze implementation
pub struct Maze {
	key_width: u32,
	key_height: u32,
	width: u32,
	height: u32,
	end: Option<(Coord, KeyCombRef)>,
	keys: HashMap<Color, u32>,
	key_combs: HashSet<KeyCombRef>,
	pixels: Vec<PixelRef>,
}

impl Default for Maze {
	fn default() -> Self {
        Maze {
			key_width: 20,
			key_height: 20,
			width: 0,
			height: 0,
			end: None,
			keys: HashMap::default(),
			key_combs: HashSet::default(),
			pixels: Vec::default(),
		}
    }
}

impl From<&bmp::Image> for Maze {
    fn from(item: &bmp::Image) -> Self {
		let mut maze = Maze::default();
		
		maze.width = item.get_width();
		maze.height = item.get_height();
		
		maze.pixels.resize_with((maze.width * maze.height) as usize, Default::default);
		
		for (x, y) in item.coordinates() {
			maze.pixel_at(&Coord::new(y, x)).unwrap().borrow_mut().color = Color::from(item.get_pixel(x, y));
		}
		
		maze
    }
}

impl Maze {
	// private functions
	
	fn is_valid(&self, c: &Coord) -> bool {
		c.row < self.height && c.col < self.width
	}

	fn pixel_indx(&self, c: &Coord) -> MazeResult<usize> {
		if self.is_valid(c) {
			Ok((c.row * self.width + c.col) as usize)
		} else {
			Err(MazeError::new(
				MazeErrorKind::CoordOutOfRange,
				format!("Expected coords with row in [0, {:?}], col in [0, {:?}] , but passed coords: {:?}",self.height, self.width, c),
			))
		}
	}
	
	fn pixel_at(&self, c: &Coord) -> MazeResult<PixelRef> {
		let indx = self.pixel_indx(c)?;
		Ok(self.pixels[indx].clone())
	}
	
	fn set_area_at(&self, c: &Coord) {
		let pxl = self.pixel_at(c).unwrap();
		
		if pxl.borrow().color == WALL_COLOR {
			pxl.borrow_mut().p_type = PixelType::WALL;
		}
		else if is_grey(&pxl.borrow().color) {
			pxl.borrow_mut().p_type = PixelType::FREE;
		}
		else {
			let mut p_type: PixelType = PixelType::ZONE;
			if pxl.borrow().color == START_COLOR { p_type = PixelType::START; }
			if pxl.borrow().color == END_COLOR { p_type = PixelType::END; }
			
			pxl.borrow_mut().p_type = p_type.clone();
			
			let mut key_pixels: Vec<PixelRef> = Vec::default();
			key_pixels.reserve((self.key_height * self.key_width) as usize);
			let (mut max_height, mut min_height, mut max_width, mut min_width) = (c.row, c.row, c.col, c.col);
			
			let mut wave: VecDeque<Coord> = VecDeque::new();
			wave.push_back(c.clone());
			key_pixels.push(pxl);
			
			while !wave.is_empty() {
				let curr: Coord = wave.pop_front().unwrap();
				let curr_pxl_color: Color = self.pixel_at(&curr).unwrap().borrow().color;
				
				for c in DIRS.iter() {
					let nb: Coord = &curr + c;
				
					match self.pixel_at(&nb) {
						Ok(nb_pxl) => {
							if nb_pxl.borrow().color != curr_pxl_color
							|| nb_pxl.borrow().p_type != PixelType::UNSET {
								continue;
							}
							
							nb_pxl.borrow_mut().p_type = p_type.clone();
							
							wave.push_back(nb.clone());
							if max_height < nb.row { max_height = nb.row; }
							if min_height > nb.row { min_height = nb.row; }
							if max_width < nb.col { max_width = nb.col; }
							if min_width > nb.col { min_width = nb.col; }
							
							if p_type == PixelType::ZONE
								&& max_height - min_height + 1 <= self.key_height
								&& max_width - min_width + 1 <= self.key_width
								&& key_pixels.len() < (self.key_height * self.key_width) as usize
							{
								key_pixels.push(nb_pxl);
							}
							else if !key_pixels.is_empty() {
								key_pixels.clear();
							}
						},
						Err(MazeError{kind: MazeErrorKind::CoordOutOfRange, ..}) => {
							continue;
						},
						Err(e) => panic!(e)	
					}
				}
			}
			
			
			if p_type == PixelType::ZONE
				&& max_height - min_height + 1 == self.key_height
				&& max_width - min_width + 1 == self.key_width
				&& key_pixels.len() == (self.key_height * self.key_width) as usize
			{
				for p in key_pixels {
					p.borrow_mut().p_type = PixelType::KEY;
				}
			}
		}
	}
	
	fn get_start(&self) -> MazeResult<Coord> {
		for  row in 0..self.height {
			for  col in 0..self.width {
				let curr = Coord::new(row, col);
				if self.pixel_at(&curr).unwrap().borrow().color == START_COLOR {
					self.set_area_at(&curr);
					return Ok(curr)
				}
			}
		}
		
		Err(MazeError::new(
			MazeErrorKind::NoStart,
			format!("There is no start."),
		))
	}
	
	fn set_end(&mut self, ends: &Vec<Coord>) -> bool {
		if ends.len() == 0 {
			self.end = None;
			return false;
		}
		
		let mut min_dist = MAX_DIST;
		for end in ends {
			let mut curr = *end;
			let mut curr_min_dist = min_dist;
			let mut key_comb: KeyCombRef = Rc::new(START_KEY_COMB.clone());
			
			let pxl = self.pixel_at(&curr).unwrap();
			for (comb, dist) in &pxl.borrow().key_dists {
				if dist < &curr_min_dist {
					key_comb = comb.clone();
					curr_min_dist = *dist;
				}
			}
			
			loop {
				//let pxl = self.pixel_at(&curr).unwrap();
			
				let mut next = curr;
				for c in DIRS.iter() {
					let nb: Coord = &curr + c;
			
					match self.pixel_at(&nb) {
						Ok(nb_pxl) => {
							if nb_pxl.borrow().color == END_COLOR {
								match nb_pxl.borrow().key_dists.get(&key_comb) {
									Some(nb_dist) => {
										if nb_dist < &curr_min_dist {
											next = nb;
											curr_min_dist = *nb_dist;
										}
									},
									None => continue
								}
							}	
						},
						Err(MazeError{kind: MazeErrorKind::CoordOutOfRange, ..}) => {
							continue;
						},
						Err(e) => panic!(e)	
					}
				}
						
						
				if next == curr {
					if curr_min_dist < min_dist {
						self.end = Some((curr, key_comb));
						min_dist = curr_min_dist;
					}
					
					break;
				}
				
				curr = next;
			}
		}
		
		return true;
	}
	
	// public functions
	
	pub fn find_path(&mut self, key_height: u32, key_width: u32) -> MazeResult<()> {
		self.key_height = key_height;
		self.key_width = key_width;
	
		let start = self.get_start()?;
		self.key_combs.insert(Rc::new(START_KEY_COMB.clone()));
		self.pixel_at(&start).unwrap().borrow_mut().key_dists.insert(
			self.key_combs.iter().next().unwrap().clone(),
			0
		);
		
		let mut wave: VecDeque<(Coord, KeyCombRef)> = VecDeque::new();
		wave.push_back( (
			start,
			self.key_combs.iter().next().unwrap().clone()
		));
		
		let mut ends: Vec<Coord> = Vec::new();
		
		while !wave.is_empty() {
			let curr: (Coord, KeyCombRef) = wave.pop_front().unwrap();
			let curr_dist = match self.pixel_at(&curr.0).unwrap().borrow_mut().key_dists.get(&curr.1) {
				Some(dist) => *dist,
				None => return Err(MazeError::new(
						MazeErrorKind::Other,
						format!("Current pixel doesn't have current combination."),
					))
			};

			for c in DIRS.iter() {
				// взимаме съседа на текущия пиксел
				let nb: Coord = &curr.0 + c;
				
				match self.pixel_at(&nb) {
					Ok(nb_pxl) => {
						if nb_pxl.borrow().p_type == PixelType::UNSET {
							self.set_area_at(&nb);
							if nb_pxl.borrow().p_type == PixelType::END {
								ends.push(nb.clone());
							}
						}	
						
						// ако е стена я пропускаме
						if nb_pxl.borrow().p_type  == PixelType::WALL { continue; }
						
						// изчисляваме цената за преминаване в съседа
						let weight: u32 = match is_grey(&nb_pxl.borrow().color) {
							true => nb_pxl.borrow().color.r as u32,
							false => 1
						};
						
						// ако новият пиксел е цветен:
						//  - ако е ключ - добавяме го (ако вече не е добавен)
						//	- ако не е ключ - проверяваме дали има ключ с такъв цвят и дали текущата комбинация съдържа този цвят
						//	  ако не го съдържа - отиваме към следващия съсед
						//	  ако го съдържа - минаваме през него и изчисляваме новата цена
						// ако не е цветен -  минаваме през него и изчисляваме новата цена
						let mut new_key_comb = curr.1.clone();
						
						if nb_pxl.borrow().p_type == PixelType::KEY {
							let len = self.keys.len();
							let pos: &u32 = self.keys.entry(nb_pxl.borrow().color).or_insert(len as u32);

							new_key_comb = Rc::new(keycomb_set(&curr.1, *pos));
							self.key_combs.insert(new_key_comb.clone());
						}
						else if nb_pxl.borrow().p_type == PixelType::ZONE {
							
							match self.keys.get(&nb_pxl.borrow().color) {
								Some(pos) => {
									if !keycomb_eq(&keycomb_set(&new_key_comb, *pos), &new_key_comb) { continue; }
								},
								None => continue
							}
						}
						
						// ако съседния пиксел няма разстояние със новата комбинация или старото такова е по голямо от новото
						// тогава актуализираме разстоянието
						if nb_pxl.borrow().key_dists.get(&new_key_comb) == None
							|| nb_pxl.borrow().key_dists[&new_key_comb] > (curr_dist + weight)
						{
							nb_pxl.borrow_mut().key_dists.insert(new_key_comb.clone(), curr_dist + weight);
							wave.push_back( (
								nb,
								new_key_comb
							));
						}
					},
					Err(MazeError{kind: MazeErrorKind::CoordOutOfRange, ..}) => {
						continue;
					},
					Err(e) => return Err(e)
				}
			}
		}
		
		match self.set_end(&ends) {
			true => return Ok(()),
			false => return Err(MazeError::new(
				MazeErrorKind::NoEnd,
				format!("There is no end zone."),
			))
		}
	}
	
	pub fn save_path(&self, file_name: &str) -> MazeResult<()> {
		let (mut curr, mut key_comb) = match &self.end {
			Some((coord, comb)) => (*coord, comb.clone()),
			None => return Err(MazeError::new(
				MazeErrorKind::NoEnd,
				format!("There is no end zone."),
			))
		};
		
		let mut img = bmp::Image::new(self.width, self.height);

		for (x, y) in img.coordinates() {
			img.set_pixel(x, y, self.pixel_at(&Coord::new(y, x)).unwrap().borrow().color.into());
		}
		
		loop {
			let pxl = self.pixel_at(&curr).unwrap();
			
			img.set_pixel(curr.col, curr.row, PATH_COLOR.into());
			
			if pxl.borrow().p_type == PixelType::START && keycomb_eq(&key_comb, &START_KEY_COMB.clone()) { 
				break;
			}
		
			// намираме съседа с минимална дистанция от тази комбинация
            // ако сме в ключ с комбинация, която той няма, значи сме излезли от него и сме с 1 комбинация назад
            // тогава цената на следващия пиксел с новата комбинация не зависи от тази на ключа(приемаме я за MAX_DIST)
			let mut next = curr;
			let mut min_dist = MAX_DIST;
			if pxl.borrow().p_type == PixelType::KEY && pxl.borrow().key_dists.get(&key_comb) != None {
                min_dist = pxl.borrow().key_dists[&key_comb];
            }
			
			for c in DIRS.iter() {
				let nb: Coord = &curr + c;
		
				// взимаме съседния пиксел на текущия пиксел
				match self.pixel_at(&nb) {
					Ok(nb_pxl) => {
						// ако съседния пиксел има цена с текущата комбинация го обработваме
						match nb_pxl.borrow().key_dists.get(&key_comb) {
							Some(nb_dist) => {
								if nb_dist < &min_dist {
									next = nb;
									min_dist = *nb_dist;
								}
							},
							None => continue
						}
					},
					Err(MazeError{kind: MazeErrorKind::CoordOutOfRange, ..}) => {
						continue;
					},
					Err(e) => panic!(e)	
				}
			}
					
			// Ако няма съсед с по-малка дистанция:
            //  - ако сме в ключ тогава махаме цвета на ключа от комбинацията и проверяваме тогава съседите
            //  - ако сме в поле различно от ключ значи пряк няма път	
			if next == curr {
				if pxl.borrow().p_type == PixelType::KEY {
					match self.keys.get(&pxl.borrow().color) {
						Some(pos) => {
							let prev_comb = Rc::new(keycomb_unset(&key_comb, *pos));
							key_comb = self.key_combs.get(&prev_comb).unwrap().clone();
						},
						None => return Err(MazeError::new(
							MazeErrorKind::Other,
							format!("Key color not included in slef.keys"),
						))
					}
				}
				else {
					return Err(MazeError::new(
						MazeErrorKind::NoEnd,
						format!("There is no path, but self.end is not None."),
					));
				}
			}
			else {
                curr = next;
            }
		}
		
		let _ = img.save(file_name);
		
		Ok(())
	}
}


mod error;

#[cfg(test)]
mod tests;