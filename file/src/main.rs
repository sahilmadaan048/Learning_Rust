use std:: {fs::File, io::Write};

fn main() {
    println!("File Handling in Rust!");

    // let's create a new file
    let mut my_file: File = File::create("output.txt").expect("could not create file");

    // write into file
    my_file.write_all(b"welcome to the rust language\n").expect("not able to write into the file");

    //a[[end the data]]
    my_file.write_all(b"hey ia m sahil madaan").expect("not able to write");
}
