mod my_mod{
    fn private_function(){
        println!("called 'my_mod::private_function");
    }

    pub fn function(){
        println!("called mymod::function()");
    }
    //can access other functions in the sam module
    pub fn indirect_access(){
        print!("called mymod::indirect_access()");
        private_function();
    }

    pub mod nested{
        pub fn function(){
            println!("called my_mod::nested::function()");
        }

        #[allow(allow_code)]
        fn private_function(){
            println!("called my_mod::nested::private_function()");
        
        }
        // Functions declared using `pub(in path)` syntax are only visible
        // within the given path. `path` must be a parent or ancestor module
        pub(in crate::my_mod) fn public_function_in_my_mod(){
            print!("called my_mod::nested::public_funciton_in_my_mod()\n");
            public_function_in_nested();
        }
        pub(self) fn public_function_in_nested(){
            println!("called my_mod::nested::publci_function_in_nested()");    
        }
        // Functions declared using `pub(super)` syntax are only visible within
        // the parent module
        pub(super) fn public_function_in_super_mod(){
            println!("called my_mod::nested::public_function_in_super_mod()")
        }
    }
    pub fn call_public_function_in_my_mod(){
        //use my_mod::nested;
        print!("called my_mod::call_public_function_in_my_mod()");
        nested::public_function_in_my_mod();
        print!(">  ");
        nested::public_function_in_super_mod();
    }
    
    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate(){
        println!("called my_mod::public_function_in crate()");
    }
    //nested modules follow the same visibility
mod private_nested{
    #[allow(dead_code)]
    pub fn function(){
        println!("called my_mod::private_nested::function()");
    }

    #[allow(dead_code)]
    pub(crate) fn restricted_function(){
        println!("called my_mod::private_nested::restricted_function()");
    }
}
}

fn function()
{
    println!("called function()");
}

fn main(){
    function();
    my_mod::function();

    my_mod::indirect_access();
    my_mod::nested::function();

    my_mod::call_public_function_in_my_mod();
    my_mod::public_function_in_crate();

    // pub(in path) items can only be called from within the module specified
    // Error! function `public_function_in_my_mod` is private
    //my_mod::nested::public_function_in_my_mod();
    // TODO ^ Try uncommenting this line

    // Private items of a module cannot be directly accessed, even if
    // nested in a public module:

    // Error! `private_function` is private
    //my_mod::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_function` is private
    //my_mod::nested::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::restricted_function();
    // TODO ^ Try uncommenting this line

}