
fn main(){

    let my_string= String::from("rohit");

    match find_first_a (my_string){

        Some(index) => println!("First 'a' found at index {}",index),
        None => println!("No 'a' found in the string"),

    }
}

fn find_first_a (s:String)-> Option<usize>{

    for (index,ch) in s.chars().enumerate(){

        if ch=='a'{

            return Some(index);
        }
    }

    None
}
