
struct User{
    username :String,
    email:String,
    age:u8,


}

fn main(){

    let user1=User{

        username:String:: from("Rohit Patil"),
        email:String::from("Rohit@123@gmail.com"),
        age:23,
    };

    println!("Username: {}",user1.username);
    println!("Email: {}", user1.email);
    println!("Age: {}", user1.age);
}