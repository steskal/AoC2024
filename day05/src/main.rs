use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

//static DATA_FILE: &str = "./test_data.txt";
static DATA_FILE: &str = "./input.txt";


// This code is a mess - especially the function get_reordered. It took me to many attempts to figure the puzzle 2 out
// and I don't have time to clean this.
fn main() {
	let tupple_separator = '|';
	
	//let mut rule_map: HashMap<i32, i32> = HashMap::new();
	let mut rules: Vec<(i32,i32)>		= Vec::new();
	let mut middle_pages_sum			= 0;
	let mut corrected_middle_pages_sum	= 0;
	
	
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
				let middle_page_result = get_middle(&rules, &line);
				if middle_page_result.1 {
					middle_pages_sum += middle_page_result.0;
				} else {
					corrected_middle_pages_sum += middle_page_result.0;
				}
			}
		}
		
		println!("Answer to puzzle 1 is: {}", middle_pages_sum);
		println!("Answer to pussle 2 is: {}", corrected_middle_pages_sum);
		
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

// Return midle page of update and 
fn get_middle(rules: &Vec<(i32, i32)>, update: &str) -> (i32, bool) {

	let mut is_valid_update = true; // added for puzzle 2
	
	let update_vec_str: Vec<&str> = update.split(',').collect();
	// Get the update for printing parsed as numbers - made mutable for puzzle 2 
	let mut update_vec_i32: Vec<i32> = update_vec_str.iter().map(|x| x.parse::<i32>().unwrap()).collect();
	
	for i in 0..update_vec_i32.len() {
		let curr_rule = update_vec_i32.get(i).unwrap();
		// Get list of pages that are required to be before the current page accordign to the rules
		// The pages are required to be before, only if they are in the update list. If there is a rule, but the required page from that rule is not in the update list, that's ok.
		// The rules are for enforcing the order ONLY if the required-before-page from the rule is present in the update
		let req_v: Vec<i32> =rules.iter().filter(|&x| x.0 == *curr_rule).map(|x| x.1).collect();
		//Find positions of required pages in the  update. If the page is not present, give her index 0, so it appears to be there but before all other pages
		let req_in_update_idxs: Vec<usize> = req_v.iter()
			.map(|x| match update_vec_i32.iter().position(|&y| y == *x) {
				Some(y) => y as usize,
				// Puzzle 2 triggered on None
				None => {
					0 as usize}
			}).collect();
			
			for rexidx in req_in_update_idxs {
				//Compare index of current page with indexes of the required pages. All required should be smaller. Equal is not possible unless we are testing the first page of the update. If this first page has rules, but pages from those rules are not present in the update, they'v been assigned index of 0 in the previous code, that is why it is necessary to test > instead of >=.
				// Eg: in update [13], 99] 13 may have rule: 97|13. As 97 is not in the update, it had received index 0 in the req_in_update_idxs vector - positions of required pages in the upadate. 13 has also index 0 in the update, that is why the test must be rexidx > i
				if rexidx > i {
					// return 0 //removed for puzzle 2
					is_valid_update = false;
				}
			}
	}
	//added for puzzle 2 reorder, ifnot valid update
	
	if !is_valid_update {
		update_vec_i32 = get_reordered(rules, &update_vec_i32);
	}
	
	// If they are asking for middle page the input bettter has odd members in the update
	let middle_index = update_vec_i32.len() / 2;
	(*update_vec_i32.get(middle_index).unwrap(), is_valid_update)	
}

fn get_reordered(rules: &Vec<(i32, i32)>, update: &Vec<i32>) -> Vec<i32> {	
	let mut update_copy: Vec<i32> = update.iter().copied().collect();
	let mut reordered_update: Vec<i32> = Vec::new();
	// reduce the rules to only those that contains ony (x, y) tuples where both 
	// x and y are in the upddate.
	let mut filtered_rules: Vec<(i32, i32)> = rules.iter().filter(|&&x| update.contains(&x.0) && update.contains(&x.1)).copied().collect();
	
	// find that does not have a rule (doesnot require a page bofore)
	// When found, add them to the resulult vector (push).
	// Remove them also
	// from the update vector. The call this function on the reduced update vector with
	// the reduced rules
	let rules_x0: Vec<i32> = filtered_rules.iter().map(|x| x.0).collect();  
	reordered_update.extend(update_copy.iter().filter(|x| !rules_x0.contains(x)));
	update_copy.retain(|x| !reordered_update.contains(x));
	filtered_rules.retain(|x| !reordered_update.contains(&x.1));
	
	if update_copy.len() > 0 {
		reordered_update.extend(get_reordered(&filtered_rules, &update_copy));
	}
	
	reordered_update
}