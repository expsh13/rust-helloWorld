#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    let rect1 = Rectangle {
        width: width1,
        height: height1,
    };

    // rect1は{}です
    println!("rect1 is {:#?}", rect1);

    // println!(
    //     // 長方形の面積は、{}平方ピクセルです
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rect1)
    // );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
