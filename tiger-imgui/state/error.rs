use std::fmt;

use crate::export::ExportError;
use crate::state::command::AsyncCommand;

#[derive(Debug)]
enum UserFacingErrorCategory {
    Open,
    Save,
    Export,
}

#[derive(Debug)]
pub struct UserFacingError {
    category: UserFacingErrorCategory,
    inner_error: anyhow::Error,
}

impl UserFacingError {
    pub fn from_command(
        source_command: AsyncCommand,
        inner_error: anyhow::Error,
    ) -> Option<UserFacingError> {
        let category = match source_command {
            AsyncCommand::ReadDocument(_) => UserFacingErrorCategory::Open,
            AsyncCommand::Save(_, _, _) => UserFacingErrorCategory::Save,
            AsyncCommand::SaveAs(_, _, _) => UserFacingErrorCategory::Save,
            AsyncCommand::Export(_) => UserFacingErrorCategory::Export,
            _ => return None,
        };
        Some(UserFacingError {
            category,
            inner_error,
        })
    }
}

impl fmt::Display for UserFacingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.category {
            UserFacingErrorCategory::Open => write!(
                f,
                "Could not open document:\n\n{}",
                self.inner_error.root_cause()
            ),
            UserFacingErrorCategory::Save => write!(f, "Could not save document"),
            UserFacingErrorCategory::Export => match self.inner_error.downcast_ref::<ExportError>()
            {
                Some(ExportError::TemplateParsingError(_)) => {
                    write!(
                        f,
                        "Export failed due to invalid syntax in the template file:\n\n{}",
                        self.inner_error.root_cause()
                    )
                }
                _ => write!(f, "Export failed:\n\n{}", self.inner_error.root_cause()),
            },
        }
    }
}
