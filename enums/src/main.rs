enum CarTypes {  //to define a custom datatype sin tust we use enunms
    Hatchback,
    Sedan,
    SUV,
    MUV,
}

fn printCars(car: CarTypes) {
    match car {  //match is used for comparing diff cfunctionalities of same datatype efined through enum
        CarTypes::Hatchback => {
            println!("small car in a segment");
        }
        CarTypes::Sedan => {
            println!("luxury car in a segment");
        }
        CarTypes::SUV => {
            println!("sports utility vehicle");
        }
        CarTypes::MUV => {
            println!("sports in the utility vehicle");
        }
    }
}

fn main() {
    println!("enum practice !");
    printCars(CarTypes::SUV);
    printCars(CarTypes::Sedan);
}
