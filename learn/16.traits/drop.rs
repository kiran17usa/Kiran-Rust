//drop during object goes out of scope
struct Droppable{
    name: &'static str,
}
impl Drop for Droppable{
    fn drop(&mut self){
        println!("Dropping {}", self.name);
    }
}
fn main()
{
    let _a = Droppable{name: "a"};
    {
        let _b = Droppable{name: "b"};
        {
            let _c = Droppable{name: "c"};
            let _d = Droppable{name: "d"};
            println!("Exiting Block B");
        }
        println!("Just Exited Block B");
        println!("Exiting Block A");
    }
    println!("Just Exited Block A");

    drop(_a);//manual drop to drop before exiting main funciton
    println!("end of the main function");

}