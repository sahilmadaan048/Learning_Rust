// extern crate regex;

// use regex::Regex;

// fn main() {
//     println!("Regular Expressions in Rust Practice!");

//     // ex1

//     let re = Regex::new(r"\w{6}").unwrap();

//     let myname = "sahil madaan";

//     println!("found the correct: -{}", re.is_match(myname));\

//     //ex2

//     let re1: Regex = Regex::new(re: r"\w{6}").unwrap();
//     match rel.captures(text:myname) {
//         Some(caps: Captures) => println!("found the correct match -{} ",caps.get(0).unwrap().as_str()),
//         None => println!("could not found the match"),
//     }
// }
extern crate regex;

use regex::Regex;
use regex::Captures;

fn main() {
    println!("Regular Expressions in Rust Practice!");

    // ex1
    let re = Regex::new(r"\w{6}").unwrap();
    let myname = "sahil madaan";
    println!("found the correct: -{}", re.is_match(myname));

    // ex2
    let re1 = Regex::new(r"\w{6}").unwrap();
    match re1.captures(myname) {
        Some(caps) => println!("found the correct match -{} ", caps.get(0).unwrap().as_str()),
        None => println!("could not find the match"),
    }
}
