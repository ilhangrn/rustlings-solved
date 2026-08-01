trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct;
// when you comment out any impl line gives error
// because some_func expects T to have both traits implemented.
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

struct OtherStruct;
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// TODO: Fix the compiler error by only changing the signature of this function.
// Here we are going to create a function that accepts anything with these traits.
// becaude we are going call methods from that thraits.
// <T: SomeTrait + OtherTrait> says that the function `some_func` accepts a generic type
// `T` that implements both `SomeTrait` and `OtherTrait`. 
// The function then calls the methods from those traits on the passed item
fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {
// itrem: T says that we will get one input as named item
// and it will be type T, 
// which is a generic type that implements both `SomeTrait` and `OtherTrait`.

// As we know T has them, now we can call the methods from those traits on the passed item.
    item.some_function() && item.other_function()
}

fn main() {
    // You can optionally experiment here.
    println!("{}", some_func(SomeStruct));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_func() {
        assert!(some_func(SomeStruct));
        assert!(some_func(OtherStruct));
    }
}
