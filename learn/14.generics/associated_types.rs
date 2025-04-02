// `A` and `B` are defined in the trait via the `type` keyword.
// (Note: `type` in this context is different from `type` when used for
// aliases).
/*
trait Contains{
    type A;
    type B;
    fn contains(&self:_:&self::A, _:&self::B)->bool;
}

//without using associated types
fn difference<A,B,C>(container:&C)->i32 where
    C:Contains<A,B>{...}
//using associated
fn difference<C:Contains>(container:&C)->i32{...}
*/
//rewrite previous

struct Container(i32,i32);
trait Contains{
    type A;
    type B;
    fn contains(&self, _:&Self::A, _:&Self::B)->bool;
    fn first(&self)->i32;
    fn last(&self)->i32;
}
impl Contains for Container{
    type A = i32;
    type B = i32;
    fn contains(&self, number_1:&i32, number_2:&i32)->bool{
        (&self.0==number_1)&&(&self.1==number_2)
    }
    fn first(&self)->i32{self.0}
    fn last(&self)->i32{self.1}
}
fn difference<C:Contains>(container:&C)->i32{
    container.last()-container.first()
}
fn main()
{
    let number_1=3;
    let number_2=10;
    let container = Container(number_1,number_2);
    println!("Does container contain {} and {}: {}", &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("first number: {}", container.first());
    println!("last number: {}", container.last());
    println!("the diff is :{}", difference(&container));    
}