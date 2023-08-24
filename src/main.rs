use clap::Parser;
use rand::Rng;

#[derive(Parser)]
struct Cli {
    #[arg(default_value_t = 6)]
    length: u8,
}

fn get_list() -> std::collections::HashMap<String, String> {
    let wordlist = include_str!("../assets/eff_large_wordlist.txt");
    let mut diceware_list = std::collections::HashMap::new();
    for line in wordlist.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        diceware_list.insert(parts[0].to_owned(), parts[1].to_owned());
    }
    diceware_list
}

fn get_phrase(diceware_list: &std::collections::HashMap<String, String>) -> String {
    let mut rng = rand::thread_rng();
    let mut key = String::new();
    for _ in 0..5 {
        key.push_str(&(rng.gen_range(1..=6)).to_string());
    }
    diceware_list.get(&key).unwrap().to_owned()
}

fn generate_password(length: u8) -> String {
    if length < 1 {
        panic!("Password must be at least one phrase");
    }
    let diceware_list = get_list();
    (0..length)
        .map(|_| get_phrase(&diceware_list))
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    let args = Cli::parse();
    println!("{}", generate_password(args.length));
}
