use crate::github::{GitHubPreview, GitHubPreviewError};
use std::fmt::Write;
use tracing::log::debug;

pub async fn get_preview(message: &str) -> Result<String, GitHubPreviewError> {
    let permalink = GitHubPreview::find_from_str(message)?;
    let code = permalink.get_code().await?;

    debug!("GitHubPreview::get_preview: {:?}", permalink);

    let mut is_backquote_replaced = false;
    is_backquote_replaced |= code.contains("```");
    let code = code.replace("```", "'''");

    let mut msg = String::new();
    let mut buf = String::new();

    // -# is a Markdown syntax for small text
    let _ = writeln!(buf, "-# {}/{}@{}: {}", permalink.owner, permalink.repo, permalink.branch, permalink.path);
    let _ = writeln!(buf, "```{}", permalink.ext);
    let _ = writeln!(buf, "{}", code);
    let _ = writeln!(buf, "```");


    msg.push_str(&buf);
    buf.clear();

    if is_backquote_replaced {
        msg.insert_str(0, "-# Backquotes were included, so they were replaced with single quotes\n");
    };

    Ok(msg)
}
