#[cfg(test)]
mod tests{
    use std::fs::OpenOptions;
    use std::io::Write;
    #[test]
    fn test_file()
    {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("ferris.txt")
            .expect("Failed to open ferris.txt");

        for _ in 0..5{
            file.write_all("Ferris\n".as_bytes())
            .expect("count not write to ferris.txt");
        }
    }
    #[test]
    fn test_file_also()
    {
        let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("ferris.txt")
        .expect("failed to open ferris.txt");

        for _ in 0..5{
            file.write_all("corro\n".as_bytes())
            .expect("count not write to ferris.txt");
        }
    }
}
//the above 2 tests writes to ferris.txt simultaniously - be careful with racing. this is because it uses concurrency