
fn main() {
    println!("Vectors in Rust Practice!");

    let mut abc = vec![1,2,3,4,5,6];
    println!("{:?}",abc);

    abc.push(7);
    println!("{:?}",abc);

    println!("{}",abc[1]);

    let mut xyz = Vec::new();
    xyz.push(32);

    println!("{}",xyz[0]);
}
