
#[derive(Debug)]
struct Rectangle {
    length: i32,
    height: i32,
}

impl Rectangle {
    fn square(side: i32) -> Rectangle {
        Rectangle{
            length: side,
            height:side,
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
         if self.length > other.length && self.height > other.height{
             true
         } else {
             false
         }
    }

    fn area(&self) -> i32{
        self.length*self.height
    }
}


fn main(){
    
    let leng = 10;
    let heig = 20;
    let side = 5;

    let r: Rectangle = Rectangle {
        length: leng,
        height: heig,
    };

    let s = Rectangle::square(side);

    println!("Area of the square is: {}", s.area());
    println!("Can R fit S: {}", r.can_hold(&s));
}




