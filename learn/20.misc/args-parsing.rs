use std::env;
fn increase(number: i32){
    println!("{}", number + 1);
}
fn decrease(number: i32){
    println!("{}", number -1);
}
fn help(){
    println!("usage:
    match args <string>
        Check whether given string is the answer.
    match args {{increase|decrease}}<integer>
        Increase or decrease given integer by one.");
}
fn main(){
    let args: Vec<String> = env::args().collect();
    match args.len(){
        1=>{
            println!("my name is 'match_args'. Try passing some arguments!");
        },
        2=>{
            match args[1].parse(){
                Ok(42)=>println!("this is the answer"),
                _=>println!("this is not the answer"),
            }
        },
        3=>{
            let cmd=&args[1];
            let num=&args[2];
            let number: i32 = match num.parse(){
                Ok(n)=>{
                    n
                },
                Err(_)=>{
                    eprintln!("error: second argument not an integer");
                    help();
                    return;
                },
            };
            match &cmd[..]{
                "increase"=>increase(number),
                "decrease"=>decrease(number),
                _=>{
                    eprintln!("error: invalid command");
                    help();
                },
            }
        },
        _=>{
            help();
        }
    }
}