let s:&'static str = "hello world";//ref static lifetime
fn generic<T>(x:T) where T:'static{} //static as trait bound