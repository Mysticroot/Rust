
fn main(){

    let vec= vec![1,2,3,4,5];

    let v1=vec.iter();

    let v2=v1.filter(|x| *x %2!=0).map(|x| x *2);

    let ans:Vec<i32>= v2.collect();

    //how to put an  iterator in another vector
    //let ans: Vec<i32>= v3.collect();

    println!("{:?}", ans);

    
}