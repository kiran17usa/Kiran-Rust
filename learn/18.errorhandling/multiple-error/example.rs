fn double_first(vec:Vec<&str>)->i32{
    let first = vec.first().unwrap();
    2*first.parse::<i32>().unwrap()
}
fn main(){
    let numbers = vec!["42", "93", "18"];
    //let empty:Vec<_> = vec![];
    let strings = vec!["tofu", "93", "18"];
    println!("the first doubled is {}", double_first(numbers));
    //println!("the first doubled is {}", double_first(empty));
    println!("the first doubled is {}", double_first(strings));

}