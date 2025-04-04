#[derive(Debug)]
enum Fruit {Apple, Orange, Banana, Kiwi, Lemon}

fn main()
{
    let mut my_fruit: Option<Fruit>=None;
    let apple = Fruit::Apple;
    let kiwi = Fruit::Kiwi;
    let first_available_fruit = my_fruit.get_or_insert(apple).get_or_insert(kiwi);
    println!("first_available_fruit is {:?}", first_available_fruit);
    println!("my_fruit is: {:?}", my_fruit);
}