use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // insert soemthing
    scores.insert("tomy", 10);
    scores.insert("jane", 12);

    let person = "tomy";
    if let Some(score) = scores.get(person) {
        println!("{}'s has this score: {}", person, score
        )
    }

    // ===
    
    let numbers = vec![1,2,3];
    let mut iter = numbers.iter();

    // iter.next() returns an Option<&i32>
    // so we need to use Some.
    assert_eq!(iter.next(), Some(&1));

    // it could we unwrap but it panics if iter.next() returns None.
    assert_eq!(iter.next().unwrap(), &2);

    // will bring the next value in the iterator, which is 3.
    assert!(iter.next().is_some());

    // will bring the next value in the iterator, which is None.
    assert!(iter.next().is_none());


    // This will pass if the next value in the iterator is None.
    assert_eq!(iter.next(), None);


    for val in numbers.iter() {
        println!("val = {}", val);
    }
    // ===

    let increments: Vec<i32> = numbers
        .iter() // create an iterator over the numbers vector
        .map(|x| x + 1) // increment each value by 1
        .collect(); // collect the results into a new vector

        assert_eq!(increments, vec![2, 3, 4]);
        // assert_eq (! is macro sign) is macro
        // checks if the two values are equal
        // if they are not, it will panic and print a message showing the expected and actual values.

        // similarly assert!()
        // is macro that checks if a condition is true. 
        // If the condition is false, it will panic and print a message.

}   