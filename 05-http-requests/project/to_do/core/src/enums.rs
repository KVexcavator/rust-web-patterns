use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TaskStatus {
    DONE,
    PENDING,
}

impl TaskStatus {
    pub fn from_string(status: &String)
    -> Result<TaskStatus, String> {
        match status.to_uppercase().as_str() {
            "DONE" => Ok(TaskStatus::DONE),
            "PENDING" => Ok(TaskStatus::PENDING),
            _ => Err(format!("Invalid status: {}", status))
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &Self::DONE => {
                write!(f, "DONE")
            }
            &Self::PENDING => {
                write!(f, "PENDING")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_display() {
        println!("{}", TaskStatus::DONE);
        println!("{}", TaskStatus::PENDING);
        let outcome = TaskStatus::DONE.to_string();
        println!("{}", outcome);
    }
}
