fn main()
{
    for n in 1..=50{
        if n%15 ==0{
            println!("halwapalwa");
        }else if n%3 ==0{
            println!("halwa");
        }else if n%5==0{
            println!("palwa");
        }else{
            println!("{}",n);
        }
    }
}