use std::fmt;

use crate::state::command::AsyncCommand;

#[derive(Debug)]
pub enum UserFacingError {
    Open(String),
    Save,
    Export(String),
}

impl UserFacingError {
    pub fn from_command(
        source_command: AsyncCommand,
        inner_error: &anyhow::Error,
    ) -> Option<UserFacingError> {
        match source_command {
            AsyncCommand::ReadDocument(_) => Some(UserFacingError::Open(format!(
                "{}",
                inner_error.root_cause()
            ))),
            AsyncCommand::Save(_, _, _) => Some(UserFacingError::Save),
            AsyncCommand::SaveAs(_, _, _) => Some(UserFacingError::Save),
            AsyncCommand::Export(_) => Some(UserFacingError::Export(format!(
                "{}",
                inner_error.root_cause()
            ))),
            _ => None,
        }
    }
}

impl fmt::Display for UserFacingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UserFacingError::Open(ref details) => {
                write!(f, "Could not open document:\n{}", details)
            }
            UserFacingError::Save => write!(f, "Could not save document"),
            UserFacingError::Export(ref details) => write!(f, "Export failed:\n{}", details),
        }
    }
}
