fn main() {
    println!("Hello REFERENCES IN RUST");
    
    let mut x =10;

    let y= &mut x;

    // let z= &mut x;
    *y = 5;
    // * z += 1;   //dereference
    println!("{}",y);
    // println!("{}",z);
}


// fn main() {
//     println!("Hello REFERENCES IN RUST");

//     let mut x = 10; // Make x mutable so we can change its value later

//     let y = &mut x; // Create a mutable reference to x

//     let z = y; // z now holds a reference to y, which is a mutable reference to x

//     *z += 1; // Dereference z to modify the value of x through y

//     println!("{}", *y); // Print the value of x through y
//     println!("{}", *z); // Print the value of x through z
// }
