use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::StatusCode;
use tracing::debug;
use url::Url;

static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"https?://(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&/=]*)").unwrap()
});
static LINE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"L(?P<l1>\d+)(?:-L(?P<l2>\d+))?").unwrap());

const PREVIEW_DEFAULT_SHOWN_LINES: usize = 12;
const PREVIEW_DL_SIZE_LIMIT: u64 = 1024 * 1024;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GitHubPreview {
    pub owner: String,
    pub repo: String,
    pub branch: String,
    pub path: String,
    pub ext: String,
    pub line1: usize,
    pub line2: Option<usize>,
}

#[derive(thiserror::Error, Debug)]
pub enum GitHubPreviewError {
    #[error("GitHub Raw Content Fetch Error")]
    Fetch { status_code: StatusCode },
    #[error("GitHub Raw Content Too Big Size")]
    TooBigContentSize,
    #[error("GitHub Raw Content Internal Error")]
    InternalError(anyhow::Error),
}

impl From<anyhow::Error> for GitHubPreviewError {
    fn from(value: anyhow::Error) -> Self {
        GitHubPreviewError::InternalError(value)
    }
}

impl GitHubPreview {
    pub fn is_exist(content: &str) -> bool {
        static SKIP_URL_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"<https?://(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&/=]*)>").unwrap()
        });

        URL_REGEX.is_match(content)
            && !SKIP_URL_REGEX.is_match(content)
            && LINE_REGEX.is_match(content)
    }

    pub fn find_from_str(content: &str) -> anyhow::Result<Self> {
        let mut res = URL_REGEX
            .find_iter(content)
            .flat_map(|m| {
                let permalink = Url::parse(m.as_str()).ok()?;

                if permalink.host_str()? != "github.com" {
                    return None;
                }

                let mut segments = permalink.path_segments()?;
                let owner = segments.next()?; // m1sk9/
                let repo = segments.next()?; // m1sk9/fromis

                if segments.next()? != "blob" {
                    // m1sk9/fromis/blob
                    return None;
                }

                let branch = segments.next()?; // m1sk9/fromis/blob/master
                let path = segments.collect::<Vec<_>>().join("/"); // m1sk9/fromis/blob/master/src/github.rs
                let filename = permalink.path_segments()?.next_back()?; // github.rs
                let ext = std::path::Path::new(filename)
                    .extension()
                    .map(|x| x.to_str().unwrap())
                    .unwrap_or(""); // .rs

                /* ファイル行をキャプチャする */
                let fragment = permalink.fragment()?;
                let captures = LINE_REGEX.captures(fragment)?;

                let line1 = captures.name("l1").unwrap().as_str().parse().ok()?;
                let line2 = match captures.name("l2") {
                    Some(l2) => Some(l2.as_str().parse().ok()?),
                    None => None,
                };

                Some(Self {
                    owner: owner.to_string(),
                    repo: repo.to_string(),
                    branch: branch.to_string(),
                    path,
                    ext: ext.to_string(),
                    line1,
                    line2,
                })
            })
            .collect::<Vec<_>>();

        debug!("GitHubPreview::find_from_str: {:?}", res);
        res.pop()
            .ok_or_else(|| anyhow::anyhow!("No GitHub URL found"))
    }

    pub async fn get_code(&self) -> anyhow::Result<String, GitHubPreviewError> {
        let raw_url = format!(
            "https://raw.githubusercontent.com/{owner}/{repo}/{branch}/{path}",
            owner = self.owner,
            repo = self.repo,
            branch = self.branch,
            path = self.path,
        );

        // TODO: Add cache for raw content

        let res = reqwest::get(&raw_url)
            .await
            .context("Failed to fetch raw content")?;
        if !res.status().is_success() {
            return Err(GitHubPreviewError::Fetch {
                status_code: res.status(),
            });
        };

        if !matches!(res.content_length(), Some(len) if len < PREVIEW_DL_SIZE_LIMIT) {
            return Err(GitHubPreviewError::TooBigContentSize);
        };

        const OFFSET: usize = PREVIEW_DEFAULT_SHOWN_LINES / 2;
        let content = res.text().await.context("Failed to read raw content")?;
        let (line1, line2) = match self.line2 {
            Some(l2) => (self.line1, l2),
            None => (
                self.line1.saturating_add(OFFSET),
                self.line1.saturating_sub(OFFSET),
            ),
        };

        let skip = line1.saturating_sub(1);

        Ok(content
            .lines()
            .skip(skip)
            .take(line2.saturating_sub(skip))
            .collect::<Vec<&str>>()
            .join("\n"))
    }
}
