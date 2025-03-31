pub fn public_function()
{
    println!("called rary's public_function()");
}
fn private_function()
{
    println!("called rarys private_functiion()");
}
pub fn indirect_access(){
    print!("called rarys indirect_access(), that \n");
    private_function();
}

//rustc --crate-type=lib rary.rs