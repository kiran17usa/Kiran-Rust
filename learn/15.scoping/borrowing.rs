fn eat_box_i32(boxed_i32:Box<i32>){
    println!("destroying box that contains{}", boxed_i32);
}
fn borrow_i32(borrowed_i32: &i32){
    println!("this int is {}", borrowed_i32);
}
fn main(){
    let boxed_i32 = Box::new(5_i32);//heap
    let stacked_i32 = 6_i32;//stack
    //borrow contents of box. ownership not taken, so contents can borrow again
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);
    {
        //ref to data contained in box
        let _ref_to_i32:&i32 = &boxed_i32;
        //cant destroy boxed i32
        //eat_box_i32(boxed_i32);
        //borrow _ref to i32
        borrow_i32(_ref_to_i32);
    }
    eat_box_i32(boxed_i32);
}