#[derive(Debug)]
struct Borrowed<'a>{
    x:&'a i32,
}
//Note that impl may have annotation of lifetimes too.
impl<'a> Default for Borrowed<'a>{
    fn default ()->Self{
        Self{
            x:&10,
        }
    }
}
fn main()
{
    let b:Borrowed = Default::default();
    println!("b is {:?}", b);
}