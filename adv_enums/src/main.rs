#[derive(Debug)]

enum GenderCat{
    Male, Female, Transgender
}

#[derive(Debug)]

struct Person{
    name: String,
    email:String,
    age:i32,
    gender:GenderCat,
}

fn main() {
    println!("Enum advance practice");

    let person1 = Person{
        name: String::from("Sahil Madaan"),
        email:String::from("madaan.sahil27@gmail.com"),
        age:43,
        gender:GenderCat:: Male,
    };

    println!("{:?}",person1);

    let result = cal(44);
    println!("{:?}",result);
}

//option enum


/*
enum Option
    Some(T)
*/

fn cal(no: i32) -> Option<bool> {
    if no%2 == 0{
        Some(true)
    }else{
        None
    }
}