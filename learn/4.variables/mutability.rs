fn main()
{
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("before mutation: {}", mutable_binding);

    mutable_binding += 1;
    println!("after mutation : {}", mutable_binding);

    //_immutable_binding += 1;


}