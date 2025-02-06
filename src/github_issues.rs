// src/github_issues.rs

use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use log::{info, error};

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub number: u32,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub author: Option<AuthorNode>,
    pub comments: CommentConnection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub body: String,
    pub created_at: String,
    pub author: Option<AuthorNode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub login: String,
    pub url: String,
}

// These types are used for the GraphQL query.
#[derive(Debug, Deserialize)]
struct GraphQLResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize)]
struct GraphQLError {
    message: String,
}

#[derive(Debug, Deserialize)]
struct RepositoryData {
    repository: Option<Repository>,
}

#[derive(Debug, Deserialize)]
struct Repository {
    #[serde(default)]
    issues: Option<IssueConnection>,
}

#[derive(Debug, Serialize, Deserialize)]
struct IssueConnection {
    pageInfo: PageInfo,
    nodes: Vec<IssueNode>,
}

#[derive(Debug, Serialize, Deserialize)]
struct IssueNode {
    id: String,
    number: u32,
    title: String,
    body: Option<String>,
    state: String,
    createdAt: String,
    updatedAt: String,
    closedAt: Option<String>,
    author: Option<AuthorNode>,
    comments: CommentConnection,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommentConnection {
    pageInfo: PageInfo,
    nodes: Vec<CommentNode>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommentNode {
    id: String,
    body: String,
    createdAt: String,
    author: Option<AuthorNode>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthorNode {
    login: String,
    #[serde(default)]
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PageInfo {
    hasNextPage: bool,
    endCursor: Option<String>,
}

// For additional comment queries.
#[derive(Debug, Deserialize)]
struct IssueData {
    issue: Option<CommentedIssue>,
}

#[derive(Debug, Deserialize)]
struct CommentedIssue {
    comments: CommentConnection,
}

// New CSV row type.
#[derive(Debug, Serialize)]
struct CsvRow {
    r#type: String,
    issue_url: String,
    comment_url: String,
    repo_name: String,
    id: String,
    issue_num: u32,
    title: String,
    user_login: String,
    user_id: String,
    user_name: String,
    user_email: String,
    issue_state: String,
    created_at: String,
    updated_at: String,
    body: String,
    reactions: String,
}

/// Fetch all issues (with comments) for a given repository via GitHub GraphQL API.
fn fetch_issues(owner: &str, repo: &str) -> Result<Vec<Issue>, Box<dyn Error>> {
    // Get the GitHub token from the environment variable.
    let github_token = env::var("GITHUB_TOKEN")
        .map_err(|_| "GITHUB_TOKEN environment variable is not set.")?;

    // Build the headers.
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", github_token))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("User-Agent", HeaderValue::from_static("rust-github-client"));

    // Create the HTTP client.
    let client = Client::builder().default_headers(headers).build()?;

    // Define the GraphQL query to fetch issues.
    let query = r#"
        query($owner: String!, $name: String!, $cursor: String) {
            repository(owner: $owner, name: $name) {
                issues(first: 100, after: $cursor, orderBy: {field: CREATED_AT, direction: ASC}) {
                    pageInfo {
                        hasNextPage
                        endCursor
                    }
                    nodes {
                        id
                        number
                        title
                        body
                        state
                        createdAt
                        updatedAt
                        closedAt
                        author {
                            login
                        }
                        comments(first: 100) {
                            pageInfo {
                                hasNextPage
                                endCursor
                            }
                            nodes {
                                id
                                body
                                createdAt
                                author {
                                    login
                                }
                            }
                        }
                    }
                }
            }
        }
    "#;

    let mut all_issues = Vec::new();
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
            let status = response.status();
            let text = response.text()?;
            error!("GitHub API request failed with status {}: {}", status, text);
            return Err(format!("GitHub API request failed with status {}", status).into());
        }

        let resp_json: GraphQLResponse<RepositoryData> = response.json()?;

        if let Some(errors) = resp_json.errors {
            for err in errors {
                error!("GraphQL error: {}", err.message);
            }
            return Err("GraphQL query failed.".into());
        }

        let data = resp_json.data.ok_or("No data received from GitHub API.")?;
        let repository = data.repository.ok_or("Repository not found or access denied.")?;

        let issues_conn = repository.issues.ok_or("Repository issues field is missing.")?;

        for issue_node in issues_conn.nodes {
            let issue = Issue {
                id: issue_node.id,
                number: issue_node.number,
                title: issue_node.title,
                body: issue_node.body,
                state: issue_node.state,
                created_at: issue_node.createdAt,
                updated_at: issue_node.updatedAt,
                closed_at: issue_node.closedAt,
                author: issue_node.author,
                comments: CommentConnection {
                    pageInfo: issue_node.comments.pageInfo,
                    nodes: issue_node.comments.nodes,
                },
            };
            all_issues.push(issue);
        }

        if issues_conn.pageInfo.hasNextPage {
            cursor = issues_conn.pageInfo.endCursor.clone();
        } else {
            break;
        }
    }

    Ok(all_issues)
}

/// Fetches issues (with comments) for the given owner/repo and writes them in CSV format.
pub fn fetch_issues_with_comments_csv(owner: &str, repo: &str, output_csv_path: &str) -> Result<(), Box<dyn Error>> {
    let issues = fetch_issues(owner, repo)?;
    let count = issues.len();

    let path = Path::new(output_csv_path);
    let file = File::create(path)?;
    let mut wtr = csv::Writer::from_writer(file);

    // Write CSV header.
    wtr.write_record(&[
        "type",
        "issue_url",
        "comment_url",
        "repo_name",
        "id",
        "issue_num",
        "title",
        "user_login",
        "user_id",
        "user_name",
        "user_email",
        "issue_state",
        "created_at",
        "updated_at",
        "body",
        "reactions",
    ])?;

    // For constructing URLs.
    let base_issue_url = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);
    let base_comment_url = format!("https://api.github.com/repos/{}/{}/issues/comments", owner, repo);
    let repo_name = repo.to_string();

    for issue in issues {
        // Issue row.
        let issue_row = CsvRow {
            r#type: "issue".to_string(),
            issue_url: format!("{}/{}", base_issue_url, issue.number),
            comment_url: "".to_string(),
            repo_name: repo_name.clone(),
            id: issue.id,
            issue_num: issue.number,
            title: issue.title,
            user_login: issue.author.map(|a| a.login).unwrap_or_default(),
            user_id: "".to_string(),
            user_name: "".to_string(),
            user_email: "".to_string(),
            issue_state: issue.state,
            created_at: issue.created_at,
            updated_at: issue.updated_at,
            body: issue.body.unwrap_or_default(),
            reactions: "".to_string(),
        };
        wtr.serialize(issue_row)?;

        // Comment rows.
        for comment_node in issue.comments.nodes {
            let comment_row = CsvRow {
                r#type: "comment".to_string(),
                issue_url: format!("{}/{}", base_issue_url, issue.number),
                comment_url: format!("{}/{}", base_comment_url, comment_node.id),
                repo_name: repo_name.clone(),
                id: comment_node.id,
                issue_num: issue.number,
                title: "NA".to_string(),
                user_login: comment_node.author.map(|a| a.login).unwrap_or_default(),
                user_id: "".to_string(),
                user_name: "".to_string(),
                user_email: "".to_string(),
                issue_state: "NA".to_string(),
                // Clone createdAt so it can be used twice.
                created_at: comment_node.createdAt.clone(),
                updated_at: comment_node.createdAt, // no separate updated time
                body: comment_node.body,
                reactions: "".to_string(),
            };
            wtr.serialize(comment_row)?;
        }
    }

    wtr.flush()?;
    info!("Fetched {} issues from {}/{}", count, owner, repo);
    Ok(())
}
