//vectors like arrays
//pointer to data , capacity, length

//can grow until the capacity

fn main()
{
    //immutable vector
    let collected_iterator: Vec<i32>=(0..=10).collect();
    println!("collected (0..10) into: {:?}", collected_iterator);

    //mutable vector
    let mut xs= vec![1i32,2,3];
    println!("initial vector: {:?}", xs);

    println!("push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);
    //immutable vector cant grow, below throws error
    //collected_iterator.push(0);

    println!("vector length: {}", xs.len());
    println!("second element: {}", xs[1]);
    println!("pop last element: {:?}", xs.pop());
    //println!("fourth element: {}", xs[3]);

    println!("contents of xs: ");
    for x in xs.iter(){
        println!("> {}",x);
    }
    //index can enumerate
    for (i,x) in xs.iter().enumerate(){
        println!("In position {} we have value {}", i, x);
    }

    for x in xs.iter_mut(){
        *x +=3;
    }
    println!("updated vector: {:?}", xs);



}