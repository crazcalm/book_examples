fn main() {
    let mut list = vec![
        ("a".to_string(), 1),
        ("ab".to_string(), 0),
        ("b".to_string(), 2),
        ("a".to_string(), 0),
        ("ba".to_string(), 0),
    ];
    list.sort();

    println!("{:?}", list);
}
