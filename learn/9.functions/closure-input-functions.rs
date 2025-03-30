fn call_me<F:Fn()>(f:F){
    f();
}


fn function()
{
    println!("iam in function");
}
fn main()
{
    let closure =||println!("iam in closure");
    call_me(closure);
    call_me(function);

}