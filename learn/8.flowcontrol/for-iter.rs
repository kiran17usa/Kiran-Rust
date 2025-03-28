fn main()
{
    let names = vec!["kiran", "krithi", "karunya"];

    for name in names.iter(){
        match name{
            &"karunya"=>println!("There is a rustacean among us!"),
            _=>println!("Hello {}", name),
        }
    }
    println!("names:{:?}",names)
}