fn main()
{
    let array=[-1,-2,6,9,11,15];
    match array{
        //[0,second,third]=>println!("array[0]=0, array[1]={}, array[2]={}", second, third),
        //single value ignored
        //[1,_,third]=>println!("array[0]=1, array[2]={} array[1] was ignored", third),
        //binding few and ignoring rest
        [-1,second, ..]=>println!("array[0]=-1, array[1]={} and all others ignored",second),
        //store in another slice of array
        //[3,second,tail@..]=>println!("array[0]=3,array[1]={} and other are {:?}",second, tail),
        [first,middle@..,last]=>println!("array[0]={}, array[last]={} and middle array={:?}",first, last,middle),


    }
}