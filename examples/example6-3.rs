fn main() {
    let mut list = vec![
        ("Marcus".to_string(), 2),
        ("Jovanna".to_string(), 5),
        ("Carmen".to_string(), 2),
        ("Christy".to_string(), 2),
        ("Dillon".to_string(), 0),
        ("Jerry".to_string(), 1),
    ];

    list.sort_by(|a, b| a.1.cmp(&b.1).reverse());

    println!("{:?}", list);
}
