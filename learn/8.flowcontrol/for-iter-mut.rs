fn main(){
    let mut names = vec!["kiran","swathi", "krithi", "karunya"];
    for name in names.iter_mut(){
        *name = match name{
            &mut "krithi"=>"there is a rust among us",
            _=>"hi hello",
        }
    }
    println!("names:{:?}", names);
}
