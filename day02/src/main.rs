use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

//static DATA_FILE: &str = "./test_data.txt";
static DATA_FILE: &str = "./input.txt";

// Expects all values in reports to be unsigned integers.
fn main() {
	
	// open the file and iterate through lines
	if let Ok(lines) = read_lines(DATA_FILE) {
		
		let mut safe_counter_one = 0;
		let mut safe_counter_two = 0;
		
		for line in lines.flatten() {
			let report: Vec<i32> = line.split(' ').map(|x| x.parse().unwrap()).collect();
			
			if is_report_safe(&report) {
				safe_counter_one += 1;
				safe_counter_two += 1;
			} else if brute_force(&report) {
				safe_counter_two += 1;
			}

		}
		println!("Answer to puzzle one is: {}", safe_counter_one);
		println!("Answer to puzzle two is: {}", safe_counter_two);
		
	}
	
}

fn read_lines<P>(filename:P) -> io::Result<io::Lines<io::BufReader<File>>>
where
	P: AsRef<Path>,
{
	let file = File::open(filename)?; // ? short syntax for match
	Ok(io::BufReader::new(file).lines()) //return
}

// Puzzle One - determine if report is safe
fn is_report_safe(_report: &Vec<i32>) -> bool {
	
	let mut asc: bool			= false;
	let mut desc: bool			= false;
	let mut previous: i32		= -1;
	
	for it in _report.iter() {
		if previous != -1 {
			let diff = it - previous;
			if diff.abs() >= 1 && diff.abs() <= 3 { // expectin only unsigned values in the report
													// two adjacent must differ at least 1 and at most 3
				if diff > 0 {
					asc = true;
				} else { //we have already established diff is >=1
					desc = true;
				}
			} else {
				return false; 
			}
		}
		previous = it.clone();
	}
	return asc ^ desc
}

// This code just iterates through report and removes one element in each pass
// until the report without it is safe or until it tries all unsuccessfully
fn brute_force(_report: &Vec<i32>) -> bool {
	
	for idx in 0.._report.len() {
		let mut reduced = _report.clone();
		reduced.remove(idx);
		
		if is_report_safe(&reduced) {
			return true
		}
	}
	return false
}