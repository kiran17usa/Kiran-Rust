#[derive(Debug)]
struct Person<'a>{
    name: &'a str,
    age: u8
}

fn main()
{
    let name = "Peter";
    let age = 27;
    let customer = Person{name, age};

    println!("{:#?}", customer);
}