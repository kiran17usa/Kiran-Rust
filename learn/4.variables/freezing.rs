fn main()
{
    let mut _mutable_integer = 7i32;
    {
        let mut _mutable_integer = _mutable_integer;
        println!("inner: {}", _mutable_integer);

        _mutable_integer = 50;
        println!("inner:post assignment {}", _mutable_integer);

    }
    _mutable_integer = 3;
    println!("outer: {}", _mutable_integer);
}