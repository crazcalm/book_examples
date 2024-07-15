use std::cmp::Ordering;

#[derive(Debug)]
struct Employee {
    name: String,
    years_of_service: u32,
}

impl Ord for Employee {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.years_of_service
                .cmp(&other.years_of_service)
                .then(self.name.cmp(&other.name)),
        )
    }
}

impl Eq for Employee {}

impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        self.years_of_service == other.years_of_service && self.name == other.name
    }
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
