use std::fmt;
use thiserror::Error;

use crate::sheet::SheetError;
use crate::state::command::AsyncCommand;

#[derive(Error, Debug)]
pub enum StateError {
    #[error("No document is open")]
    NoDocumentOpen,
    #[error("Requested document was not found")]
    DocumentNotFound,
    #[error("Cannot perform undo operation")]
    UndoOperationNowAllowed,
    #[error("Sheet has no export settings")]
    NoExistingExportSettings,
    #[error("Requested frame is not in document")]
    FrameNotInDocument,
    #[error("Requested animation is not in document")]
    AnimationNotInDocument,
    #[error("Frame does not have a hitbox with the requested name")]
    InvalidHitboxName,
    #[error("Animation does not have a frame at the requested index")]
    InvalidKeyframeIndex,
    #[error("No keyframe found for requested time")]
    NoKeyframeForThisTime,
    #[error("Expected a hitbox to be selected")]
    NoHitboxSelected,
    #[error("Expected an keyframe to be selected")]
    NoKeyframeSelected,
    #[error("A hitbox with this name already exists")]
    HitboxAlreadyExists,
    #[error("An animation with this name already exists")]
    AnimationAlreadyExists,
    #[error("Not currently editing any frame")]
    NotEditingAnyFrame,
    #[error("Not currently editing any animation")]
    NotEditingAnyAnimation,
    #[error("Not currently adjusting export settings")]
    NotExporting,
    #[error("Not currently renaming an item")]
    NotRenaming,
    #[error("Not currently adjusting keyframe position")]
    NotAdjustingKeyframePosition,
    #[error("Not currently adjusting hitbox size")]
    NotAdjustingHitboxSize,
    #[error("Not currently adjusting hitbox position")]
    NotAdjustingHitboxPosition,
    #[error("Not currently adjusting keyframe duration")]
    NotAdjustingKeyframeDuration,
    #[error("Missing data while adjusting hitbox size")]
    MissingHitboxSizeData,
    #[error("Missing data while adjusting hitbox position")]
    MissingHitboxPositionData,
    #[error("Missing data while adjusting keyframe position")]
    MissingKeyframePositionData,
    #[error("Missing data while adjusting keyframe duration")]
    MissingKeyframeDurationData,
    #[error("Invalid sheet operation")]
    InvalidSheetOperation(#[from] SheetError),
}

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
