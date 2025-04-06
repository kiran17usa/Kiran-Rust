use std::thread;
const NTHREADS: u32=10;
fn main(){
    let mut children = vec![];
    for i in 0..NTHREADS{
        children.push(thread::spawn(move||{
            println!("this is thread number: {}", i);
        }));
    }
    for child in children{
        //wait for the thread to finish
        let _=child.join();
    }
}