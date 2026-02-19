use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TaskStatus {
    DONE,
    PENDING,
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
