fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    let iter1 = vec.iter().filter(|x| *x % 2 == 1).map(|x| x * 2);

    let ans = iter1.collect::<Vec<_>>();

    println!("{:?}", ans);

    //println!("{}", vec.len()); //this will give error as vec is moved
}
