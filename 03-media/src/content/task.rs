use super::employee::Employee;

#[derive(Debug)]
pub struct Task {
    pub assigned_to: Option<Employee>,
}
