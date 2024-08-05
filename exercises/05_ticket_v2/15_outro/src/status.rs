// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Status::ToDo
            | Status::InProgress
            | Status::Done
            => "Invalid",
        };
        write!(f, "{message}")
    }
}

impl Error for Status {
}

impl TryFrom<String> for Status {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(())
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Status::try_from(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::*;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
