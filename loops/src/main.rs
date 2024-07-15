// fn main() {
//     println!("HELLO Loops !");
//     first();
//     second();
//     third();
//     fourth();
// }


// //types of loops in rust
// //loop
// //while
// //for

// fn first(){
//     let mut x= 0 ;
//     loop {
//         // println!("Sahil Madaan")
//         x+=1;
//         println!("x = {}",x);

//         if x == 5{
//             println!("We did it");
//             break;
//         }
//     }
// }


// //example of while loop
// fn second(){
//     let mut num = 0 ; 

//     while num != 0{
//         println!("num = {}",num);
//         num = 1;
//     }
//     println!("HELLO");
// }


// fn third(){
//     let a = [10,20,30,40,50,60];

//     let mut index = 0 ;
//     while index<7{
//         println!("the value is:{} ",a[index]);
//         index +=1 ;
//     }

//     println!("while loop done");
// }

// //for loop
// // fn fourth(){
// //     // let mut x = 0 ;
// //     for mut x in 1..11 {
// //         if x == 5{
// //         continue;
// //     }
// //     println!{"x is {}",x};
// //     x+=1;
// // }

// // }

// fn fourth() {
//     for x in 1..11 {
//         if x == 5 {
//             continue;
//         }
//         println!("x is {}", x); // Fixed the missing brace
//     }
// }


fn main() {
    println!("HELLO Loops!");
    first();
    second(); // Note: This function might still cause an infinite loop.
    third();
    fourth();
}

//for
//while
//do while

fn first() {
    let mut x = 0;
    loop {
        x += 1;
        println!("x = {}", x);

        if x == 5 {
            println!("We did it");
            break;
        }
    }
}

fn second() {
    let mut num = 0;

    while num!= 0 {
        println!("num = {}", num);
        num -= 1; // Changed from num = 1 to decrement num instead of setting it to 1.
    }
    println!("HELLO");
}

fn third() {
    let a = [10, 20, 30, 40, 50, 60];

    let mut index = 0;
    while index < 6 { // Corrected from <7 to <6 to avoid out-of-bounds access.
        println!("the value is: {}", a[index]);
        index += 1;
    }

    println!("while loop done");
}

fn fourth() {
    for x in 1..11 {
        if x == 5 {
            continue;
        }
        println!("x is {}", x);
    }
}
