#![allow(unreachable_code, unused_labels)]

fn main()
{
    'outer: loop{
        println!("entered outer loop");
        'inner: loop{
            println!("entered inner loop");
            //break; it can break inner loop
            break 'outer;
        }
        println!("never reached in outer since we broke");
    }
    println!("exited the outer loop");
}