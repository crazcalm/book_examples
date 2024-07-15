fn main() {
    let one = 1;
    let also_one = 1;
    let two = 2;
    let also_two = 2;

    let list_1 = vec![one, also_one, two, also_two];
    let list_2 = vec![also_one, one, also_two, two];

    println!("{:?} == {:?} = {}", list_1, list_2, list_1 == list_2);
}
