struct User {
    name: String,
    company: String,
    age: u32,
}


struct home {
    house_no: u32,
    house_name: String,
    you_lived: bool,
}
fn main() {
    // println!("Hello, world!");

    let mut u1 = User {
        name: String :: from("Pankaj Rathore"),
        company: String :: from("ABC"),
         age: 100,
    };

    let mut uh1 = home{
        house_no: 455,
        house_name: String :: from("Shanti nivaas"),
        you_lived: true,
    };

    uh1.you_lived = false;
    u1.age = 200;
    println!("My name is {} and age is {} and company name is {}",u1.name, u1.age, u1.company);
}
