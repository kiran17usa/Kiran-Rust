//extern crate rary; - for rust 2015 before

fn main()
{
    rary::public_function();
    rary::indirect_access();
    //rary::private_function(); - cant call
}

//rustc executable.rs --extern rary=library.rlib