#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Employee {
    years_of_service: u32,
    name: String,
}

fn main() {}

#[cfg(test)]
mod test {
    use super::Employee;

    #[test]
    fn test_sort() {
        let expected = vec![
            Employee {
                years_of_service: 0,
                name: "Dillon".to_string(),
            },
            Employee {
                years_of_service: 1,
                name: "Jerry".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Carmen".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Christy".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Marcus".to_string(),
            },
            Employee {
                years_of_service: 5,
                name: "Jovanna".to_string(),
            },
        ];

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

        assert_eq!(list_of_employees, expected);
    }
}
