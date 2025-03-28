fn main()
{
    let number = 13;
    println!("tells about this number {}", number);
    for number in 1..=20{
    match number{
        1=>println!("one"),
        2|3|5|7|11=>println!("this is prime"),
        13..=19=>println!("teenager"),
        _=>println!("nothing special about this number"),
    }
   }

    let boolean = true;
    let binary = match boolean{
        false=>0,
        true=>1,
    };
    println!("{}->{}", boolean, binary);
}