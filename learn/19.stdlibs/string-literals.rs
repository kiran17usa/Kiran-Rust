fn main(){
    let byte_escape = "I am writing \x52\x75\x73\x74!";
    println!("what are you doing \x3F (\\3F means ?) {}",byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R \" ";
    println!("unicode character {} (U+211D) is called {}", 
        unicode_codepoint, character_name);
    let long_string = "String literals can span multiple lines.
                      the linebreak and indentation here ->\
                      <- can be escaped too!";
    println!("{}", long_string);                         
}