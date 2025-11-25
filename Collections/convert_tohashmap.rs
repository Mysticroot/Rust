use  std::collections::HashMap;

fn main(){

    let  vec= vec![("bob",1),("mary",2),("john",3)];

    let hashmap= to_hashmap(vec);
    println!("{:?}",hashmap);

}

fn to_hashmap(v:Vec<(&str,i32)>) -> HashMap<&str,i32> {

    let mut map= HashMap::new();

    for (key,value) in v {
        map.insert(key,value);
    }

    map
}