struct S;
struct GenericVal<T>(T);

impl GenericVal<f32>{}
impl GenericVal<S>{}

impl<T> GenericVal<T>{}

struct Val{
    val:f64,
}
struct GenVal<T>{
    gen_val:T,
}
impl Val{
    fn value(&self)->&f64{
        &self.val
    }
}
impl<T>GenVal<T>{
    fn value(&self)->&T{
        &self.gen_val
    }
}
fn main()
{
    let x=Val{val:3.0};
    let y=GenVal{gen_val:3i32};
    println!("{},{}", x.value(),y.value());
}