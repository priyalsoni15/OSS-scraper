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
    /// Fallback global id (base64–encoded) in case databaseId is missing.
    pub id: String,
    /// Numeric issue id (as provided by GitHub’s REST API) if available.
    pub databaseId: Option<u64>,
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
    /// Fallback global id.
    pub id: String,
    /// Numeric comment id if available.
    pub databaseId: Option<u64>,
    pub body: String,
    pub created_at: String,
    pub author: Option<AuthorNode>,
}

// GraphQL response types.
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
    databaseId: Option<u64>,
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
    databaseId: Option<u64>,
    body: String,
    createdAt: String,
    author: Option<AuthorNode>,
}

/// The author information. Note that we use an inline fragment on User (in the query) so that GitHub
/// returns the numeric databaseId as well as name and email (if public).
#[derive(Debug, Serialize, Deserialize)]
struct AuthorNode {
    login: String,
    #[serde(default)]
    databaseId: Option<u64>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PageInfo {
    hasNextPage: bool,
    endCursor: Option<String>,
}

// (Unused additional types for comment queries can remain here.)
#[derive(Debug, Deserialize)]
struct IssueData {
    issue: Option<CommentedIssue>,
}

#[derive(Debug, Deserialize)]
struct CommentedIssue {
    comments: CommentConnection,
}

// CSV row type.
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

    // Build headers.
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", github_token))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("User-Agent", HeaderValue::from_static("rust-github-client"));

    let client = Client::builder().default_headers(headers).build()?;

    // GraphQL query with inline fragments requesting numeric ids (databaseId),
    // name, and email for authors.
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
                        databaseId
                        number
                        title
                        body
                        state
                        createdAt
                        updatedAt
                        closedAt
                        author {
                            login
                            ... on User {
                                databaseId
                                name
                                email
                            }
                        }
                        comments(first: 100) {
                            pageInfo {
                                hasNextPage
                                endCursor
                            }
                            nodes {
                                id
                                databaseId
                                body
                                createdAt
                                author {
                                    login
                                    ... on User {
                                        databaseId
                                        name
                                        email
                                    }
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
                databaseId: issue_node.databaseId,
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
    let mut wtr = csv::WriterBuilder::new()
    .has_headers(false)
    .from_writer(file);

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

    let base_issue_url = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);
    let base_comment_url = format!("https://api.github.com/repos/{}/{}/issues/comments", owner, repo);
    let repo_name = repo.to_string();

    for issue in issues {
        // Create a reactions JSON string as in your old CSV.
        let issue_reactions = format!(r#"{{"url": "https://api.github.com/repos/{}/{}/issues/{}/reactions", "total_count": 0, "+1": 0, "-1": 0, "laugh": 0, "hooray": 0, "confused": 0, "heart": 0, "rocket": 0, "eyes": 0}}"#, owner, repo, issue.number);
        let issue_id = match issue.databaseId {
            Some(dbid) => dbid.to_string(),
            None => issue.id.clone(),
        };
        let (user_login, user_id, user_name, user_email) = if let Some(author) = issue.author {
            (
                author.login,
                author.databaseId.map(|id| id.to_string()).unwrap_or_default(),
                author.name.unwrap_or_default(),
                author.email.unwrap_or_default(),
            )
        } else {
            (String::new(), String::new(), String::new(), String::new())
        };

        let issue_row = CsvRow {
            r#type: "issue".to_string(),
            issue_url: format!("{}/{}", base_issue_url, issue.number),
            comment_url: format!("{}/{}", base_issue_url, issue.number),
            repo_name: repo_name.clone(),
            id: issue_id,
            issue_num: issue.number,
            title: issue.title,
            user_login,
            user_id,
            user_name,
            user_email,
            issue_state: issue.state,
            created_at: issue.created_at,
            updated_at: issue.updated_at,
            body: issue.body.unwrap_or_default(),
            reactions: issue_reactions,
        };
        wtr.serialize(issue_row)?;

        for comment_node in issue.comments.nodes {
            let comment_reactions = format!(r#"{{"url": "https://api.github.com/repos/{}/{}/issues/comments/{}/reactions", "total_count": 0, "+1": 0, "-1": 0, "laugh": 0, "hooray": 0, "confused": 0, "heart": 0, "rocket": 0, "eyes": 0}}"#, owner, repo, comment_node.id);
            let comment_id = match comment_node.databaseId {
                Some(dbid) => dbid.to_string(),
                None => comment_node.id.clone(),
            };
            let (c_user_login, c_user_id, c_user_name, c_user_email) = if let Some(author) = comment_node.author {
                (
                    author.login,
                    author.databaseId.map(|id| id.to_string()).unwrap_or_default(),
                    author.name.unwrap_or_default(),
                    author.email.unwrap_or_default(),
                )
            } else {
                (String::new(), String::new(), String::new(), String::new())
            };
            let comment_row = CsvRow {
                r#type: "comment".to_string(),
                issue_url: format!("{}/{}", base_issue_url, issue.number),
                comment_url: format!("{}/{}", base_comment_url, comment_node.id),
                repo_name: repo_name.clone(),
                id: comment_id,
                issue_num: issue.number,
                title: "NA".to_string(),
                user_login: c_user_login,
                user_id: c_user_id,
                user_name: c_user_name,
                user_email: c_user_email,
                issue_state: "NA".to_string(),
                created_at: comment_node.createdAt.clone(),
                updated_at: comment_node.createdAt,
                body: comment_node.body,
                reactions: comment_reactions,
            };
            wtr.serialize(comment_row)?;
        }
    }

    wtr.flush()?;
    info!("Fetched {} issues from {}/{}", count, owner, repo);
    Ok(())
}
