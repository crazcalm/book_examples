#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Employee {
    years_of_service: u32,
    name: String,
}

fn main() {
    let mut list_of_employees = vec![
        Employee {
            name: "Marcus".to_string(),
            years_of_service: 2,
        },
        Employee {
            name: "Jovanna".to_string(),
            years_of_service: 5,
        },
        Employee {
            name: "Carmen".to_string(),
            years_of_service: 2,
        },
        Employee {
            name: "Christy".to_string(),
            years_of_service: 2,
        },
        Employee {
            name: "Dillon".to_string(),
            years_of_service: 0,
        },
        Employee {
            name: "Jerry".to_string(),
            years_of_service: 1,
        },
    ];

    list_of_employees.sort();

    println!("{:#?}", list_of_employees);
}
