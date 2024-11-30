// 'super' is the reference to the parent module
use super::employee::Employee;

#[derive(Debug)]
pub struct Task {
    pub assigned_to: Option<Employee>,
}
