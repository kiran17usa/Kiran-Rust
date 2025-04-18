// + in arg list tells the arg repeat once or * for the zero or more times repeat
macro_rules! find_min{
    ($x:expr)=>($x);
    ($x:expr, $($y:expr), +)=>(
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main()
{
    println!("{}", find_min!(1));
    println!("{}", find_min!(1+2, 2));
    println!("{}", find_min!(5,2*3,4, 8, 3));
}
