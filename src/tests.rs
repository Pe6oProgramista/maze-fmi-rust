use super::*;

#[test]
fn test_found_01_maze3x3() {
	let img = bmp::open("./inputs/01.maze3x3.bmp").unwrap_or_else(|e| {
		panic!("Failed to open: {}", e);
	});
	let mut maze: Maze = Maze::from(&img);
     assert_eq!(maze.find_path(3, 3).err(), Option::<MazeError>::None);
}

#[test]
fn test_found_01_maze3x3_20x20() {
	let img = bmp::open("./inputs/01.maze3x3.bmp").unwrap_or_else(|e| {
		panic!("Failed to open: {}", e);
	});
	let mut maze: Maze = Maze::from(&img);
     assert_eq!(maze.find_path(20, 20).err(), Some(MazeError::new(
		MazeErrorKind::NoEnd,
		format!("There is no end zone."),
	)));
}

#[test]
fn test_found_02_maze3x3() {
	let img = bmp::open("./inputs/02.maze3x3.bmp").unwrap_or_else(|e| {
		panic!("Failed to open: {}", e);
	});
	
	let mut maze: Maze = Maze::from(&img);
     assert_eq!(maze.find_path(3, 3).err(), Option::<MazeError>::None);
}

#[test]
fn test_found_03_maze3x3() {
	let img = bmp::open("./inputs/03.maze3x3.bmp").unwrap_or_else(|e| {
		panic!("Failed to open: {}", e);
	});
	
	let mut maze: Maze = Maze::from(&img);
     assert_eq!(maze.find_path(3, 3).err(), Some(MazeError::new(
		MazeErrorKind::NoEnd,
		format!("There is no end zone."),
	)));
}

#[test]
fn test_found_04_maze3x3() {
	let img = bmp::open("./inputs/04.maze3x3.bmp").unwrap_or_else(|e| {
		panic!("Failed to open: {}", e);
	});
	
	let mut maze: Maze = Maze::from(&img);
    assert_eq!(maze.find_path(3, 3).err(), Some(MazeError::new(
		MazeErrorKind::NoStart,
		format!("There is no start."),
	)));
}

#[test]
fn test_found_05_maze3x3() {
	let img = bmp::open("./inputs/05.maze3x3.bmp").unwrap_or_else(|e| {
		panic!("Failed to open: {}", e);
	});
	
	let mut maze: Maze = Maze::from(&img);
    assert_eq!(maze.find_path(3, 3).err(), Some(MazeError::new(
		MazeErrorKind::NoStart,
		format!("There is no start."),
	)));
}

#[test]
fn test_found_06_maze3x3() {
	let img = bmp::open("./inputs/06.maze3x3.bmp").unwrap_or_else(|e| {
		panic!("Failed to open: {}", e);
	});
	
	let mut maze: Maze = Maze::from(&img);
    assert_eq!(maze.find_path(3, 3).err(), Option::<MazeError>::None);
}

#[test]
fn test_found_07_maze3x3() {
	let img = bmp::open("./inputs/07.maze3x3.bmp").unwrap_or_else(|e| {
		panic!("Failed to open: {}", e);
	});
	
	let mut maze: Maze = Maze::from(&img);
    assert_eq!(maze.find_path(3, 3).err(), Some(MazeError::new(
		MazeErrorKind::NoEnd,
		format!("There is no end zone."),
	)));
}

#[test]
fn test_found_08_maze3x3() {
	let img = bmp::open("./inputs/08.maze3x3.bmp").unwrap_or_else(|e| {
		panic!("Failed to open: {}", e);
	});
	
	let mut maze: Maze = Maze::from(&img);
     assert_eq!(maze.find_path(3, 3).err(), Some(MazeError::new(
		MazeErrorKind::NoEnd,
		format!("There is no end zone."),
	)));
}

#[test]
fn test_found_01_maze20x20() {
	let img = bmp::open("./inputs/01.maze20x20.bmp").unwrap_or_else(|e| {
		panic!("Failed to open: {}", e);
	});
	
	let mut maze: Maze = Maze::from(&img);
    assert_eq!(maze.find_path(20, 20).err(), Option::<MazeError>::None);
}

#[test]
fn test_found_01_maze20x20_3x3() {
	let img = bmp::open("./inputs/01.maze20x20.bmp").unwrap_or_else(|e| {
		panic!("Failed to open: {}", e);
	});
	
	let mut maze: Maze = Maze::from(&img);
     assert_eq!(maze.find_path(3, 3).err(), Some(MazeError::new(
		MazeErrorKind::NoEnd,
		format!("There is no end zone."),
	)));
}

#[test]
fn test_found_02_maze20x20() {
	let img = bmp::open("./inputs/02.maze20x20.bmp").unwrap_or_else(|e| {
		panic!("Failed to open: {}", e);
	});
	
	let mut maze: Maze = Maze::from(&img);
     assert_eq!(maze.find_path(20, 20).err(), Some(MazeError::new(
		MazeErrorKind::NoEnd,
		format!("There is no end zone."),
	)));
}


// save tests

#[test]
fn test_save_01_maze3x3() {
	let img = bmp::open("./inputs/01.maze3x3.bmp").unwrap_or_else(|e| {
		panic!("Failed to open: {}", e);
	});
	
	let mut maze: Maze = Maze::from(&img);
    let _ = maze.find_path(3, 3);
	assert_eq!(maze.save_path("./outputs/01.maze3x3.bmp").err(), Option::<MazeError>::None);
}

#[test]
fn test_save_01_maze20x20() {
	let img = bmp::open("./inputs/01.maze20x20.bmp").unwrap_or_else(|e| {
		panic!("Failed to open: {}", e);
	});
	
	let mut maze: Maze = Maze::from(&img);
    let _ = maze.find_path(20, 20);
	assert_eq!(maze.save_path("./outputs/01.maze20x20.bmp").err(), Option::<MazeError>::None);
}

#[test]
fn test_save_03_maze20x20() {
	let img = bmp::open("./inputs/03.maze20x20.bmp").unwrap_or_else(|e| {
		panic!("Failed to open: {}", e);
	});
	
	let mut maze: Maze = Maze::from(&img);
    let _ = maze.find_path(20, 20);
	assert_eq!(maze.save_path("./outputs/03.maze20x20.bmp").err(), Option::<MazeError>::None);
}

#[test]
fn test_find_save_02_maze20x20() {
	let img = bmp::open("./inputs/02.maze20x20.bmp").unwrap_or_else(|e| {
		panic!("Failed to open: {}", e);
	});
	
	let mut maze: Maze = Maze::from(&img);
    assert_eq!(maze.find_path(20, 20).err(), Some(MazeError::new(
		MazeErrorKind::NoEnd,
		format!("There is no end zone."),
	)));
	assert_eq!(maze.save_path("./outputs/02.maze20x20.bmp").err(), Some(MazeError::new(
		MazeErrorKind::NoEnd,
		format!("There is no end zone."),
	)));
}