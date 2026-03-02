use serde::{Deserialize, Serialize};
use std::fmt;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TaskStatus {
    DONE,
    PENDING,
}

impl TaskStatus {
    pub fn from_string(status: &String) -> Result<TaskStatus, NanoServiceError> {
        match status.to_uppercase().as_str() {
            "DONE" => Ok(TaskStatus::DONE),
            "PENDING" => Ok(TaskStatus::PENDING),
            _ => Err(
                NanoServiceError::new(
                    "Invalid status".to_string(),
                    NanoServiceErrorStatus::BadRequest
                )
            ),
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
