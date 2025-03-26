fn main()
{
    let x = 5u32;
    let y ={
        let x_squared = x*x;
        let x_cube = x_squared *x;
        x_cube + x_squared +x // if you keep ; at the end of expression , it suppresses and adds just () to y
    };
    let z={
        //2*x; //bcoz of ; it just adds () to z
        2*x
    };
    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}