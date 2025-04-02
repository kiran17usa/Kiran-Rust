struct ToDrop;
impl Drop for ToDrop{
    fn drop(&mut self){
        println!("ToDrop is being dropped");
    }
}
fn main()
{
    let x = ToDrop;
    println!("made a ToDrop!");
}