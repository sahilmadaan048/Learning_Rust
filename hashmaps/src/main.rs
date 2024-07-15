use std::collections::HashMap;  //way to import hashmap

fn main() {
    println!("Hashmap Practice!");

    //declaring new hashmap as a marks available
    let mut marks: HashMap<&str, i32> =  HashMap::new();

    //adding values to the hashmap
    marks.insert("rust",100);
    marks.insert("java",80);
    marks.insert("science",40);

    println!("{:?}",marks);

    //find the length
    println!("how many subjects you have studied ->{}",marks.len());

    //lets match the value
    match marks.get("science") {
        Some(mark) => println!("you got {} for the science",mark),
        None => println!("you did not study this subject"),
    }


    //remove the vlaue
    marks.remove("java");
    println!("remaining data in the hashmap marks are {:?}",marks);

    //loop through hashmap
    for (subject, mark) in &marks {
        println!("you got {} in {}",mark,subject);
    }

    //check the value
    println!("did youo study c++ ? {}",marks.contains_key("c++"));
}

