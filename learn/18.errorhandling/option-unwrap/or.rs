#[derive(Debug)]
enum Fruit{Apple, Orange, Banana, Kiwi, Lemon}

fn main()
{
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;

    let first_available_fruit = no_fruit.or(apple).or(orange);
    println!("first_available_fruit: {:?}", first_available_fruit);

    //first_available_fruit: Some(Orange)
}