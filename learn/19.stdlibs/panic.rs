fn division(dividend: i32, divisor:i32)->i32{
    if divisor ==0{
        panic!("division by zero");
    }else {
        dividend/divisor
    }
}
fn main(){
    let _x = Box::new(0i32); //heap allocated
    division(3,0);
    println!("this point wont be reached");
   // _x destroyed 
}