use std::env;

fn main() {
    println!("how to pass smd arguments !");
    
    let args : Vec<String> = env::args().collect();
    // println!("{}",args[1]);

    //pass all the argguments at the run time using for loop
    for arguments in args.iter(){
        println!("{}",arguments);
    }
}
