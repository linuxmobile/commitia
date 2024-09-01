use crate::ai::types::{CommitContext, FileDiff};
use anyhow::{Context, Result};
use async_std::sync::{Arc, Mutex};
use google_generative_ai_rs::v1::{
  api::Client,
  gemini::{request::Request, response::GeminiResponse, Content, Part, ResponseType, Role},
};
use serde_json::json;

pub fn prepare_commit_context(
  repo_name: &str,
  selected_files: &[(String, String)],
) -> Result<CommitContext> {
  let files = selected_files
    .iter()
    .map(|(path, diff)| FileDiff {
      path: path.to_string(),
      diff: diff.to_string(),
    })
    .collect();

  Ok(CommitContext {
    repo_name: repo_name.to_string(),
    files,
  })
}

pub async fn generate_commit_message(context: CommitContext) -> Result<String> {
  let api_key = crate::config::load_token().context("Failed to load API token")?;
  let client = Client::new_from_model_response_type(
    google_generative_ai_rs::v1::gemini::Model::Gemini1_5Pro,
    api_key,
    ResponseType::StreamGenerateContent,
  );

  let files_json = context
    .files
    .iter()
    .map(|file| {
      json!({
          "path": file.path,
          "diff": file.diff
      })
    })
    .collect::<Vec<_>>();

  let prompt = format!(
    r#"Generate a commit message for the following changes in the '{}' repository:

Repository: {}

Changes:
{}

Please format the commit message using the Conventional Commits specification:
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]

Consider the following guidelines:
1. The commit type should be one of: feat, fix, docs, style, refactor, perf, test, or chore.
2. The description should be concise and describe what was changed, not how it was changed.
3. If there are multiple changes, focus on the most significant one for the type and description.
4. Use the body to explain the what and why of the change, not the how.
5. If there are breaking changes, mention them in the footer with a "BREAKING CHANGE:" prefix.

Respond only with the generated commit message, without any additional explanation or formatting."#,
    context.repo_name,
    context.repo_name,
    serde_json::to_string_pretty(&files_json).context("Failed to serialize file changes")?
  );

  let request = Request {
    contents: vec![Content {
      role: Role::User,
      parts: vec![Part {
        text: Some(prompt),
        inline_data: None,
        file_data: None,
        video_metadata: None,
      }],
    }],
    tools: vec![],
    safety_settings: vec![],
    generation_config: None,
    system_instruction: None,
  };

  let response = client
    .post(30, &request)
    .await
    .context("Failed to get response from AI")?;

  let commit_message = Arc::new(Mutex::new(String::new()));

  if let Some(stream_response) = response.streamed() {
    if let Some(json_stream) = stream_response.response_stream {
      let commit_message_clone = Arc::clone(&commit_message);
      Client::for_each_async(json_stream, move |response: GeminiResponse| {
        let commit_message = Arc::clone(&commit_message_clone);
        async move {
          if let Some(text) = response.candidates[0].content.parts[0].text.as_ref() {
            let mut lock = commit_message.lock().await;
            lock.push_str(text);
          }
        }
      })
      .await;
    }
  }

  let final_message = Arc::try_unwrap(commit_message).unwrap().into_inner();

  Ok(final_message)
}
