fn main() {
    let mut list = vec![3, 2, 5, 1, 6, 7, 8, 0, 9, 4];
    list.sort_by(|a, b| a.cmp(b));

    println!("{:?}", list);
}
