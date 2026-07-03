fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
    let n = 100;

    // This is dynamic, located in heap.
    let a = vec![0; n];

    // normally you let a = [0; 100]; // this is static, located in stack.


    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
