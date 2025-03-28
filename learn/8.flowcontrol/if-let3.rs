enum Foo{Bar}

fn main()
{
    let a =Foo::Bar;
    if let Foo::Bar =a{
        println!("a is foobar");
    }
}