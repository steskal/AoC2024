use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Index;
use std::path::Path;

//static DATA_FILE: &str = "./test_data.txt";
static DATA_FILE: &str = "./input.txt";

static SEARCH_FOR: &str = "XMAS";


// Relies on matrix of characters in file being square 
// 	-> each row must have exactly as many characters as there are rows in the file
fn main() {
	
	let mut total1: i32 = 0;
	
	if let Ok(lines) = read_lines(DATA_FILE) {
		let mut rows: Vec<String> = Vec::new();

		for line in lines.flatten() {

			total1 += count_in_line(&line); // count horizontal lines
			rows.push(line.clone());
		}
		
		total1 += count_vertical(&rows);
		total1 += count_r_to_l_diagonals(&rows);
		total1 += count_l_to_r_diagonals(&rows);
	}
	
	println!("Answer to puzzle one is: {}", total1);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

// Count matches in both directions of a string
fn count_in_line(_str: &str) -> i32 {
	let mut counter: i32 = 0;
	let matchesl: Vec<&str> = _str.matches(SEARCH_FOR).collect();
	counter+= matchesl.len() as i32;
	
	let reversed: String = _str.chars().rev().collect();
	let matchesr: Vec<&str> = reversed.matches(SEARCH_FOR).collect();
	counter += matchesr.len() as i32;
	
	counter
}

// Get columns and count matches in them
fn count_vertical(_arr: &Vec<String>) -> i32 {
	let mut counter = 0;
	
	let width = _arr.index(0).len();
	
	for idx in 0..width {
		let col: String = _arr.iter().map(|x| x.chars().nth(idx).unwrap()).collect();
		
		counter += count_in_line(&col);
	}
	
	counter
}

// Get top-right to bottom-left diagonals and count matches in them
fn count_r_to_l_diagonals(_arr: &Vec<String>) -> i32 {
	let mut counter = 0;
	
	let width = _arr.index(0).len();
	let searched_width = SEARCH_FOR.len() -1; //-1 because it is for indexing. Want to start on column 3 if lenth is 4 
	
	// Walk the rows and columns of the left triangel
	for col in searched_width..width {
		let mut diag = String::new();
		for row in 0..=col {
			diag.push(_arr.index(row).chars().nth(col - row).unwrap());
		}
		counter += count_in_line(&diag);
	}
	
	// Walk the rows and columns of the right triangel
	for row in 1..(width - searched_width) {
		let mut diag = String::new();
		for col in (row..=width-1).rev() {
			diag.push(_arr.index(row+(width-col -1)).chars().nth(col).unwrap());
		}
		counter += count_in_line(&diag);
	}
	counter
}

//  Get top left to bottom right diagonala
fn count_l_to_r_diagonals(_arr: &Vec<String>) -> i32 {
	let mut counter = 0;
	
	let width = _arr.index(0).len();
	let searched_width = SEARCH_FOR.len() -1; //-1 because it is for indexing. Want to start on column 3 if lenth is 4 
	let row_start = width - searched_width -1;
	
	// walk the rows and columns in left triangel
	for row in (0..=row_start).rev() {
		let mut diag = String::new();
		for col in 0..(width - row) {
			diag.push(_arr.index(row+col).chars().nth(col).unwrap());
		}
		counter += count_in_line(&diag);
	} 
	
	// walk the rows and columns in the right triangle
	for col in 1..=row_start {
		let mut diag = String::new();
		for row in 0..width-col {
			diag.push(_arr.index(row).chars().nth(col+row).unwrap());
		}
		counter += count_in_line(&diag);
	}
	
	counter
}