fn main()
{
    let triple = (3,-2,4);
    println!("tell me about {:?}", triple);
    match triple{
        (0,y,z)=>println!("first is zero, y is {:?}, z is {:?}",y,z),
        (1,..) =>println!("first is 1, rest doesnt matter"),
        (..,2)=>println!("last is 2, first doesnt matter"),
        (3,..,4)=>println!("first is 3, last is 4, middle doesnt matter"),
        _=>println!("anything from this tuple"),
    }
}