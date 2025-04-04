//the below both have same signatures but not lifetime
fn elided_input(x: &i32){
    println!("elided_input: {}", x);
}
fn annotated_input<'a>(x:&'a i32){
    println!("annotated_input:{}", x);
}

fn elided_pass(x:&i32)->&i32{x} //lifetime added implicitly
fn annotated_pass<'a>(x:&'a i32)->&'a i32{x}
fn main()
{
    let x = 3;
    elided_input(&x);
    annotated_input(&x);
    println!("elided_pass: {}", elided_pass(&x));
    println!("annotated_pass: {}", annotated_pass(&x));
}