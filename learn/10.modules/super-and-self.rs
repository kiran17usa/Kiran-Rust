fn function()
{
    println!("called function()");
}
mod cool{
    pub fn function(){
        println!("called cool::function()");
    }
}
mod my{
    fn function(){
        println!("called my::funciton()");
    }
    mod cool{
        pub fn function(){
            println!("called my::cool::function()");
        }
    }
    pub fn indirect_call(){
        print!("called my::indirect_call(), that \n");
        //self belongs to current mod, so normal funciton and self funciton refers same
        self::function();
        function();

        //refers to cool mod
        self::cool::function();
        //super refers to parent mod -upper
        super::function();
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}
fn main()
{
    my::indirect_call();
}