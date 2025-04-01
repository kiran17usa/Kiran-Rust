#![crate_type = "lib"]

#![crate_type="rary"]

pub fn public_function(){
    println!("called rary public_function()");
}
fn private_function(){
    println!("called rary private_funciton()");
}
pub fn indirect_access(){
    println!("called rary indirect_access()");
}
//while compiling no need to add --crate_type flag to rustc