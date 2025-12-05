

pub trait  Summary {
    fn summarize(&self) -> String;
    
}

struct User{
    name:String,
    age:u8,
}

impl Summary for User{

    fn summarize(&self) -> String {
        format!("User: {}, Age: {}",self.name,self.age)
    }

}

 fn main(){
    let user1 = User{
        name:String::from("Alice"),
        age:30,
    };

    println!("{}",user1.summarize());
 }