macro_rules! say_hello{
    ()=>{ // () refers to no input args
        println!("Hello!"); //this block will replace macro
    };
}
fn main()
{
    say_hello!();
}