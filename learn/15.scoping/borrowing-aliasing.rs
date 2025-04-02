struct Point{x:i32, y:i32, z:i32}
fn main()
{
    let mut point = Point{x:0, y:0, z:0};
    let borrowed_point = &point;
    let another_borrow = &point;

    //data can access via ref's and original owner
    println!("point has coordinates: ({}, {}, {})",
            borrowed_point.x, borrowed_point.y,point.z);
    //cant take point as mutable bcoz its already borrowed as immutable
    //let mutable_borrow = &mut point;
    println!("point has cooridnates:({}, {}, {})",
            borrowed_point.x, another_borrow.y, point.z);
    //borrowed used above hence we can reborrow mut ref
    let mutable_borrow= &mut point;
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;
    //cant do becoz its borrowed above as mut
    //let y = &point.y;
    //cant do below bcoz println takes immutable ref
    //println!("point z coordinates is {}", point.z);

    println!("point has coordinates: ({}, {}, {})", 
            mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);
    let new_borrowed_point = &point;
    println!("point now has coordinates:({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);        

}