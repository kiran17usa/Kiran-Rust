//the cfg attribute: #[cfg(...)] in attribute position
//the cfg! macro: cfg!(...) in boolean expressions

#[cfg(target_os="linux")]
fn are_you_on_linux()
{
    println!("you are running linux");
}

#[cfg(not(target_os="linux"))]
fn are_you_on_linux()
{
    println!("you are not running linux");
}

fn main()
{
    are_you_on_linux();
    println!("are you sure!");
    if cfg!(target_os="linux"){
        println!("yes. definately linux");
    }else{
        println!("yes. its definately not linux");
    }
}