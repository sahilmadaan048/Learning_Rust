// Use the "use" keyword for importing any module or functions from different files
// use rand;  // This line is commented because the `rand` crate is not used in this example

// Getting functions from another file
use lib; // Ensure that `lib.rs` file exists and contains the functions being called

// Normal module
mod my_module {
    pub fn personal() {
        println!("hello i am Sahil");
    }

    // Uncomment and complete the function if needed
    // pub fn my_info() {
    //     pub 
    // }
}

// Nested module
mod my_mod2 {
    pub mod movie {
        pub mod english {
            pub fn play(name: &str) {
                println!("playing this English movie -> {}", name);
            }
        }

        pub mod hindi {
            pub fn play(name: &str) {
                println!("playing this Hindi movie -> {}", name);
            }
        }
    }
}

use my_mod2::movie::english::play;

fn main() {
    println!("Modules Practice!");

    my_module::personal();

    // Ensure these functions are defined in `lib.rs`
    lib::print_my_message();  
    lib::print_my_message2();
    lib::print_my_message3();

    // Let's do nesting
    play("Titanic");
}
