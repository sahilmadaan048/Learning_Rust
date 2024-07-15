


struct Rectangle {
    width:u32,
    height:u32,
}

//implementations are used to make the code look more clean
impl Rectangle{
    fn printdescrp(&self){
        println!("Rectangle {} multiple {}",self.width, self.height);
    }

    fn is_square(&self)->bool{
        self.width == self.height
    }

    fn cal_square(&self)->u32{
        self.width * self.height
    }
}

fn main() {
    let my_rectangle = Rectangle{width:45, height: 45};
    my_rectangle.printdescrp();

    println!("rectangke is square: {}",my_rectangle.is_square());
    println!("square area of rectangle is {}",my_rectangle.cal_square());
}
