//String = Vec<v8> - heap allcoated
//&str is a slice - &[u8]

fn main(){
    //a ref to a string allocated in readonly mem
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("pangram: {}", pangram);

    println!("words in reverse"); //no need of new string
    for word in pangram.split_whitespace().rev(){
        println!("> {}", word);
    }

    //copy chars to vector, sort and remove duplicates    
    let mut chars: Vec<char>=pangram.chars().collect();
    chars.sort();
    chars.dedup();

    //create empty and growable string
    let mut string = String::new();
    for c in chars{
        string.push(c);
        string.push_str(", ");
    }

    let chars_to_trim: &[char]=&[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("used characters: {}", trimmed_str);

    //heap allocate a string
    let alice = String::from("I like dogs");
    //allocate new mem and store the modified string
    let bob: String = alice.replace("dog", "cat");
    println!("alice says: {}", alice);
    println!("bob says: {}", bob);
}    

