

pub trait  Summary {
    fn summarize(&self) -> String{
        String::from("Summary complete")
    }
    
}

pub trait Fix {
    fn fix(&self) -> String{
        String::from("Fix complete")
    }
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

impl Fix for User{
    fn fix(&self) -> String {
        format!("Fixing user: {}", self.name)
    }
}

fn notify<T: Summary + Fix>(item:&T){
    println!("Notification: {}",item.summarize());
}

 fn main(){
    let user1 = User{
        name:String::from("Alice"),
        age:30,
    };


    notify(&user1);

    println!("{}",user1.summarize());
 }