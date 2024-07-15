use std::fs::File;
use std::io::prelude::*;



fn main() {
    println!("Reading a file!");

    //open the file
    let mut file: Result<File, Error> = File::open(path: "../intro.txt").expect("not able to find the text file");

    //read the file
    let mut content = String ::new();

    //process the file for reading
    file.read_to_string(buf: &mut content).expect("not able to process the file reading");
    println!("file contains this data -> \n {}",content);

}
