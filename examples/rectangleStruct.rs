
struct Rect{
    width:u32,
    height:u32,
}

impl Rect{

    fn area(&self) ->u32 {
        self.width * self.height
    }   

}

fn main(){

    let my_rect=Rect{
        width:30,
        height:50,
    };

    println!("Area of rectangle is {}",my_rect.area());
}