//let optional = Some(10);
/*
match optional{
    Some(i)=>println!("looks like long string and {:?} ", i),
_=>{},
};
*/
fn main()
{
    let number = Some(10);
    let letter:Option<i32> = None;
    let emoticon:Option<i32> = None;

    if let Some(i)=number{
        println!("matched {:?}",i);
    }else{
        println!("not a match");
    }
    let i_like_letters = false;
    if let Some(i)=emoticon{
        println!("matched{:?}",i);
    }else if i_like_letters{
        println!("didnt match a number. lets go with letter");
    }else{
        println!("not liking letters.lets go with emoticon");
    }
}