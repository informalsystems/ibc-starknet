use starknet_signers::SigningKey;

fn main() {
    let felt = SigningKey::from_random().secret_scalar();
    println!("{}", felt.to_hex_string());
}
