fn main() {
    // TODO: Change the line below to fix the compiler error.
    let x: i32 = Default::default();
    // or we can use let x :i32 = 0;
    let y: i32 = 5;
    let mut z = 6; // will get default type i32

    z = x + y;
    println!("Number {x}");
    println!("Number {y}");
    println!("Number {z}");
}
