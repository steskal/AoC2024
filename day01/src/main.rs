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
    //let mut distances: Vec<i32> = Vec::new();
    // If data file can be opened - get iterator as lines
    if let Ok(lines) = read_lines(DATA_FILE) {
        // read each line from BufferReader::Lines - using flatten()
        // which would flatten all nested iterators, should there be any.
        for line in lines.flatten() {
            if let Some((first, last)) = line.split_once(' ') {
                // split using first space to tuple - only if succedes to split it (returns Some)
                println!("First: {}, Last: {}", first, last);
                col1.push(first.trim().parse().unwrap()); //unwrap() returns value or panics if string cannot be parsed
                col2.push(last.trim().parse().unwrap());
            }
        }
        
        col1.sort();
        col2.sort();
        
        // use into_iter() combined with zip() to iterate through both of the vectors
        // map() defines what to do with the elements in each iterations - map taks a closer as an argument
        // collect returns collection (eg vector) from the iterrable result
        let distances: Vec<i32> = col1.into_iter().zip(col2).map(|(x,y)| (x-y).abs()).collect();
        
        let puzzle_one_answer: i32 = distances.iter().sum(); // Sum the distances
        println!("Answer to puzzle one is: {}", puzzle_one_answer);

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
