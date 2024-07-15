fn main() {
    let mut list_1 = [6, 5, 7, 4, 8, 2, 9, 1, 3, 0];
    let mut list_2 = [6, 5, 7, 4, 8, 2, 9, 1, 3, 0];

    list_1.sort();
    list_1.reverse();

    list_2.sort_by(|a, b| a.cmp(b).reverse());

    assert_eq!(list_1, list_2);
}
