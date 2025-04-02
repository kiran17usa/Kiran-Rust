fn main()
{
    #[derive(Debug)]
    struct Person{
        name: String,
        age: Box<u8>,
    }
    //impl Drop for Person{
    //    fn drop(&mut self){
    //        println!("Dropping the person struct {:?}", self);
    //    }
    //}
    let person=Person{
        name: String::from("Alice"),
        age:Box::new(20),
    };
    //the below name is moved out of person but not age due to ref used
    let Person{name, ref age}=person;
    println!("the person's age is {}", age);
    println!("the person's name is {}", name);
    //println!("the person struct is {:?}", person);
    println!("the person's age from person struct is {}", person.age)
}