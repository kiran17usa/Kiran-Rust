/*
You can unpack Options by using match statements, but it's often easier to use the ? operator. If x is an Option, then evaluating x? will return the underlying value if x is Some, otherwise it will terminate whatever function is being executed and return None
*/

fn next_birthday(current_age:Option<u8>)->Option<String>{
    let next_age: u8 = current_age? +1;  //current_age? tells 
    Some(format!("next year i will be {}", next_age))
}

struct Person{
    job:Option<Job>,
}
#[derive(Clone,Copy)]
struct Job{
    phone_number: Option<PhoneNumber>,
}
#[derive(Clone,Copy)]
struct PhoneNumber{
    area_code: Option<u8>,
    number: u32,
}
impl Person{
    fn work_phone_area_code(&self)->Option<u8>{
        self.job?.phone_number?.area_code
    }
}
fn main(){
    let p = Person{
        job:Some(Job{
            phone_number:Some(PhoneNumber{
                area_code:Some(61),
                number: 43987655,
            }),
        }),
    };
    assert_eq!(p.work_phone_area_code(), Some(61));
    println!("we are done!");
}
