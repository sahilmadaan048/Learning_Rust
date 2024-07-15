// trait Printable{
//     fn print(&self);
// }

// struct Rectangle{
//     width: u32,
//     height: u32,
// }

// impl Printable for Rectangle{
//     fn print(&self){
//         println!("Rectangle : {} x {} ",self.width, self.height);
//     }
// }

// fn main() {
//     println!("Traits in Rust Practice!");

//     let rec1 = Rectangle{width: 19, height:12};
//     rec1.print();
// }


use std:: f64;

trait Hasarea {
    fn area(&self) -> f64;
}

struct Square{
    x: f64,
    y:f64,
    side: f64,
}

impl Hasarea for Square{
    fn area(&self)->f64{
        self.side*self.side
    }
}
fn main() {
    println!("Traits in Rust Practice!");

    let sq1 = Square{x: 19.2, y:12.1, side:10.1};
    println!("{}",sq1.area());
}
