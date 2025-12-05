use std::thread;
// use std::time::Duration;

fn main() {
    // let handle=thread::spawn(|| {
    //     for i in 1..10{
    //         println!("spawned {}",i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // handle.join().unwrap();

    // for i in 1..5{
    //     println!("main {}",i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    let vec = vec![1, 2, 3, 4, 5];

    thread::spawn(move || {
        println!("{:?}", &vec);
    })
    .join()
    .unwrap();
}
