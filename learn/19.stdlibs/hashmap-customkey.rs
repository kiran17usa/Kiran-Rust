use std::collections::HashMap;
#[derive(PartialEq, Eq, Hash)]
struct Account<'a>{
    username: &'a str,
    password: &'a str,
}
struct AccountInfo<'a>{
    name: &'a str,
    email: &'a str,
}
type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;
fn try_logon<'a>(accounts: &Accounts<'a>,
    username: &'a str, password: &'a str){
        println!("username: {}", username);
        println!("password: {}", password);
        println!("attempting logon..");
    let logon = Account{
        username,
        password,
    };
    match accounts.get(&logon){
        Some(account_info)=>{
            println!("successful logon");
            println!("Name: {}", account_info.name);
            println!("email: {}", account_info.email);
        },
        _=>println!("logon failed: "),
    }
}
fn main(){
    let mut accounts: Accounts = HashMap::new();
    let account = Account{
        username: "j.everyman",
        password: "password123",
    };
    let account_info= AccountInfo{
        name: "John Everyman",
        email: "j.everyman@email.com",
    };
    accounts.insert(account, account_info);
    try_logon(&accounts, "j.everyman", "psasword123");
    try_logon(&accounts, "j.everyman", "password123");
}