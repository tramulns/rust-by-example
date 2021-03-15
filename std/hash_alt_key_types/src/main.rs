use std::collections::HashMap;

// `Eq` требует, чтобы для типа был также выведен `PartialEq`.
#[derive(PartialEq, Eq, Hash)]
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
    println!("Имя пользователя: {}", username);
    println!("Пароль: {}", password);
    println!("Попытка входа...");

    let logon = Account { username, password };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Успешный вход!");
            println!("Имя: {}", account_info.name);
            println!("Email: {}", account_info.email);
        }
        _ => println!("Ошибка входа!"),
    }
}

fn main() {
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

    try_logon(&accounts, "j.everyman", "psasword123");

    try_logon(&accounts, "j.everyman", "password123");
}
