use std::str::FromStr;
fn get_count_item(s:&str)->(u64,&str){
    let mut it = s.split(' ');
    /*
    let(Some(count_str),Some(item))=(it.next(),it.next())
    else{
        panic!("cant segment count item pair:'{s}'");
    };
    let Ok(count)=u64::from_str(count_str)else{
        panic!("cant parse integer:'{count_str}'");
    };
    (count,item)
    */
    //the above logic can be written using match let else
    let (count_str,item)=match (it.next(),it.next()){
        (Some(count_str),Some(item))=>(count_str,item),
        _=>panic!("cant segment counte item pair:'{s}'"),
    };
    let count=if let Ok(count)=u64::from_str(count_str){
        count
    }else{
        panic!("cant parse intege:'{count_str}'");
    };
    (count,item)
}
fn main()
{
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
}