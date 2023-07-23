use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use csv::Reader;

#[allow(dead_code, unused_variables)]
pub fn read_csv_file(file_path: String) -> Result<Reader<BufReader<File>>, Box<dyn Error>> {
// Open the CSV file
let mut current_dir = std::env::current_dir()?;
current_dir.push(file_path);
// println!("Path => {:?}", current_dir);
let file = File::open(current_dir)?;
let reader = BufReader::new(file);

// Create a CSV reader
let fetched_csv_file = Reader::from_reader(reader);
Ok(fetched_csv_file)

// Iterate over each record in the CSV file
// for result in fetched_csv_file.records() {
//     let record = result?;
    
//     // Print the values of each column in the record
//     for column in record.iter() {
//         print!("{} ", column);
//     }
//     println!();
// }
}

