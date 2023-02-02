/// Catch-all error for front-end facing commands
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error(transparent)]
    RusqliteError(#[from] rusqlite::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type CommandResult<T, E = CommandError> = anyhow::Result<T, E>;

// Tauri requires all command return a String Serializable value
// anyhow::Error doesn't setup String Serialization so we must do it manually
impl serde::Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
