fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    let iter = vec.iter();
    let iter2 = iter.map(|x| x + 1);

    //used for immutable references

    for i in iter2 {
        println!("{:?}", i);
    }

    let iter3 = vec.iter();
    let iter4 = iter3.filter(|x| *x % 2 == 0);

    for i in iter4 {
        println!("Filtered value is : {:?}", i);
    }
}
