use std::fs;
use std::error::Error;
use regex::Regex;

//static DATA_FILE: &str = "./test_data.txt";
static DATA_FILE: &str = "./input.txt";

//Box -> allocate on the heap. Boxing of errors basically preserves the original errors.
// since the compiler cannot know the type of the error at compile time, we need to tell it
// it is dynamically generated -> dyn
fn main() -> Result<(), Box<dyn Error>> {

	let input: String 	= fs::read_to_string(DATA_FILE)?;
	
	// Puzzle One
	// ++++++++++
	
	let re				= Regex::new(r"mul\(\d*,\d*\)").unwrap();
	let matches: Vec<&str> = re.find_iter(&input).map(|x| x.as_str()).collect();
	
	let mut added = 0;
	
	// multiply numbers in all matches and add up the muptiplications 	
	for amatch in matches {
		added += mul(amatch);
	}
	
	println!("Answer to puzzle one is: {}", added);
	
	// Puzzle Two
	// ++++++++++
	 
	// Match either mul(x,y) or don't() or do()
	let re							= Regex::new(r"(mul\(\d*,\d*\)|don't\(\)|do\(\))").unwrap();
	let matches2: Vec<&str>			= re.find_iter(&input).map(|x| x.as_str()).collect();
	// onlly don't() turns of processing -> start with true
	let mut process_mul: bool		= true;
	// reset the counter 
	added 							= 0; 
	
	// Status machine...
	for amatch in matches2 {
		
		if amatch.starts_with("do(") {
			process_mul = true;
		} else if amatch.starts_with("don't(") {
			process_mul = false;
		} else {
			if process_mul {
				added += mul(amatch);
			}
		}
		
	}
	
	println!("Answer to puzzle two is: {}", added);

	Ok(()) //since the main returs result
}

fn mul(_mulstr: &str) -> i32 {
	//input looks like eg. mul(11,8)
	//regex - \d* - any number of digits -> find 2 numbers separated by coma
	let re = Regex::new(r"\d*,\d*").unwrap(); 
	
	//Find 2 numbers separated by coma - split them, convert to i32 and multiply them
	let multiple:i32 = re.find(_mulstr).map(|x| x.as_str()).unwrap().split_once(',').map(
		|(x, y)| x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap()
		).unwrap();
	
	multiple
}
