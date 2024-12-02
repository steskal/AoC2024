use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

//static DATA_FILE: &str = "./test_data.txt";
static DATA_FILE: &str = "./input.txt";


// It doesn't expect the number from the imput to be negative
// There are alwasy 2 values per line divided by space
// The source file needs have a name input.txt and be inthe same directory as the binary

fn main() {
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();
    
    // If data file can be opened - get iterator as lines
    if let Ok(lines) = read_lines(DATA_FILE) {
        // read each line from BufferReader::Lines - using flatten()
        // which would flatten all nested iterators, should there be any.
        for line in lines.flatten() {
            if let Some((first, last)) = line.split_once(' ') {
                // split using first space to tuple - only if succedes to split it (returns Some)
                col1.push(first.trim().parse().unwrap()); //unwrap() returns value or panics if string cannot be parsed
                col2.push(last.trim().parse().unwrap());
            }
        }
     
        col1.sort();
        col2.sort();
        
        // PUZZLE 1
        
        // use into_iter() combined with zip() to iterate through both of the vectors
        // map() defines what to do with the elements in each iterations - map taks a closer as an argument
        // collect returns collection (eg vector) from the iterrable result
        // Note from Puzzle 2 - needed to clone() to both vectors. Otherwise the next part would complain, I am rtying to borrow already borrowed value.
        let distances: Vec<i32> = col1.clone().into_iter().zip(col2.clone()).map(|(x,y)| (x-y).abs()).collect();
        
        let puzzle_one_answer: i32 = distances.iter().sum(); // Sum the distances
        println!("Answer to puzzle one is: {}", puzzle_one_answer);
        
        // PUZZLE 2
        
        // the filter returns the element from col2 only if it can find it in the col1. The closure in the filter is referencing iter which it self is referencing the value in the vector... (And pointers in C are confusing they say..)
        // 
        // fold() replaces for loop in this case it is calling a closure for each element. 
        // 		In this case it creates empty hashmap as initial value of the accumulator and reference to the. element (returned by the filter). Fold closure must return the accumulator for the next pass.
        // or_insert - ensures that value is in the entry by inserting it if is the first time
        // The closure passed to a filter takes reference, therefor && is required
        let counts: HashMap<i32, i32>  = col2.iter()
        	.filter(|&&x| col1.contains(&x))
         	.fold(HashMap::new(), |mut acc, &x| {
         		*acc.entry(x).or_insert(0) +=1;
           		acc
          	}
        );
        
        let similarity: Vec<i32> = col1.iter().copied().map(|x| x*counts.get(&x).unwrap_or(&0)).collect();
        
        let puzzle_two_answer: i32 = similarity.iter().sum();
        println!("Answer to puzzle two is: {}", puzzle_two_answer);
        
    } //~ if let Ok(lines) = read_lines(DATA_FILE)
}

//->  returns Result of Lines iterator of BuffRead
// where is a constraint
// where P: AsRef<Path> - P must be referencable std::path
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?; // ? short syntax for match
    Ok(io::BufReader::new(file).lines()) // Read file as lines
}
