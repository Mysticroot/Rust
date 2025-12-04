fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    let mut iter = vec.iter();
    //used for immutable references

    //let mut iter= vec.iter_mut();
    //used for  mutable references

    while let Some(value) = iter.next() {
        println!("{:?}", value);
    }

    let iter = vec.into_iter();
    //this takes ownership of the vector and returns owned values

    for value in iter {
        println!("{:?}", value);
    }

    //println!("{}", vec.len()); //this will give error as vec is moved
}
