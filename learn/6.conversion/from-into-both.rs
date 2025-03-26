use std::convert::From;

#[derive(Debug)]
struct Number{
    value: i32,
}
//implementation of From trait will automatically done for Into but vice versa is not true
//i.e., if you implement Into will not create From 
impl From<i32>for Number{
    fn from(item:i32)->Self{
        Number{value:item}
    }
}
fn main()
{
    let int = 5;
    let num: Number = int.into();
    println!("my number is {:?}", num);
}