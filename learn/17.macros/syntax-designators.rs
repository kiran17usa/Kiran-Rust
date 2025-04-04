macro_rules! create_function{
    //
    ($func_name: ident)=>{
        fn $func_name(){
            println!("you called {:?}", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result{
    ($expression:expr)=>{
        println!("{:?}= {:?}",
                stringify!($expression),
                $expression)
    };
}
fn main(){
    foo();
    bar();
    print_result!(1u32 +1);
    print_result!({
        let x = 1u32;
        x*x+2*x-1
    });
}
/*other designators
block
expr
ident
item
literal
pat
path
stmt
tt - token tree
ty - type
vis - vis qualifier

*/