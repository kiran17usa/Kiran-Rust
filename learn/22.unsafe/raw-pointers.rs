//unsafe operations
fn main(){
    let raw_p: *const u32= &10; //guarantee due to borrow checker
    //Dereferencing a raw pointer can only be done through an unsafe block.
    unsafe{
        assert!(*raw_p ==10); //no borrow checker so shld be under unsafe block
    }
}