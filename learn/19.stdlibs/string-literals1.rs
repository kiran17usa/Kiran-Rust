use std::str;
fn main(){
    let bytestring: &[u8;21]=b"this is a byte string";
    println!("A byte string: {:?}", bytestring);

    let escaped = b"\x52\x75\x73\x74 as bytes";

    println!("some escaped bytes: {:?}", escaped);

    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    if let Ok(my_str)= str::from_utf8(raw_bytestring){
        println!("And the same as text: '{}'  ", my_str);
    }
    let _quotes = br#"you can also use "fancier" formatting,\
                  like with normal raw strings"#;
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb";
    match str::from_utf8(shift_jis){
        Ok(my_str)=>println!("conversion successful: '{}'",my_str),
        Err(e)=>println!("conversion failed: {:?}", e),
    }               
}