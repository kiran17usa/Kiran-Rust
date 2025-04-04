fn drink(beverage:&str){
    if beverage =="lemonade"{
        panic!("AAAaaaaaaa!!!");
    }
    println!("Some refreshing {} is all i need", beverage);
}
fn main(){
    drink("water");
    drink("lemonade");
    drink("still water");
}