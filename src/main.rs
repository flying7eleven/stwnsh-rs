use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The algorithm which should be used for hashing.
    #[clap(subcommand)]
    algorithm: Algorithms,
}

#[derive(Subcommand)]
enum Algorithms {
    /// Use the BCrypt algorithm for hashing.
    Bcrypt {
        /// The cost value for hashing the input text.
        #[arg(short, long, default_value_t = 12)]
        cost: u32,
        /// The text which should be hashed.
        #[arg(value_parser)]
        input_text: String,
    },
    /// Use the SHA256 algorithm for hashing.
    SHA256 {
        /// The text which should be hashed.
        #[arg(value_parser)]
        input_text: String,
    },
    /// Use the SHA512 algorithm for hashing.
    SHA512 {
        /// The text which should be hashed.
        #[arg(value_parser)]
        input_text: String,
    },
}

fn hash_bcrypt(cost: &u32, text: &String) -> String {
    return match bcrypt::hash(text, cost.clone()) {
        Ok(hashed_text) => hashed_text,
        Err(error) => format!("ERROR: {}", error),
    };
}

fn hash_sha256(text: &String) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(text))
}

fn hash_sha512(text: &String) -> String {
    use sha2::{Digest, Sha512};
    format!("{:x}", Sha512::digest(text))
}

fn main() {
    // parse the supplied arguments
    let arguments = Args::parse();

    // based on the supplied algorithm, calculate and output the hash
    match &arguments.algorithm {
        Algorithms::Bcrypt { cost, input_text } => println!("{}", hash_bcrypt(cost, input_text)),
        Algorithms::SHA256 { input_text } => println!("{}", hash_sha256(input_text)),
        Algorithms::SHA512 { input_text } => println!("{}", hash_sha512(input_text)),
    }
}
