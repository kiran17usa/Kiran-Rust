use std::mem;

fn analyze_slice(slice: &[i32]){
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main()
{
    let xs: [i32; 5] = [1,2,3,4,5];
    let ys: [i32;500] = [0;500];//all initialize to 0

    println!("first element of the array:{}", xs[0]);
    println!("second element of the array:{}", xs[1]);

    println!("Number of elements in array:{}", xs.len());

    println!("array occupies {} bytes", mem::size_of_val(&xs) );

    println!("Borrow the whole array as slice.");
    analyze_slice(&xs);

    //borrow a section
    println!("Borrow the section of array as a slice");
    analyze_slice(&ys[1 .. 4]);

    let empty_array:[u32;0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); //same as above but more verbose

    for i in 0..xs.len() +1{
        match xs.get(i){ //matches the element and runs for some only
            Some(xval)=>println!("{}:{}",i,xval), //it can run for 0-4 times only
            None=>println!("Slow down! {} is too far!", i),//will run when finds upper bound i.e., when no element : here is 5
        }
    }

}