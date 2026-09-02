//! this demo runs with cargo run --bin heap_things
//! tests run with cargo test --bin heap_things
//! this style of comment called as inner doc comment
//! this is a doc comment for the whole crate, and will be included in the generated documentation
//! cargo doc --open will open the generated documentation in the browser

fn main() {
    // the most basic smart pointer is Box<T>,
    // which allows you to store data on the heap rather than the stack.
    // Box<T> is a smart pointer because it implements the Deref and Drop traits.
    let heap_int = Box::new(5);

    println!("heap int is {}", heap_int);

    let new_owner = heap_int;

    // the ownership of the heap_int has been moved to new_owner.
    assert_eq!(*new_owner, 5);

    // old on drop fail
    // println!("heap int is {}", heap_int);

    // == if we have multiple owners and don't know which user will drop first
    // we use Rc<T> (Reference Counted) smart pointer.
    // Rc<T> keeps track of the number of owners of a value which is stored on the heap.
    // When the last owner goes out of scope, the value will be dropped.
    // it is for single threaded scenarios. 
    // For multi-threaded scenarios, we use Arc<T> (Atomic Reference Counted) smart pointer.

    use std::rc::Rc;

    let shared_int = Rc::new(10); // this is actual heap allocation
    // Rc::new() creates a new reference counted pointer to the value 10 on the heap.
    // Rc::clone() creates a new reference counted pointer to the same value on the heap.
    // Rc::strong_count() returns the number of owners of the value on the heap.
    // Rc::weak_count() returns the number of weak references to the value on the heap.
    // Rc is basic, fast and single threaded.
    // it will not copy the data on the heap, but will just increase the reference count for
    // the pointer to the data on the heap.

    println!("ref count for is {}", Rc::strong_count(&shared_int));

    {
        let next_user = Rc::clone(&shared_int);
        println!("ref count for is {} ", Rc::strong_count(&shared_int));

        let another_user = Rc::clone(&shared_int);
        println!("ref count now {}", Rc::strong_count(&shared_int));
    }

    println!("ref count after loop is {}", Rc::strong_count(&shared_int));
    

}


#[test]
fn test_heap_int() {
    let heap_int = Box::new(6);

    println!("heap int is {}", heap_int);

    let new_owner = heap_int;

    // the ownership of the heap_int has been moved to new_owner.
    assert_eq!(*new_owner, 6);

    // old on drop fail
    // println!("heap int is {}", heap_int);
}
