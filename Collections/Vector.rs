//vector

fn main() {
    let mut vec = Vec::new();
    //let mut vec= vec![1,2,3,4,5];
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

    println!("{:?}", vec);

    println!("{:?}", sort_even(&vec));

    //most common mwthods of vectors as follows with their time complexities  with what do they do 

   vec.pop(); //O(1) time complexity removes last element
   vec.push(6); //O(1) time complexity adds element at the end
   vec.len(); //O(1) time complexity returns length of vector
   vec.capacity(); //O(1) time complexity returns capacity of vector
   vec.is_empty(); //O(1) time complexity checks if vector is empty
   
   vec.remove(3); //O(n) time complexity removes element at index 3
   vec.insert(2, 10); //O(n) time complexity inserts element at index 2
    vec.clear(); //O(n) time complexity clears the vector
    vec.resize(3, 0); //O(n) time complexity resizes the vector to length 3, filling new elements with 0


}

fn sort_even(v: &Vec<i32>) -> Vec<i32> {

    let mut even_vec = Vec::new();

    for val in v {
        if val % 2 == 0 {
            even_vec.push(*val);
        }
    }

    even_vec
}
