fn main()
{
    let reference = &4;
    match reference{
        //patten matched against ref
        &val=>println!("got a value via destructuring: {:?}", val)
    }
    //if not above , you need to dereference before match
    match *reference{
        val=>println!("got a value via dereferencing:{:?} ", val),

    }
    let _not_a_reference = 3;//not ref
    let ref _is_a_reference = 3;//will become ref

    //with out ref you can retrieve as a ref
    let value = 5;
    let mut mut_value = 5;


    match value{
        ref r=>println!("got a ref to a value:{:?}", r),
    }
    match mut_value{
        ref mut m=>{
            *m +=10;
            println!("we added 10. 'mut_value:{:?}", m);
        }
    }
}