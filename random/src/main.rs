// extern crate rand;

// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;


// fn main() {
//     println!("Random number in Rust!");
    
//     let secretNumber = rand::thread_rng().gen_range(1..100);
//     println!("{}",secretNumber);

//     loop {
//         println!("please mter your number: ");
//         let mut guess: String = String::new();
    
//         re::stdion() stdion
//             .read_line(buf: &mut guess) Result<usize, Error>
//             .expect("failed to read user input");
    
//         let guess: u32 = match guess.trim().prase() {
//             OK(num) => num,
//             Err(_) => continue,
//         };

//         println!("your guessed number is : {}",guess);

//         match guess.cmp(&secretNumber) {
//             Ordering::Less => println!("too small number");
//             Ordering::Greater => println!("too greater number");
//             Ordering::Equal => {println!("great, you won") ;break; }
//         }
//     }
// }


extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Random number in Rust!");

    let secret_number = rand::thread_rng().gen_range(1..101); // Changed 1..100 to 1..101 for inclusive range
    println!("{}", secret_number);

    loop {
        println!("Please enter your number: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guessed number is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small number"),
            Ordering::Greater => println!("Too large number"),
            Ordering::Equal => {
                println!("Great, you won!");
                break;
            }
        }
    }
}
