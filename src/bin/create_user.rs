use hci_bildung::crypt;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let username = std::env::args().nth(1).ok_or("Please provide a username")?;
    let password = rpassword::prompt_password("New Password: ")?;
    let password_repeat = rpassword::prompt_password("Repeat Password: ")?;
    if password == password_repeat {
        let password_hash = crypt::hash_password(&password)?;
        print_sql(&username, &password_hash);
        Ok(())
    } else {
        Err("Passwords do not match".into())
    }
}

fn print_sql(username: &str, password_hash: &str) {
    println!("INSERT INTO users(username, password_hash) VALUES ('{}', '{}') RETURNING id", username, password_hash);
}