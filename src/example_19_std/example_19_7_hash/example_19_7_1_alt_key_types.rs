use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Attempting logon...");

    let logo = Account { username, password };

    match accounts.get(&logo) {
        Some(account_info) => {
            println!("Successful login!");
            println!("Name: {}", account_info.name);
            println!("Email: {}", account_info.email);
        }
        _ => println!("Login failed")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_19_7_1_alt_key_types() {
        let mut accounts: Accounts = HashMap::new();

        let account = Account {
            username: "j.everyman",
            password: "password123",
        };

        let account_info = AccountInfo {
            name: "John Everyman",
            email: "j.everyman@email.com",
        };

        accounts.insert(account, account_info);

        try_logon(&accounts, "j.everyman", "password1234");
        try_logon(&accounts, "j.everyman", "password123");
    }
}