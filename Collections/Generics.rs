

fn main(){

    println!("{}",largest_int(25,50));
    println!("{}",general_largest(10,50));
    
    println!("{}",largest_char("a","z"));
    println!("{}",general_largest("a","z"));

}

fn largest_int<T: PartialOrd>(x:T,y:T)->T{
    if x>y{
        x
    }else{
        y
    }
}

fn largest_char<T: PartialOrd>(x:T,y:T)->T{
    if x>y{
        x
    }else{
        y
    }
}

fn general_largest<T: PartialOrd>(x:T,y:T)->T{
    if x>y{
        x
    }else{
        y
    }
}