#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]

#[macro_use]
extern crate bmp;

extern crate maze;
use maze::Maze;
use std::rc::Rc;
fn main() {
    let img = bmp::open("./inputs/01.maze20x20.bmp").unwrap_or_else(|e| {
		panic!("Failed to open: {}", e);
	});
	
	let mut maze: Maze = Maze::from(&img);
	let _ = maze.find_path(20, 20);
	let _ = maze.save_path("./outputs/01.maze20x20.bmp");
}