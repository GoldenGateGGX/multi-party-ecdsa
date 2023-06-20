use multi_party_ecdsa::multichain::greet;

fn main() {
    let result = greet("Rust");
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = greet("Rust");
        assert_eq!(result, "Hello from Go, Rust!");
    }
}
