use super::employee::Employee; // 'super' is the reference to the parent module

#[derive(Debug)]
pub struct Task {
    pub assigned_to: Option<Employee>,
}
