//Result is richer than Option
//Resutl<T,E> could have two outcomes 
//Ok(T) - element T found
//Err(E) - error was found

fn multiply(first_number_str: &str, second_number_str: &str)->i32{
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}
fn main()
{
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);
    let tt = multiply("t", "2");//unsuccessful unwrap and will panic the programm
    println!("double is {}", tt);
}