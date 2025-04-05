use std::rc::Rc;
fn main(){
    let rc_examples = "Rc examples".to_string();
    {
        println!("----rc_a - is created");
        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("reference count of rc_a: {}", Rc::strong_count(&rc_a));
        {
            println!("-----rc_a is cloned to rc_b----");
            let rc_b: Rc<String>=Rc::clone(&rc_a);
            println!("ref count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("ref count of rc_a: {}", Rc::strong_count(&rc_a));

            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));
            println!("length of the value inside rc_a: {}", rc_a.len());
            println!("value of rc_b: {}", rc_b);
            println!("-----rc_b is dropped out of scope ---");
        }
        println!("ref count of rc_a: {}", Rc::strong_count(&rc_a));
        println!("-----rc_a is dropped out of scope ---- ");
        
    }
}