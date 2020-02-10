use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Debug, Clone, Hash, Eq, Serialize, Deserialize)]
/// Represents a comment on a [Ticket](Ticket)
/// Wraps a string and checks that it is not empty when set
pub struct Comment {
    comment: String,
}

#[derive(PartialEq, Debug, Clone)]
/// Error if a comment cannot be created
pub struct CommentError {
    details: String,
}

/// Sets the error message for a comment if it cannot be created
impl CommentError {
    fn new(msg: &str) -> CommentError {
        CommentError {
            details: msg.to_string(),
        }
    }
}

/// Format CommentError for user display purposes
impl fmt::Display for CommentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for CommentError {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Creates a Comment for a [Ticket](Ticket)
/// Results in a [CommentError](CommentError) if the string passed in is empty
impl Comment {
    pub fn new(comment: String) -> Result<Comment, CommentError> {
        if comment.is_empty() {
            Err(CommentError::new("Comment cannot be empty"))
        } else {
            Ok(Comment { comment })
        }
    }
}

impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "Comment: {:?}", self.comment)
    }
}

#[cfg(test)]
mod comment_tests {
    use crate::models::Comment;

    #[test]
    fn creating_empty_comment_should_fail() {
        // arrange
        // act
        let new_comment = Comment::new("".to_string());
        // assert
        assert!(new_comment.is_err());
    }
}
