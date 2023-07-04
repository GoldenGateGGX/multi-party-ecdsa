use multi_party_ecdsa::multichain::create_node;

fn main() {
    let result = create_node(1, 1, 1024, "EC256STARK");
    println!("{}", result);
}
