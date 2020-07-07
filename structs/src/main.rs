struct Rectangle {
    width: u32,
    height: u32
}


fn main() {
    let rect1 = Rectangle {
                    width : 30,
                    height : 35
                    };

    println!("The area of a rectangle {} by {} is {}.",
             rect1.width,
             rect1.height,
             area(&rect1));

}   


fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

