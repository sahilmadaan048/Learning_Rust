fn main() {
    println!("Data types in rust");

    //scalar type-->types where we can store single values
    //integer, strings, boolean, floating, char

    //length 8bit 16 32 64 128 arch signed(i) or unsigbed(u)-> u8, u16, 

    let no = 2;
    println!("{}",no);

    let is_male = true ;
    println!("{}",is_male);

    let char = "Abc";
    println!("{}",char);

    let dec = 78.90;
    println!("{}",dec);

    //compound types-----types where we cacn store multple data at a time
    //arrays, tuples, dictionary

    //tuples 
    let mut tup : (i32, u8, f64) = (32,43,56.50);  //will throw error if decimal value not used
    println!("{:?}",tup);  //this is the formatter
    println!("{}",tup.0);  //here this 0 is the index 

   tup.1=89;
   println!("{:?}",tup);

   //arrays
   let a = [1,2,3,4,5];
   println!("{:?}",a);
   println!("{}",a[2]);
}


