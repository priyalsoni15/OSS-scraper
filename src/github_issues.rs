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
    pub id: String,
    pub databaseId: Option<u64>,
    pub body: String,
    pub created_at: String,
    pub author: Option<AuthorNode>,
}

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

#[derive(Debug, Deserialize)]
struct IssueData {
    issue: Option<CommentedIssue>,
}

#[derive(Debug, Deserialize)]
struct CommentedIssue {
    comments: CommentConnection,
}

// CSV row type
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

/// Fetch issues (with comments) from GitHub GraphQL
fn fetch_issues(owner: &str, repo: &str) -> Result<Vec<Issue>, Box<dyn Error>> {
    let github_token = env::var("GITHUB_TOKEN")
        .map_err(|_| "GITHUB_TOKEN environment variable is not set.")?;
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", github_token))?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("User-Agent", HeaderValue::from_static("rust-github-client"));
    let client = Client::builder().default_headers(headers).build()?;

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
        let response = client
            .post("https://api.github.com/graphql")
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

/// Writes issues + comments to CSV
pub fn fetch_issues_with_comments_csv(owner: &str, repo: &str, output_csv_path: &str) -> Result<(), Box<dyn Error>> {
    let issues = fetch_issues(owner, repo)?;
    let count = issues.len();

    let path = Path::new(output_csv_path);
    let file = File::create(path)?;
    let mut wtr = csv::WriterBuilder::new().has_headers(false).from_writer(file);

    // Write CSV header
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

    // For building final links:
    let base_issue_url = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);
    let base_comment_url = format!("https://api.github.com/repos/{}/{}/issues/comments", owner, repo);

    let repo_name = repo.to_string();

    for issue in issues {
        // Build JSON-like reactions placeholder
        let issue_reactions = format!(
            r#"{{"url": "https://api.github.com/repos/{}/{}/issues/{}/reactions", "total_count": 0, "+1": 0, "-1": 0, "laugh": 0, "hooray": 0, "confused": 0, "heart": 0, "rocket": 0, "eyes": 0}}"#,
            owner, repo, issue.number
        );
        // Use numeric databaseId if present:
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

        // Write an "issue" row
        let issue_row = CsvRow {
            r#type: "issue".to_string(),
            issue_url: format!("{}/{}", base_issue_url, issue.number),
            // For the issue row, we keep comment_url the same as issue_url (as in your original code).
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

        // Now for each comment on this issue
        for comment_node in issue.comments.nodes {
            // Build comment reactions placeholder
            // Use numeric databaseId if present for the final link:
            let comment_url_id = match comment_node.databaseId {
                Some(dbid) => dbid.to_string(),
                None => comment_node.id.clone(),
            };
            let comment_reactions = format!(
                r#"{{"url": "https://api.github.com/repos/{}/{}/issues/comments/{}/reactions", "total_count": 0, "+1": 0, "-1": 0, "laugh": 0, "hooray": 0, "confused": 0, "heart": 0, "rocket": 0, "eyes": 0}}"#,
                owner, repo, comment_url_id
            );
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

            // Write a "comment" row
            let comment_row = CsvRow {
                r#type: "comment".to_string(),
                issue_url: format!("{}/{}", base_issue_url, issue.number),
                comment_url: format!("{}/{}", base_comment_url, comment_id),
                repo_name: repo_name.clone(),
                id: comment_id,
                issue_num: issue.number,
                title: "NA".to_string(), // no separate title for a comment
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

/// If you want grouped-by-developer logic (issues + comments)
pub fn write_issue_stats_grouped_by_developer(owner: &str, repo: &str, output_folder: &str) -> Result<(), Box<dyn Error>> {
    use std::collections::HashMap;
    use chrono::DateTime;

    #[derive(Debug, Serialize)]
    struct IssueDevStat {
        date_time: String,
        file: String,
        committer_name: String,
        committer_email: String,
        url: String,
        month: String,
    }

    let issues = fetch_issues(owner, repo)?;
    let mut grouped_stats: HashMap<String, Vec<IssueDevStat>> = HashMap::new();

    let extract_month = |dt_str: &str| -> String {
        if let Ok(dt) = DateTime::parse_from_rfc3339(dt_str) {
            dt.format("%m").to_string()
        } else {
            String::from("NA")
        }
    };

    for issue in issues.iter() {
        // Issue's author
        if let Some(author) = &issue.author {
            let name = author.name.clone().unwrap_or(author.login.clone());
            let email = author.email.clone().unwrap_or_default();
            let month = extract_month(&issue.created_at);
            let url = format!("https://github.com/{}/{}/issues/{}", owner, repo, issue.number);
            let stat = IssueDevStat {
                date_time: issue.created_at.clone(),
                file: issue.title.clone(),
                committer_name: name.clone(),
                committer_email: email,
                url,
                month,
            };
            grouped_stats.entry(name).or_default().push(stat);
        }
        // Comments
        for comment in issue.comments.nodes.iter() {
            if let Some(author) = &comment.author {
                let name = author.name.clone().unwrap_or(author.login.clone());
                let email = author.email.clone().unwrap_or_default();
                let month = extract_month(&comment.createdAt);
                let url_id = match comment.databaseId {
                    Some(dbid) => dbid.to_string(),
                    None => comment.id.clone(),
                };
                let url = format!("https://github.com/{}/{}/issues/comments/{}", owner, repo, url_id);
                let stat = IssueDevStat {
                    date_time: comment.createdAt.clone(),
                    file: issue.title.clone(),
                    committer_name: name.clone(),
                    committer_email: email,
                    url,
                    month,
                };
                grouped_stats.entry(name).or_default().push(stat);
            }
        }
    }

    std::fs::create_dir_all(output_folder)?;

    for (dev_name, stats) in grouped_stats {
        let file_path = format!("{}/{}.csv", output_folder, dev_name);
        let file = File::create(&file_path)?;
        let mut writer = csv::Writer::from_writer(file);
        writer.write_record(&["date_time", "file", "committer_name", "committer_email", "comment_url/issue_url", "month"])?;

        for stat in stats {
            writer.write_record(&[
                stat.date_time,
                stat.file,
                stat.committer_name,
                stat.committer_email,
                stat.url,
                stat.month,
            ])?;
        }
        writer.flush()?;
        info!("Issue stats written to {}", file_path);
    }

    Ok(())
}
