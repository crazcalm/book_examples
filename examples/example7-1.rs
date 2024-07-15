use std::fmt;

struct Employee {
    name: String,
    years_of_service: u32,
}

impl fmt::Debug for Employee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Employee")
            .field("name", &self.name)
            .field("years_of_service", &self.years_of_service)
            .finish()
    }
}

fn main() {
    let marcus = Employee {
        name: "Marcus".to_string(),
        years_of_service: 2,
    };
    println!("{:?}", marcus);
}
