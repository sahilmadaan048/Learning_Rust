fn main() {
    println!("Ownerships practice !!");
    first();
}

fn first(){
    let x =90;
    println!("{}",x);  //-->8 bit

    let y=x;    //can use clone here also
    println!("{}",y);   //-->8 bit extra not taken just x is transferred/memory transfership

    //-------------------
    let a = String :: from("Sahil Madaan");
    // let b = a;  //will take too much memory so rather do this
    
    let b = a.clone();
    println!("{a}----{b}");
}
