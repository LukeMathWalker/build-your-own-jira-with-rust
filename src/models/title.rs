use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug, Clone, Hash, Eq, Serialize, Deserialize)]
/// The title of a [Ticket](Ticket)
/// Wraps a string and checks that it's not empty when set
pub struct Title {
    title: String,
}

#[derive(PartialEq, Debug, Clone)]
/// Error if a title cannot be created
pub struct TitleError {
    details: String,
}

/// Sets the error message for a title if it cannot be created
impl TitleError {
    fn new(msg: &str) -> TitleError {
        TitleError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for TitleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for TitleError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl Title {
    /// Creates a Title for a [Ticket](Ticket)
    /// results in a [TitleError](TitleError) if the string passed in is empty
    pub fn new(title: String) -> Result<Title, TitleError> {
        if title.is_empty() {
            Err(TitleError::new("Title Cannot be empty"))
        } else {
            Ok(Title { title })
        }
    }
}

#[cfg(test)]
mod title_tests {
    use crate::models::Title;

    #[test]
    fn creating_an_empty_title_should_fail() {
        //arrange
        //act
        let new_title = Title::new("".to_string());

        assert!(new_title.is_err())
    }
}
