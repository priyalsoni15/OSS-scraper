// commit_metrics_graphql.rs
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde_json::json;
use std::error::Error;
use std::fs;
use std::fs::File;
use chrono::{DateTime, Utc, NaiveDate};
use crate::Args;
use log::{info, error};

// Types for GraphQL response
#[derive(Debug, serde::Deserialize)]
struct GraphQLResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, serde::Deserialize)]
struct GraphQLError {
    message: String,
}

#[derive(Debug, serde::Deserialize)]
struct RepositoryData {
    repository: Option<Repository>,
}

#[derive(Debug, serde::Deserialize)]
struct Repository {
    defaultBranchRef: Option<DefaultBranchRef>,
}

#[derive(Debug, serde::Deserialize)]
struct DefaultBranchRef {
    target: Option<CommitHistoryTarget>,
}

#[derive(Debug, serde::Deserialize)]
struct CommitHistoryTarget {
    history: History,
}

#[derive(Debug, serde::Deserialize)]
struct History {
    pageInfo: PageInfo,
    nodes: Vec<CommitNode>,
}

#[derive(Debug, serde::Deserialize)]
struct PageInfo {
    hasNextPage: bool,
    endCursor: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct CommitNode {
    oid: String,
    message: String,
    committedDate: String,
    author: Option<CommitAuthor>,
}

#[derive(Debug, serde::Deserialize)]
struct CommitAuthor {
    email: Option<String>,
    name: Option<String>,
}

// Types for REST API commit details
#[derive(Debug, serde::Deserialize)]
struct RestCommit {
    sha: String,
    commit: InnerCommit,
    files: Option<Vec<RestFile>>,
}

#[derive(Debug, serde::Deserialize)]
struct InnerCommit {
    message: String,
    author: RestAuthor,
}

#[derive(Debug, serde::Deserialize)]
struct RestAuthor {
    date: String,
}

#[derive(Debug, serde::Deserialize)]
struct RestFile {
    filename: String,
    status: String,
    additions: u32,
    deletions: u32,
}

// --- Added `commit_url` field below ---
#[derive(Debug, serde::Serialize)]
struct CsvRow {
    project: String,
    start_date: String,
    end_date: String,
    status: String,
    incubation_month: String,
    commit_sha: String,
    commit_url: String,                // <--- NEW FIELD
    email: String,
    name: String,
    date: String,
    timestamp: i64,
    filename: String,
    change_type: String,
    lines_added: u32,
    lines_deleted: u32,
    commit_message: String,
}

pub fn analyze_online_repo(
    online_url: &str,
    args: &Args,
    start_date: &str,
    end_date: &str,
    status: &str,
) -> Result<(), Box<dyn Error>> {
    // Extract owner and repo name from URL.
    let url = online_url.trim_end_matches(".git");
    let parts: Vec<&str> = url.split('/').collect();
    if parts.len() < 2 {
        return Err("Invalid GitHub URL provided.".into());
    }
    let owner = parts[parts.len()-2];
    let repo = parts[parts.len()-1];

    // Build HTTP client with headers (using GITHUB_TOKEN from .env)
    let github_token = std::env::var("GITHUB_TOKEN")
        .map_err(|_| "GITHUB_TOKEN environment variable is not set.")?;
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", github_token))?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(USER_AGENT, HeaderValue::from_static("rust-github-client"));
    let client = Client::builder().default_headers(headers).build()?;

    // GraphQL query to fetch commit history from the default branch.
    let query = r#"
    query($owner: String!, $name: String!, $cursor: String) {
      repository(owner: $owner, name: $name) {
        defaultBranchRef {
          target {
            ... on Commit {
              history(first: 100, after: $cursor) {
                pageInfo {
                  hasNextPage
                  endCursor
                }
                nodes {
                  oid
                  message
                  committedDate
                  author {
                    email
                    name
                  }
                }
              }
            }
          }
        }
      }
    }
    "#;

    let mut all_commits = Vec::new();
    let mut cursor: Option<String> = None;
    loop {
        let variables = json!({
            "owner": owner,
            "name": repo,
            "cursor": cursor,
        });
        let body = json!({
            "query": query,
            "variables": variables,
        });
        let response = client.post("https://api.github.com/graphql")
            .json(&body)
            .send()?;
        if !response.status().is_success() {
            let status_code = response.status();
            let text = response.text()?;
            error!("GraphQL API request failed with status {}: {}", status_code, text);
            return Err(format!("GraphQL API error: {}", status_code).into());
        }
        let resp_json: GraphQLResponse<RepositoryData> = response.json()?;
        if let Some(errors) = resp_json.errors {
            for err in errors {
                error!("GraphQL error: {}", err.message);
            }
            return Err("GraphQL query failed.".into());
        }
        let data = resp_json.data.ok_or("No data received from GitHub GraphQL API.")?;
        let repo_data = data.repository.ok_or("Repository not found or access denied.")?;
        let default_branch = repo_data.defaultBranchRef.ok_or("Default branch not found.")?;
        let commit_history_target = default_branch.target.ok_or("Default branch target not found.")?;
        let history = commit_history_target.history;

        all_commits.extend(history.nodes);
        if history.pageInfo.hasNextPage {
            cursor = history.pageInfo.endCursor;
        } else {
            break;
        }
    }
    info!("Fetched {} commits via GraphQL", all_commits.len());

    // Prepare CSV writer.
    let output_folder = args.flag_output_folder.as_deref().unwrap_or("data");
    fs::create_dir_all(output_folder)?;
    let csv_path = format!("{}/{}-commit-file-dev.csv", output_folder, repo);
    let file = File::create(&csv_path)?;
    let mut wtr = csv::WriterBuilder::new().has_headers(true).from_writer(file);

    // For each commit, fetch detailed commit info (REST API) to get file-level changes.
    for commit in all_commits {
        let commit_rest_url = format!(
            "https://api.github.com/repos/{}/{}/commits/{}",
            owner, repo, commit.oid
        );
        let rest_response = client.get(&commit_rest_url).send()?;
        if !rest_response.status().is_success() {
            error!("Failed to fetch commit details for {}: status {}", commit.oid, rest_response.status());
            continue;
        }
        let rest_commit: RestCommit = rest_response.json()?;
        let commit_date = DateTime::parse_from_rfc3339(&commit.committedDate)?
            .with_timezone(&Utc);
        let timestamp = commit_date.timestamp();
        let time_window = args.flag_time_window.unwrap_or(30);
        let start_naive = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")?;
        let commit_naive = commit_date.date_naive();
        let days_diff = (commit_naive - start_naive).num_days();
        let incubation_month = ((days_diff / time_window) + 1).to_string();

        if let Some(files) = rest_commit.files {
            for file_detail in files {
                let row = CsvRow {
                    project: repo.to_string(),
                    start_date: start_date.to_string(),
                    end_date: end_date.to_string(),
                    status: status.to_string(),
                    incubation_month: incubation_month.clone(),
                    commit_sha: commit.oid.clone(),
                    commit_url: format!("https://github.com/{}/{}/commit/{}", owner, repo, commit.oid), // <--- POPULATE URL
                    email: commit.author.as_ref().and_then(|a| a.email.clone()).unwrap_or_default(),
                    name: commit.author.as_ref().and_then(|a| a.name.clone()).unwrap_or_default(),
                    date: commit.committedDate.clone(),
                    timestamp,
                    filename: file_detail.filename,
                    change_type: file_detail.status,
                    lines_added: file_detail.additions,
                    lines_deleted: file_detail.deletions,
                    commit_message: commit.message.clone(),
                };
                wtr.serialize(row)?;
            }
        } else {
            // Write a row with empty file details if none are available.
            let row = CsvRow {
                project: repo.to_string(),
                start_date: start_date.to_string(),
                end_date: end_date.to_string(),
                status: status.to_string(),
                incubation_month: incubation_month.clone(),
                commit_sha: commit.oid.clone(),
                commit_url: format!("https://github.com/{}/{}/commit/{}", owner, repo, commit.oid), // <--- POPULATE URL
                email: commit.author.as_ref().and_then(|a| a.email.clone()).unwrap_or_default(),
                name: commit.author.as_ref().and_then(|a| a.name.clone()).unwrap_or_default(),
                date: commit.committedDate.clone(),
                timestamp,
                filename: "".to_string(),
                change_type: "".to_string(),
                lines_added: 0,
                lines_deleted: 0,
                commit_message: commit.message.clone(),
            };
            wtr.serialize(row)?;
        }
    }
    wtr.flush()?;
    info!("GraphQL-based commit analysis completed for repository {}", repo);
    Ok(())
}
