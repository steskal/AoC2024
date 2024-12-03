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
				//} else if is_corrected_report_safe(&report) {
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

fn is_corrected_report_safe(_report: &Vec<i32>) -> bool {
	
	//lets use 0 for out of range (diff == 0 || > 3) , 1 for up adn 2 for down
	let mut direction: Vec<i8>	= Vec::new();
	
	let mut previous: i32		= -1;
	
	for it in _report.iter() {
		if previous != -1 {
			let diff = it - previous;
			
			if diff.abs() >=1 && diff.abs() <=3 {
				if diff > 0 {
					direction.push(1);
				} else {
					direction.push(2);
				}
			} else {
				direction.push(0);
			}
		}

		previous = it.clone();
	}
	
	let oor	= direction.iter().filter(|&&x| x == 0).count();
	let up	= direction.iter().filter(|&&x| x == 1).count();
	let dwn = direction.iter().filter(|&&x| x == 2).count();
	
	println!("Counts - Oor: {}, up: {}, down: {}, report.len(): {}", oor, up, dwn, _report.len());
	
	if (up == _report.len() - 2) || (dwn == _report.len() - 2) {
		let mut idx;
		
		if oor == 1 {
			idx = direction.iter().position(|&x| x == 0).unwrap();
		} else if up == 1 {
			idx = direction.iter().position(|&x| x == 1).unwrap();
		} else {
			idx = direction.iter().position(|&x| x == 2).unwrap();
		}
		
		idx += 1; // direction vector is one shor2ter compared to report
		
		let mut _reduced_report = _report.clone();
		_reduced_report.remove(idx);
		return is_report_safe(&_reduced_report);
	}
	false
}

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