

fn main(){

    let vec=vec![10,20,30,40,50];

    let v1= vec.iter();

    let total:i32=v1.sum();

    //this will give error as v1 is moved after sum operation
    

    println!("Total sum is : {}",total);

    // for v in v1{
    //     println!("Value is : {}",v);
    // }
}