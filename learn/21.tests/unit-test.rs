pub fn add(a:i32, b:i32)->i32{
    a+b
}
#[allow(dead_code)]
fn bad_add(a:i32, b:i32)->i32{
    a-b
}
#[cfg(test)]
mod tests{
    use super::*; //importing names from outer
    #[test]
    fn test_add(){
        assert_eq!(add(1,2),3); //pass
    }
    #[test]
    fn test_bad_add(){
        assert_eq!(bad_add(1,2),3) //fail
    }
}