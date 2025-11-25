

use std::collections::HashMap;

fn main(){

    let mut users=HashMap::new();

    users.insert("Alice",25);
    users.insert("Bob",30);
    users.insert("Charlie",35);

    println!("{:?}",users);

    let age_of_bob=users.get("Bob");
    
    match age_of_bob{
        Some(age)=>println!("Bob's age is {}",age),
        None=>println!("Bob not found"),
    }

    for (name,age) in &users{
        println!("{} is {} years old",name,age);
    }

    //common methods of hashmaps with their time complexities and what they do

    users.remove("Alice"); //O(1) average time complexity removes the key-value pair for "Alice"
    users.insert("David",40); //O(1) average time complexity adds key-value pair for "David"
    users.len(); //O(1) time complexity returns number of key-value pairs in the hashmap
    users.is_empty(); //O(1) time complexity checks if the hashmap is empty or not
    users.contains_key("Bob"); //O(1) average time complexity checks if "Bob" exists as a key in the hashmap
    users.entry("Eve").or_insert(28); //O(1) average time complexity inserts key "Eve" with value 28 if it doesn't exist

    users.clear(); //O(n) time complexity removes all key-value pairs from the hashmap
    users.reserve(10); //O(n) time complexity reserves capacity for at least 10 more key-value pairs in the hashmap
    users.shrink_to_fit(); //O(n) time complexity reduces the capacity of the hashmap to fit its current size
    users.iter(); //O(n) time complexity returns an iterator over the key-value pairs in the hashmap


    //it stores all key value pairs in no particular order and uses hashing to provide fast access to values based on their keys
    //average time complexity for most operations is O(1) due to hashing, but can degrade to O(n) in worst-case scenarios like many hash collisions
    //keys must implement the Eq and Hash traits to be used in a hashmap
    





    

}