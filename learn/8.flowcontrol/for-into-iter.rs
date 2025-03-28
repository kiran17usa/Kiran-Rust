fn main()
{
    let names = vec!["kiran", "krithi", "karunya"];
    for name in names.into_iter(){
        match name{
        "karunya"=>println!("there is rust among us!"),
        _=>println!("Hi {}", name),
        }
    }
    //below line will not work as names moved into it consumed and not available for reuse hence cantuse below println for names
    //println!("names:{:?}", names);
}