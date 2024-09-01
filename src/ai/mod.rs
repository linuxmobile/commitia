mod prompt;
mod types;

pub use prompt::{generate_commit_message, prepare_commit_context};
pub use types::CommitContext; // Re-export CommitContext
