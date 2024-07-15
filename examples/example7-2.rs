#[derive(Debug)]
struct Employee {
    name: String,
    years_of_service: u32,
}

fn main() {
    let marcus = Employee {
        name: "Marcus".to_string(),
        years_of_service: 2,
    };
    println!("{:?}", marcus);
}
