//closures are functions that captures enclosing environment
//     |var|var+x
fn main()
{
    let outer_var = 42;
    //fn function(i:i32)->i32{i+outer_var}
    let closure_annotated = |i:i32|->i32{i+outer_var};
    let closure_inferred = |i|i+outer_var;

    println!("closure_annotated:{}", closure_annotated(1));
    println!("closure_inferred:{}",closure_inferred(1));
    //above - once closure type has been inferred, cant change
    println!("cant reuse closure_inferred with anotehr type : {}", closure_inferred(42i32));

    let one = ||1;
    println!("closre returning one :{}", one());
    

}