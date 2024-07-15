fn main() {   //main is the entry point in rust
    println!("Hello, world!");
    first_fn();

        //passing single parameter
        second_fn(34);

        //passing multiple paramteres
        third_fn(48 , 'S');

        ex();

        //calling function
        let xy = return_value();
        println!("the value xpresiion is {}",xy);

}
//simple function
fn first_fn(){
    println!("new function");

}

//pass single paramter
fn second_fn(x : i32) {   //i32 means expecting a integer
    println!("the value of x is - {}", x);
    
}

//pass multiple paramters  //examples of statemenets
fn third_fn(x : i32 , y: char){
    println!("the value of x is - {x} and y is - {y}");
}


//expresiions

fn ex(){
    let y = {
        let x = 9;
        x+1  //will make y = 10
    };

    println!("the value of y is {}",y);
}

//return value from function

fn return_value() -> i32 {

     28+30
}

// use std::future::pending;

// fn main() {   //main is the entry point in rust
//     println!("Hello, world!");
//     first_fn();

//         //passing single parameter
//         second_fn(34);

//         //passing multiple paramteres
//         third_fn(48 , 'S');

//         ex();

//         //calling function
//         let xy = return_value();
//         println!("the value xpresiion is {}",xy);

// }


// fn first_fn() {
//     println!("new function");
// }

// //pass a single parameter
// fn second_fn(x : i32) {
//     println!("the value of x is - {}",x);
// }


// //pass multiple parameters
// fn third_fn(c : i32, y : char) {
//     println!("the value of x is - {x} and y is - {y} ");
// }

// //expressions
// fn ex() {
//     let y= {
//         let x = 9;
//         x+1;  //will make y=10
//     };
//     println!("value of y is -> {y}");
// }


// //return value from function

// fn return_value() -> i32 {
//     78 + 34
// }
