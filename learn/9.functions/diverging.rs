//it never returns
fn foo()->!{
    panic!("this call never returns");
}

fn some_fn(){
    ()
}
fn main(){
    let _a:()=some_fn();
    println!("this funciton returns and you can see this line");
}
/* 
#![feature(never_type)]
fn main(){
let x: !=panic!("this call never returns");
println!("you will never see this line");
}
*/

