fn destroy_box(c:Box<i32>){
    println!("destroying a box that contains {}", c);
}
fn main()
{
    let x = 5u32; //stack allocation
    let y = x; //no resources moved

    println!("x is {}, y is {}", x, y);
    let a = Box::new(5i32); //allocated on heap
    println!("a contains:{}", a);
    let b = a;//pointer address moved from a ->b
    //println!("a contains {}",a);
    destroy_box(b);
    //println!("b contains {}, b");


}