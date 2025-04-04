#[cfg(panic = "unwind")]
fn ah(){
    println!("spit it out!!!");
}
#[cfg(not(panic="unwind"))]
fn ah(){
    println!("this is not your party. Run!!!");
}
fn drink(beverage: &str){
    if beverage == "lemonade"{
        ah();
    }else{
        println!("some refreshing  {} is all i need", beverage);
    }
}
fn main(){
    drink("water");
    drink("lemonade");
}
//panic can be set from command prompt
// rustc  lemonade.rs -C panic=abort