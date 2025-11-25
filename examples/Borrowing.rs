//Immutable ref &s1 => can be only one 
//mutable ref &mut s1 => can be multiple


fn create_string(){


    let mut s1=String::from("Hello peer");
    dosome(&mut s1);
    domore(&mut s1);

}

fn dosome(mut s2: &mut String){
    domore(&mut s2);
    println!("{}",s2);
}

fn domore(s3:&String){
    println!("{}",s3)
}


fn main(){

    create_string();
}