// Include the generated protobuf code
pub mod employees {
    include!(concat!(env!("OUT_DIR"), "/employees.rs"));
}

fn main() {
    use employees::{Employee, Employees};

    // Create some employees
    let employee_list = Employees {
        employees: vec![
            Employee {
                id: 1001,
                name: "Alice".to_string(),
                salary: 1000.0,
            },
            Employee {
                id: 1002,
                name: "Emil".to_string(),
                salary: 2000.0,
            },
        ],
    };

    println!("Created {} employees", employee_list.employees.len());
    for emp in &employee_list.employees {
        println!("Employee: {} (ID: {}), Salary: ${}", emp.name, emp.id, emp.salary);
    }
}

