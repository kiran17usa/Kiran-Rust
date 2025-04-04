#[derive(Debug)]
enum Fruit{Apple, Orange, Banana, Kiwi, Lemon}
fn main(){
    let mut my_fruit: Option<Fruit>=None; //my_fruit none is assigned
    let get_lemon_as_fallback = ||{
        println!("providing lemon as fallback");
        Fruit::Lemon
    };
    let first_available_fruit = my_fruit.get_or_insert_with(get_lemon_as_fallback); //my_fruit is assigned with first come
    println!("first_available_fruit is {:?}", first_available_fruit);
    println!("my_fruit is: {:?}", my_fruit);

    let mut my_apple=Some(Fruit::Apple); //my_apple assigned with apple
    let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback); //my_apple is unchanged as it is already assigned and not none
    println!("should_be_apple is : {:?}", should_be_apple);
    println!("my_apple is unchanged: {:?}", my_apple);
}    