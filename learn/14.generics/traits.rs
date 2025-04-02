struct Empty;
struct Null;
//a trait generic over T
trait DoubleDrop<T>{
    fn double_drop(self,_:T);
}
impl<T,U>DoubleDrop<T> for U{
    fn double_drop(self,_:T){}
}

fn main()
{
    let empty=Empty;
    let null = Null;
    empty.double_drop(null);

    //empty;
    //null;

}