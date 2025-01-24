// src/github_issues.rs

use serde::Deserialize;
use serde::Serialize;
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
    pub comments: Vec<Comment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub author: Option<Author>,
    pub body: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub login: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
struct GraphQLResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize)]
struct GraphQLError {
    message: String,
    // You can add more fields if needed
}

#[derive(Debug, Deserialize)]
struct RepositoryData {
    repository: Option<Repository>,
}

#[derive(Debug, Deserialize)]
struct Repository {
    issues: IssueConnection,
}

#[derive(Debug, Deserialize)]
struct IssueConnection {
    pageInfo: PageInfo,
    nodes: Vec<IssueNode>,
}

#[derive(Debug, Deserialize)]
struct IssueNode {
    id: String,
    number: u32,
    title: String,
    body: Option<String>,
    state: String,
    createdAt: String,
    updatedAt: String,
    closedAt: Option<String>,
    comments: CommentConnection,
}

#[derive(Debug, Deserialize)]
struct CommentConnection {
    pageInfo: PageInfo,
    nodes: Vec<CommentNode>,
}

#[derive(Debug, Deserialize)]
struct CommentNode {
    id: String,
    author: Option<AuthorNode>,
    body: String,
    createdAt: String,
}

#[derive(Debug, Deserialize)]
struct AuthorNode {
    login: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct PageInfo {
    hasNextPage: bool,
    endCursor: Option<String>,
}

/// Fetches all issues with comments for a given repository.
///
/// # Arguments
///
/// * `owner` - The owner of the repository.
/// * `repo` - The repository name.
/// * `output_path` - The file path to save the fetched issues.
///
/// # Errors
///
/// Returns an error if the request fails or if the GitHub token is not set.
pub fn fetch_issues_with_comments(owner: &str, repo: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    // Get GitHub token from environment variable
    let github_token = match env::var("GITHUB_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            error!("GITHUB_TOKEN environment variable is not set.");
            return Err("GITHUB_TOKEN environment variable is not set.".into());
        }
    };

    // Initialize HTTP client
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", github_token))?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("User-Agent", HeaderValue::from_static("rust-github-client"));

    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    // GraphQL query to fetch issues with comments
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
                        comments(first: 100) {
                            pageInfo {
                                hasNextPage
                                endCursor
                            }
                            nodes {
                                id
                                author {
                                    login
                                    url
                                }
                                body
                                createdAt
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
            for error in errors {
                error!("GraphQL error: {}", error.message);
            }
            return Err("GraphQL query failed.".into());
        }

        let data = match resp_json.data {
            Some(data) => data,
            None => {
                error!("No data received from GitHub API.");
                return Err("No data received from GitHub API.".into());
            }
        };

        let repository = match data.repository {
            Some(repo) => repo,
            None => {
                error!("Repository {} does not exist or access is denied.", repo);
                return Err(format!("Repository {} does not exist or access is denied.", repo).into());
            }
        };

        for issue_node in repository.issues.nodes {
            let mut comments = Vec::new();
            let mut comment_cursor: Option<String> = None;

            loop {
                let comment_query = r#"
                    query($owner: String!, $name: String!, $issueNumber: Int!, $cursor: String) {
                        repository(owner: $owner, name: $name) {
                            issue(number: $issueNumber) {
                                comments(first: 100, after: $cursor) {
                                    pageInfo {
                                        hasNextPage
                                        endCursor
                                    }
                                    nodes {
                                        id
                                        author {
                                            login
                                            url
                                        }
                                        body
                                        createdAt
                                    }
                                }
                            }
                        }
                    }
                "#;

                let comment_variables = json!({
                    "owner": owner,
                    "name": repo,
                    "issueNumber": issue_node.number,
                    "cursor": comment_cursor,
                });

                let comment_body = json!({
                    "query": comment_query,
                    "variables": comment_variables,
                });

                let comment_response = client.post("https://api.github.com/graphql")
                    .json(&comment_body)
                    .send()?;

                if !comment_response.status().is_success() {
                    let status = comment_response.status();
                    let text = comment_response.text()?;
                    error!("GitHub API request for comments failed with status {}: {}", status, text);
                    return Err(format!("GitHub API request for comments failed with status {}", status).into());
                }

                let comment_resp_json: GraphQLResponse<RepositoryData> = comment_response.json()?;

                if let Some(errors) = comment_resp_json.errors {
                    for error in errors {
                        error!("GraphQL error: {}", error.message);
                    }
                    return Err("GraphQL query for comments failed.".into());
                }

                let comment_data = match comment_resp_json.data {
                    Some(data) => data,
                    None => {
                        error!("No data received from GitHub API for comments.");
                        return Err("No data received from GitHub API for comments.".into());
                    }
                };

                let issue = match comment_data.repository {
                    Some(repo) => repo.issues.nodes.into_iter().next(),
                    None => None,
                };

                let issue = match issue {
                    Some(issue) => issue,
                    None => {
                        error!("Issue not found when fetching comments.");
                        break;
                    }
                };

                for comment_node in issue.comments.nodes {
                    comments.push(Comment {
                        id: comment_node.id,
                        author: comment_node.author.map(|a| Author {
                            login: a.login,
                            url: a.url,
                        }),
                        body: comment_node.body,
                        created_at: comment_node.createdAt,
                    });
                }

                if issue.comments.pageInfo.hasNextPage {
                    comment_cursor = issue.comments.pageInfo.endCursor;
                } else {
                    break;
                }
            }

            let issue = Issue {
                id: issue_node.id,
                number: issue_node.number,
                title: issue_node.title,
                body: issue_node.body,
                state: issue_node.state,
                created_at: issue_node.createdAt,
                updated_at: issue_node.updatedAt,
                closed_at: issue_node.closedAt,
                comments,
            };

            all_issues.push(issue);
        }

        if repository.issues.pageInfo.hasNextPage {
            cursor = repository.issues.pageInfo.endCursor;
        } else {
            break;
        }
    }

    // Serialize all issues to JSON and save to file
    let path = Path::new(output_path);
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, &all_issues)?;

    info!("Fetched {} issues from {}/{}", all_issues.len(), owner, repo);

    Ok(())
}
