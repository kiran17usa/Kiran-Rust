//all are stack allocated
//box is heap and is a smart pointer
//can be dereferenced with *
use std::mem;
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point{
    x:f64,
    y:f64,
}
struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}
fn origin()->Point{
    Point{x:0.0, y:0.0}
}
fn boxed_origin()->Box<Point>{
    Box::new(Point{x:0.0, y:0.0})
}
fn main(){
    let point:Point = origin(); //stack allcoated
    let rectangle: Rectangle = Rectangle{
        top_left: origin(),
        bottom_right: Point{x:3.0, y:-4.0}
    };
    //below heap allocated
    let boxed_rectangle:Box<Rectangle>=Box::new(Rectangle{
        top_left:origin(),
        bottom_right:Point{x:3.0, y:-4.0},
    });
    //output of fucniton can be boxed
    let boxed_point: Box<Point>=Box::new(origin());
    //double indirection
    let box_in_a_box:Box<Box<Point>> = Box::new(boxed_origin());

    println!("Point occupies {} bytes on the stack", mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes on the stack", mem::size_of_val(&rectangle));

    println!("boxed point occupies {} bytes on the stack", mem::size_of_val(&boxed_point));
    println!("boxed rectangle occupies {} bytes on the stack", mem::size_of_val(&boxed_rectangle));
    println!("boxed box occupies {} bytes ont eh stack", mem::size_of_val(&box_in_a_box));

    let unboxed_point: Point = *boxed_point;
    println!("unboxed point occupies {} bytes on the stack", mem::size_of_val(&unboxed_point));
}