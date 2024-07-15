//importing some threads
use std::thread;
use std::time::Duration;


fn main() {
    println!("Concurrency in rust!");

    //create  a new thread to run paralley with the main function
    thread::spawn(|| {
        for i in 1..20 {
            println!("the numbers are {} from the spawned thread !",i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    //code from the main function
    for  i in 1..10 {
        println!("the numbers are {} from the main thread",i);
        thread::sleep(Duration::from_millis(1));
    }
}
