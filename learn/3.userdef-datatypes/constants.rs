
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32)->bool{
    n>THRESHOLD
}

fn main()
{
    let n = 16;
    println!("this is {}", LANGUAGE);
    println!(" the thrshold is {}", THRESHOLD);
    println!("{} is {}", n,if is_big(n){ "big"} else {"small"});

    // THRESHOLD = 5; //cant do this since constant
    //LANGUAGE = "GO"
}