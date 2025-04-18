//asyn channels for message passing service b/w threads
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 10;
fn main(){
    let (tx,rx): (Sender<i32>, Receiver<i32>)=mpsc::channel();
    let mut children =Vec::new();
    for id in 0..NTHREADS{
        let thread_tx =tx.clone();
        let child = thread::spawn(move||{
            thread_tx.send(id).unwrap();
            println!("thread {} finished", id);
        });
        children.push(child);
    }
    //collect msgs
    let mut ids =Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS{
        ids.push(rx.recv());
    }
    //wait for threads to complete
    for child in children{
        child.join().expect("oops! the child thread panicked");
    }
    println!("{:?}", ids);
}
