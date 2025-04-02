#[derive(Debug,Clone,Copy)]
struct Unit;

#[derive(Clone,Debug)]
struct Pair(Box<i32>,Box<i32>);
fn main(){
    let unit = Unit; //instantiate
    let copied_unit = unit; //copy
    //can be used seperately
    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);
    //instantiate pair
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);
    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);
    //println!("original: {:?}", pair); - pair already moved

    let cloned_pair= moved_pair.clone();
    drop(moved_pair);
    //println!("moved and dropped: {:?}", moved_pair);
    println!("clone: {:?}", cloned_pair);
}