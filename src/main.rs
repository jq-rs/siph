use siphasher::sip::SipHasher;
use std::hash::{Hash, Hasher};
use std::{env, process};

const USAGE: &str = "usage: siph <'string'>/<'[\"string1\", \"string2\", ..]'>";   

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{}", USAGE);
        process::exit(-1);
    }

    let input = &args[1];
    let hash = if input.starts_with('[') {
        let json_input: Vec<String> = match serde_json::from_str(input) {
            Ok(input) => input,
            Err(err) => {
                println!("Failed to parse JSON input: {}", err);
                println!("{}", USAGE);
                process::exit(-2);
            }
        };
        hash_over_strings(json_input)
    } else {
        if args.len() > 2 {
            println!("{}", USAGE);
            process::exit(-1);
        }
        hash_over_strings(vec![input.to_owned()])
    };
    println!("{}", hash);
    process::exit(0);
}

fn hash_over_strings(strings: Vec<String>) -> u64 {
    let mut hasher = SipHasher::new();
    for string in strings {
        string.hash(&mut hasher);
    }
    hasher.finish()
}
