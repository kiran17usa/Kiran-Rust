fn main(){
    let raw_str= r"Escapes dont work here: \x3F \u{211D}";
    println!("{} ", raw_str);
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    let longer_delimeter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimeter);

}