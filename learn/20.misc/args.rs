use std::env;
fn main(){
    let args: Vec<String> = env::args().collect();
    println!("My path is {}", args[0]);
    println!("I got  {:?} args: {:?}", args.len()-1, &args[1..]);
}
// ./args.exe 1 2 3