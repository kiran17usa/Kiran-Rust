use std::collections::HashMap;
fn call(number:&str)->&str{
    match number{
        "798-1364"=>"we are sorry, the call cannot be completed as dialed
            Please hangup and try again.",
        "645-7689"=>"hellow, this is mr.awesome pizza. my name is fred.
            What can i get for you today?",
        _=>"Hi! who is this again"
    }
}
fn main(){
    let mut contacts = HashMap::new();
    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "965-1745");
    // Takes a reference and returns Option<&V>
    match contacts.get(&"Daniel"){
        Some(&number)=>println!("calling Daniel: {}", call(number)),
        _=>println!("Dont have Daniels number."),
    }
    contacts.insert("Daniel", "164-6743");//daniel number got changed
    match contacts.get(&"Ashley"){
        Some(&number)=>println!("calling Ashley:{}", call(number)),
        _=>println!("Dont have Ashleys number."),
    }
    contacts.remove(&"Ashley");//ashley removed

    for(contact, &number) in contacts.iter(){
        println!("calling {}:{}", contact, call(number));
    }
}