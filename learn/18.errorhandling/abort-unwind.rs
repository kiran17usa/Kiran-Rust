fn drink(beverage:&str){
    if beverage == "lemonade"{
        if cfg!(panic ="abort"){
            println!("This is not your party. Run!!!");
        }else{
            println!("Spit it out!!!");
        }
    }else{
        println!("some refreshing {} is all i need", beverage);
    }
}
fn main()
{
    drink("water");
    drink("lemonade");
}

//can run from command line
//rustc  lemonade.rs -C panic=abort