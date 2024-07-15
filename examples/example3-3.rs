use std::cmp::Ordering;

fn main() {
    let mut list: Vec<i32> = vec![3, 2, 5, 1, 6, 7, 8, 0, 9, 4];

    list.sort_by(|a, b| {
        let a_is_odd = a.abs() % 2 == 1;
        let b_is_odd = b.abs() % 2 == 1;

        if a_is_odd && b_is_odd {
            a.cmp(b)
        } else if a_is_odd {
            Ordering::Less
        } else if b_is_odd {
            Ordering::Greater
        } else {
            a.cmp(b)
        }
    });

    println!("{:?}", list);
}
