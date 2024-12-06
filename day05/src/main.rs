use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

//static DATA_FILE: &str = "./test_data.txt";
static DATA_FILE: &str = "./input.txt";

fn main() {
	let tupple_separator = '|';
	
	//let mut rule_map: HashMap<i32, i32> = HashMap::new();
	let mut rules: Vec<(i32,i32)> = Vec::new();
	let mut middle_pages_sum = 0;
	
	
	//loop through file lines
	if let Ok(lines) = read_lines(DATA_FILE) {
		let mut in_rules = true;
		for line in lines.flatten() {
			// use enmpty line as separator between page rules and updates to print
			if line.trim().len() == 0 {
				in_rules = false;
				continue;
			}
			
			//these are page ordering rules
			if in_rules {
				//add tuple of (i32,i32) into vector of rules
				rules.push(split_rule(&line, tupple_separator));
			} else {
				//check if update follows the rules and add middle page to count if yes
				middle_pages_sum += is_valid_update(&rules, &line);
			}
		}
		
		println!("Answer to puzzle 1 is: {}", middle_pages_sum);
		
	}
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

// split rule into 2 parts
fn split_rule(input: &str, delim: char) -> (i32,i32) {
	let mut rule: (i32, i32) = (0,0);
	
	let split_str:(&str, &str) = input.split_once(delim).unwrap();
	// Swap the order, so  rule is first, value for rule is 2nd
	rule.0 = split_str.1.parse().unwrap();
	rule.1 = split_str.0.parse().unwrap();
	
	rule
}

// check if update print is valid
fn is_valid_update(rules: &Vec<(i32, i32)>, update: &str) -> i32 {
	
	let update_vec_str: Vec<&str> = update.split(',').collect();
	// Get the update for printing parsed as numbers
	let update_vec_i32: Vec<i32> = update_vec_str.iter().map(|x| x.parse::<i32>().unwrap()).collect();
	
	for i in 0..update_vec_i32.len() {
		let curr_rule = update_vec_i32.get(i).unwrap();
		//let req_v: Vec<i32> = rules.iter().position(|&x| x.1 == *curr_rule).map(|i| rules.get(i).unwrap().1).iter().cloned().collect();
		// Get list of pages that are required to be before the current page accordign to the rules
		// The pages are required to be before, only if they are in the update list. If there is a rule, but the required page from that rule is not in the update list, that's ok.
		// The rules are for enforcing the order ONLY if the required-before-page from the rule is present in the update
		let req_v: Vec<i32> =rules.iter().filter(|&x| x.0 == *curr_rule).map(|x| x.1).collect();
		//Find positions of required pages in the  update. If the page is not present, give her index 0, so it appears to be there but before all other pages
		let req_in_update_idxs: Vec<usize> = req_v.iter()
			.map(|x| match update_vec_i32.iter().position(|&y| y == *x) {
				Some(y) => y as usize,
				None => 0 as usize
			}).collect();
			
			for rexidx in req_in_update_idxs {
				//Compare index of current page with indexes of the required pages. All required should be smaller. Equal is not possible unless we are testing the first page of the update. If this first page has rules, but pages from those rules are not present in the update, they'v been assigned index of 0 in the previous code, that is why it is necessary to test > instead of >=.
				// Eg: in update [13], 99] 13 may have rule: 97|13. As 97 is not in the update, it had received index 0 in the req_in_update_idxs vector - positions of required pages in the upadate. 13 has also index 0 in the update, that is why the test must be rexidx > i
				if rexidx > i {
					return 0
				}
			}
	}
	
	// If they are asking for middle page the input bettter has odd members in the update
	let middle_index = update_vec_i32.len() / 2;
	*update_vec_i32.get(middle_index).unwrap()	
}