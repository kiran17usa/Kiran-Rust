fn parse_csv_document<R:std::io::BufRead>(src:R)->std::io::Result<Vec<Vec<String>>>{
    src.lines()
        .map(|line|{
            line.map(|line|{
                line.split(',')
                    .map(|entry|String::from(entry.trim()))
                    .collect()
            })
        })
        .collect()
}
//the above can also be written
fn parse_csv_document1(src: impl std::io::BufRead)->std::io::Result<Vec<Vec<String>>>{
    src.lines()
        .map(|line|{
            line.map(|line|{
                line.split(',')
                    .map(|entry|String::from(entry.trim()))
                    .collect()
            })
        })
        .collect()    
}
fn main()
{

}