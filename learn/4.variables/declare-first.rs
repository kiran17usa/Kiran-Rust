fn main()
{
    let a_binding ;
    {
        let x =2;
        a_binding=x*x;

    }
    println!("a binding:{}", a_binding);

    let mut another_binding=0;
    println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding:{}", another_binding);
}