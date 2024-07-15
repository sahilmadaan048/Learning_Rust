fn main() {   //tuple are compound data type

    println!("Tuple struct in Rust!");

    struct User (u8, u8, u8);

    let mut user1 = User(5,90,34);

    println!("{}, {}, {}",user1.0,user1.1,user1.2);

    user1.1 = 23;
    println!("{}, {}, {}",user1.0,user1.1,user1.2);


    struct UserDetails (String,i16,bool);

    let user2 = UserDetails(String::from("Sahil Madaan", 48, true));
    
    println!("name is {}, age is{}, boolean is {}",user2.0, user2.1, user2.2);

}
