fn main()
{
    let mut count = 0u32;
    println!("lets count until infinity");

    loop{
        count +=1;
        if count == 3{
            println!("three");
            continue;
        }
        println!("{}", count);
        if count==5{
            println!("Ok, reached 5");
            break;
        }
    }
}