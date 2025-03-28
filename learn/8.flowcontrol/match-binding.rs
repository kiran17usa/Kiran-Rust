fn age()->u32{
    55
}
//you can match or bind to destructure - option
fn some_number()->Option<u32>{
    Some(45)
}
fn main()
{
    println!("tell me what type of person you are");
    match age()
    {
        0=>println!("no birthday yet since zero"),
        n@1..=12=>println!("i am a chile of age {:?}",n),
        n@13..19=>println!("iam a teen of age {:?}",n),
        n=>println!("i am an old person age:{:>}",n),
    }



    match some_number(){
        Some(n@42)=>println!("ans is : {}", n),
        Some(n)=>println!("not 42, ans is :{}",n),
        _=>(),
    }
}