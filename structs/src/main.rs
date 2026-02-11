#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32{
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle)-> bool{
        self.width > other.width && self.height > other.height
    }
}

fn main(){


    let rect = Rectangle{
        width: 32,
        height: 30
    };

      let rect2 = Rectangle{
        width: 20,
        height: 24
    };

      let rect3 = Rectangle{
        width: 42,
        height: 50
    };

    println!("Rect: {:#?}", rect);

    println!("The area is: {}", rect.area());

    println!("This can hold: {}", rect.can_hold(&rect2));
    println!("This can hold: {}", rect.can_hold(&rect3));

}


