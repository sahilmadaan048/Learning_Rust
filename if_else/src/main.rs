fn main() {
    println!("if, if-else, and practice");
    first();
    second();
    third();
}


fn first(){

    let number = 10;  //earlier we took number =4 
    if number < 5{
        println!("the condition is true");
    }else{
        let abc = 4+9;
        println!("{abc}");
        println!("the condition is false");
    }
}

fn second(){
    let no = 3;

    if no % 4 == 0{
        println!("the number is divisible by 4");
    }else if no % 6 ==0{
        println!("the number is divisible by 6");
    }else if no % 8 == 0 {
        println!("the number is divisible by 8");
    }else {
        println!("this number is not divisible by any number");
    }
}

//using shorthanded condition

//kinda like ternary operator
fn third(){
    let condition = true;
    let number = if condition {5} else {0};  //ternaey operator only

    println!("the value of the number is {}",number);
}