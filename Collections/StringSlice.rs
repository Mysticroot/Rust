

fn main(){
    let s = String::from("hello world");

    // let hello = &s[0..5];
    // let world = &s[6..11];

    // println!("{} {}", hello, world);

    let word = first_word(&s);
    println!("{}", word);
}

fn first_word(s: &String) -> &str {
    let mut index = 0;

    for i  in s.chars().enumerate() {
        if i.1 == ' ' {
            break
        }
        index +=1;
        
    }
    &s[0..index]
}