//printing is handled by series of macros defined in std::fmt

fn main()
{
    println!("{} days",31); //31 days
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // alice -0 , bob -1
    println!("{subject} {verb} {object}", 
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb="jumps over");
    println!("base 10: {}", 69420);
    println!("base 2: {:b}", 69420);
    println!("base 8: Octa  {:o}", 69420);
    println!("base 16: Hex {:x}", 69420);

    println!("{number:>5}", number = 1);
    println!("{number:0>5}", number =1);
    println!("{number:0<5}", number =1);
    println!("{number:0>width$}", number =1, width=7);
    println!("my name is {0}, {1} {0}", "Bond", "James");
    
    #[allow(dead_code)] //disable dead_code
    struct Structure(i32);
//fmt::Display whoever implements can display to the output
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

}