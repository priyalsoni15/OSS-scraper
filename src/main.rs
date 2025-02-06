// #![deny(warnings)]
use calamine::{open_workbook, Reader, Xlsx};
use git2::Repository;
use git2::{Error, ErrorCode};
use indexmap::{IndexMap, IndexSet};
use log::{error, info};

use rayon::iter::{ParallelBridge, ParallelIterator};

use std::sync::{Arc, RwLock};
use std::time::Duration;
use structopt::StructOpt;

// Load .env file to bring in environment variables (like GITHUB_TOKEN)
use dotenv::dotenv;

mod commits_metrics;
mod dev_stats;
mod emails;
mod github_issues;
mod metrics;
mod pre_post_incubation_analysis;
mod project;
mod repo;
mod sokrates_metrics;
mod statistics;
mod tokei_metrics;
mod utils;

use crate::dev_stats::DevStats;
use crate::project::Project;
use crate::repo::*;
use crate::statistics::*;
use crate::utils::*;
use pre_post_incubation_analysis::pre_post_analysis;
use github_issues::fetch_issues_with_comments_csv;

#[derive(StructOpt)]
pub struct Args {
    // Existing flags

    #[structopt(name = "force-full-analysis", long)]
    /// Force to run a full analysis for those projects that do not have pre incubation commits. So we will only have during and post incubation analysis
    flag_force_full_analysis: bool,

    #[structopt(name = "full-analysis", long)]
    /// Run a full analysis - pre/during/post incubation. This only runs on repositories that have previous/post incubation period commits
    flag_full_analysis: bool,

    #[structopt(name = "threads", long)]
    /// Number of threads
    flag_threads: Option<usize>,

    #[structopt(name = "download-emails", long)]
    /// Download all projects' emails
    flag_download_emails: bool,

    #[structopt(name = "project", long)]
    /// Only parse given project
    flag_parse_single_project: Option<String>,

    #[structopt(name = "list-projects", long)]
    /// Only show projects
    flag_list_projects: bool,

    #[structopt(name = "skip-tokei", long)]
    /// Skip tokei & folder analysis
    flag_skip_tokei: bool,

    #[structopt(name = "skip-sokrates", long)]
    /// Skip sokrates analysis
    flag_skip_sokrates: bool,

    #[structopt(name = "skip-emails", long)]
    /// Skip email analysis
    flag_skip_email_analysis: bool,

    #[structopt(name = "commit-messages", long)]
    /// Extract commit messages
    flag_commit_messages: bool,

    #[structopt(name = "missing-emails", long)]
    /// Check for any missing email archives
    flag_missing_emails: bool,

    #[structopt(name = "supported-languages", long)]
    /// Print supported languages
    flag_print_supported_languages: bool,

    #[structopt(name = "restrict-languages", long)]
    /// Restrict supported languages
    flag_restrict_languages: bool,

    #[structopt(name = "manual-test", long)]
    /// Manual test project
    flag_manual_test_project: Option<String>,

    #[structopt(name = "output-folder", long)]
    /// Set output folder, otherwise default is data
    flag_output_folder: Option<String>,

    #[structopt(name = "metadata-filepath", long)]
    /// Path to project's metadata
    flag_metadata_filepath: Option<String>,

    #[structopt(name = "commit-devs-files", long)]
    /// Return a list of commits and the files changed in each file together with whom changed the file
    flag_commit_devs_files: bool,

    #[structopt(name = "time-window", long)]
    /// Instead of analyzing by default incubation-months (start-date until end of month, and then monthly basis),
    /// analyze per a time window provided by the user. For example time-window=10, each incubation month will be 10 days
    flag_time_window: Option<i64>,

    #[structopt(name = "incubation-dates", long)]
    /// Print for each incubation month, the start and end date for time window projects
    flag_print_incubation_dates: bool,

    #[structopt(name = "ignore-start-end-date", long)]
    /// Ignore the start and end date from the Excel sheet input, and run from the first to the last commit in the project
    flag_ignore_start_end_dates: bool,

    #[structopt(name = "ignore-commit-message", long)]
    /// Use this option together with the commit-devs option, to ignore the commit messages and not send them to output
    flag_ignore_commit_message: bool,

    #[structopt(name = "git-folder", long)]
    /// Use this option to provide a git folder with the projects. You likely also want to use the flag ignore start end dates
    flag_git_folder: Option<String>,

    #[structopt(name = "fetch-github-issues", long)]
    /// Fetch GitHub issues and comments for projects or a single repository
    flag_fetch_github_issues: bool,

    #[structopt(name = "github-output-folder", long)]
    /// Specify the output folder for GitHub issues CSV file
    flag_github_output_folder: Option<String>,

    #[structopt(name = "github-url", long)]
    /// GitHub repository URL to fetch issues from (e.g., https://github.com/apache/hunter.git)
    flag_github_url: Option<String>,
}

fn list_projects(metadata_filepath: &str) -> indexmap::IndexSet<Project> {
    let path = metadata_filepath;
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    let mut projects = indexmap::IndexSet::<Project>::new();
    // Read whole worksheet data and provide some statistics
    if let Some(Ok(range)) = workbook.worksheet_range("projects") {
        let status_col = 2;
        let start_date_col = 5;
        let end_date_col = 6;
        let github_url_col = 7;
        for row in range.rows() {
            let status = row[status_col].get_string().unwrap().to_string();
            let start_date = row[start_date_col].get_string().unwrap().to_string();

            if status == "graduated".to_string() || status == "retired" {
                let end_date = row[end_date_col].get_string().unwrap().to_string();
                let github_url = row[github_url_col].get_string();

                if github_url.is_some() && !github_url.unwrap().to_string().is_empty() {
                    // repo_name, repo_path, start_date, end_date
                    let repo = github_url
                        .unwrap()
                        .to_string()
                        .split("/")
                        .last()
                        .unwrap()
                        .to_string();
                    let repo_path = format!("../../projects/git/{}", repo);
                    let repo_name = repo
                        .replace("incubator-retired-", "")
                        .replace("incubator-", "")
                        .trim()
                        .to_string();
                    projects.insert(Project {
                        name: repo_name,
                        path: repo_path,
                        start_date,
                        end_date,
                        status,
                    });
                }
            }
        }
    }
    projects
}

fn print_incubation_dates(projects: IndexSet<Project>, args: &Args) {
    let time_window = if let Some(time_window) = args.flag_time_window {
        time_window
    } else {
        log::error!("The incubation dates flag requires time-window flag specified");
        panic!(
            "Error. Check the logs. The incubation dates flag requires time-window flag specified"
        );
    };
    let mut results = Vec::<String>::new();

    results.push("project, status, start_date, end_date, incubation_month, incubation_month_start, incubation_month_end".to_string());
    projects.into_iter().for_each(|project| {
        let incubation_months = utils::parse_date_to_inc_months_with_time_window(
            &project.start_date,
            &project.end_date,
            time_window,
        );
        for (month, data) in incubation_months {
            results.push(format!(
                "{}, {}, {}, {}, {}, {}, {}",
                project.name,
                project.status,
                project.start_date,
                project.end_date,
                month,
                data.start_date,
                data.end_date
            ));
        }
    });

    let mut writer = csv::WriterBuilder::default()
        .has_headers(true)
        .from_path(format!(
            "incubation-dates-{}days-time-window.csv",
            time_window
        ))
        .unwrap();

    for r in results {
        writer.serialize(r).unwrap();
    }
}

fn _show_branch(repo: &Repository, repo_name: &str) -> Result<(), Error> {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
            None
        }
        Err(e) => return Err(e),
    };
    let head = head.as_ref().and_then(|h| h.shorthand());

    println!("{}: {}", repo_name, head.unwrap_or("HEAD (no branch)"));
    Ok(())
}

fn manual_test_project() {
    let writer = Arc::new(RwLock::new(
        csv::WriterBuilder::default()
            .has_headers(true)
            .from_path("test.csv")
            .unwrap(),
    ));
    let args = Args::from_args();
    let path = args.flag_manual_test_project.clone();
    let p = Project {
        name: "test".to_string(),
        path: path.unwrap(),
        start_date: "2022-02-01".to_string(),
        end_date: "2023-01-01".to_string(),
        status: "graduated".to_string(),
    };
    let java_path = java_path();
    let git_repo = Repository::open(p.path.as_str());
    if let Ok(git_repo) = git_repo {
        let mut repo = Repo::new(
            &git_repo,
            p.name.as_str(),
            p.start_date.as_str(),
            p.end_date.as_str(),
            p.status.as_str(),
            &args,
        )
        .unwrap();

        repo.checkout_master_main_trunk(&args);
        let mut stats = Stats::new(
            p.name.as_str(),
            &p.start_date,
            &p.end_date,
            &p.status,
            &java_path,
        );
        let metrics = stats.compute_statistics(&mut repo, &args);
        if let Ok(metrics) = metrics {
            let writer = writer.clone();
            let mut guard = writer.write().expect("Unable to lock");
            for m in metrics {
                guard.serialize(m).unwrap();
            }
        } else {
            error!("{} cannot extract the metrics", p.name.as_str());
        }
    } else {
        error!(
            "{} cannot find the git repository at {}",
            p.name.as_str(),
            p.path.as_str()
        );
    }
}

fn analyze_test_project(project: String, metadata_filepath: &str) {
    let args = Args::from_args();
    let writer = Arc::new(RwLock::new(
        csv::WriterBuilder::default()
            .has_headers(true)
            .from_path(format!("{project}.csv"))
            .unwrap(),
    ));
    let java_path = java_path();
    let projects = list_projects(metadata_filepath);
    projects.iter().filter(|x| x.name == project).for_each(|p| {
        let git_repo = Repository::open(p.path.as_str());
        if let Ok(git_repo) = git_repo {
            let mut repo = Repo::new(
                &git_repo,
                p.name.as_str(),
                p.start_date.as_str(),
                p.end_date.as_str(),
                p.status.as_str(),
                &args,
            )
            .unwrap();
            repo.checkout_master_main_trunk(&args);
            let mut stats = Stats::new(
                p.name.as_str(),
                &p.start_date,
                &p.end_date,
                &p.status,
                &java_path,
            );
            let metrics = stats.compute_statistics(&mut repo, &args);
            if let Ok(metrics) = metrics {
                let writer = writer.clone();
                let mut guard = writer.write().expect("Unable to lock");
                for m in metrics {
                    guard.serialize(m).unwrap();
                }
            } else {
                error!("{} cannot extract the metrics", p.name.as_str());
            }
        } else {
            error!(
                "{} cannot find the git repository at {}",
                p.name.as_str(),
                p.path.as_str()
            );
        }
    });
}

fn remove_sokrates_temp(repo: &Repository) {
    std::fs::remove_dir_all(format!(
        "{}/_sokrates",
        repo.path().parent().unwrap().to_str().unwrap()
    ))
    .ok();
    std::fs::remove_file(format!(
        "{}/git-history.txt",
        repo.path().parent().unwrap().to_str().unwrap()
    ))
    .ok();
}

fn check_for_missing_emails(args: &Args, metadata_filepath: &str) {
    let projects = list_projects(metadata_filepath);
    let emails_folder = "../../projects/emails";

    projects.iter().par_bridge().for_each(|p| {
        let git_repo = Repository::open(p.path.as_str());
        if let Ok(git_repo) = git_repo {
            let repo = Repo::new(
                &git_repo,
                p.name.as_str(),
                p.start_date.as_str(),
                p.end_date.as_str(),
                p.status.as_str(),
                &args,
            );
            if let Ok(repo) = repo {
                log::info!("Checking repo {}", repo.project.to_lowercase());
                for (_, month) in repo.dates_to_months() {
                    let path = format!(
                        "{}/{}-dev-{}.mbox",
                        emails_folder,
                        repo.project.to_lowercase(),
                        month
                    );
                    let email_path = std::path::Path::new(&path);
                    if email_path.exists() && email_path.metadata().unwrap().len() == 0 {
                        log::error!(
                            "{} - email archive {}-dev-{}.mbox is empty",
                            repo.project.to_lowercase(),
                            repo.project.to_lowercase(),
                            month
                        );
                    }
                    if !email_path.exists() {
                        log::error!(
                            "{} - email archive {}-dev-{}.mbox does not exist",
                            repo.project.to_lowercase(),
                            repo.project.to_lowercase(),
                            month
                        );
                    }
                }
            }
        }
    });
}

fn commits_messages(data_folder_path: &str, args: &Args, metadata_filepath: &str) {
    let projects = list_projects(metadata_filepath);
    let mut writer = csv::WriterBuilder::default()
        .has_headers(true)
        .from_path(format!("{}/commit-messages.csv", data_folder_path))
        .unwrap();

    projects.iter().for_each(|p| {
        let git_repo = Repository::open(p.path.as_str());
        if let Ok(git_repo) = git_repo {
            remove_sokrates_temp(&git_repo);
            let repo = Repo::new(
                &git_repo,
                p.name.as_str(),
                p.start_date.as_str(),
                p.end_date.as_str(),
                p.status.as_str(),
                &args,
            );
            if let Ok(mut repo) = repo {
                let checkout = repo.checkout_master_main_trunk(&args);
                if let Ok(_checkout) = checkout {
                    for (month, commits) in repo.inc_month_commits {
                        log::info!(
                            "{} - month: {} found {} commits",
                            p.name.as_str(),
                            month,
                            commits.len()
                        );
                        for c in commits {
                            match writer.serialize(CommitMessage {
                                project: p.name.to_string(),
                                status: p.status.to_string(),
                                inc_month: month,
                                sha: c.id().to_owned().to_string(),
                                message: c.message().unwrap_or("").to_string(),
                            }) {
                                Ok(()) => {}
                                Err(e) => {
                                    error!("{} - cannot serialize metric value: {}", p.name.as_str(), e);
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}

pub fn java_path() -> String {
    let java_path = match std::env::var("JAVA_HOME") {
        Ok(p) => {
            if std::env::consts::OS == "windows" {
                log::info!("OS detected: Windows");
                format!("{}\\bin\\java", p)
            } else if std::env::consts::OS == "linux" {
                log::info!("OS detected: Linux");
                if p.ends_with("/") {
                    format!("{}java", p)
                } else {
                    format!("{}/java", p)
                }
            } else {
                log::info!("OS is different than Windows or Linux, defaulting to command java. If this command is not available in your system, you have to install java and make it accessible");
                "java".to_string()
            }
        }
        Err(_e) => "java".to_string(),
    };
    java_path
}

fn print_supported_languages(exts: IndexSet<String>) {
    log::info!("Following languages are supported and files with these extensions are considered in the analysis: ");
    for e in exts {
        log::info!("{:?}, ", e)
    }
}

/// When not using a single GitHub URL, this function iterates over projects parsed from the metadata file.
fn fetch_github_issues_for_projects(projects: IndexSet<Project>, args: &Args) {
    let output_folder = args.flag_github_output_folder.as_deref().unwrap_or("github_issues");
    if let Err(e) = std::fs::create_dir_all(output_folder) {
        error!("Failed to create GitHub issues output folder {}: {}", output_folder, e);
        return;
    }
    projects.into_iter().par_bridge().for_each(|project| {
        let github_url = project.path.trim();
        let parts: Vec<&str> = github_url.split('/').collect();
        if parts.len() < 2 {
            error!("Invalid GitHub URL for project {}: {}", project.name, github_url);
            return;
        }
        let owner = parts[parts.len() - 2];
        let repo = parts[parts.len() - 1];
        let output_path = format!("{}/{}_issues.csv", output_folder, project.name);
        match fetch_issues_with_comments_csv(owner, repo, &output_path) {
            Ok(_) => info!("Successfully fetched issues for {}/{}", owner, repo),
            Err(e) => error!("Failed to fetch issues for {}/{}: {}", owner, repo, e),
        }
    });
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file, if it exists.
    dotenv().ok();

    // **** LOGGING SETUP **** //
    let start = std::time::Instant::now();
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    log::info!("Booting up");

    let args = Args::from_args();

    if args.flag_restrict_languages {
        let cwd = std::env::current_dir();
        if let Ok(mut cwd) = cwd {
            cwd.push("tokei.toml");
            if !cwd.exists() {
                log::error!("There is no tokei.toml file in current working directory. Aborting");
                panic!("There is no tokei.toml file in current working directory")
            }
        } else {
            panic!("Cannot get current working directory when initializing tokei with the configuration")
        }
    }

    // --- EARLY EXIT: If fetching GitHub issues, do not load metadata ---
    if args.flag_fetch_github_issues {
        if let Some(github_url) = &args.flag_github_url {
            let trimmed = github_url.trim_end_matches(".git");
            let parts: Vec<&str> = trimmed.split('/').collect();
            if parts.len() < 2 {
                error!("Invalid GitHub URL provided: {}", github_url);
                return Ok(());
            }
            let owner = parts[parts.len() - 2];
            let repo = parts[parts.len() - 1];
            let output_folder = args.flag_github_output_folder.as_deref().unwrap_or("github_issues");
            std::fs::create_dir_all(output_folder)?;
            let output_csv = format!("{}/{}_issues.csv", output_folder, repo);
            match fetch_issues_with_comments_csv(owner, repo, &output_csv) {
                Ok(_) => info!("Successfully fetched issues for {}/{}", owner, repo),
                Err(e) => error!("Failed to fetch issues for {}/{}: {}", owner, repo, e),
            }
        } else {
            let metadata_filepath = if let Some(path) = &args.flag_metadata_filepath {
                path
            } else {
                "../../apache-projects.xlsx"
            };
            let projects = list_projects(metadata_filepath);
            fetch_github_issues_for_projects(projects, &args);
        }
        let duration = start.elapsed();
        let seconds = duration.as_secs() % 60;
        let minutes = (duration.as_secs() / 60) % 60;
        let hours = (duration.as_secs() / 60) / 60;
        log::info!("GitHub issues fetching completed in {}h:{}m:{}s", hours, minutes, seconds);
        return Ok(());
    }
    // --- End early exit for GitHub issues ---

    // Load projects via git folder or metadata
    let mut projects = if args.flag_git_folder.is_some() {
        let folder = args.flag_git_folder.clone().unwrap();
        let mut cwd = std::env::current_dir()?;
        cwd.push(folder);
        std::fs::read_dir(cwd)?
            .filter_map(|dir| match dir {
                Ok(path) => {
                    println!("{:?}", path);
                    let git_path = path.path().to_str().unwrap().to_string();
                    Some(Project {
                        name: path.file_name().to_str().unwrap().to_string(),
                        path: git_path,
                        start_date: "".to_string(),
                        end_date: "".to_string(),
                        status: "".to_string(),
                    })
                }
                Err(e) => {
                    log::error!("Error reading directory entry: {}", e);
                    None
                }
            })
            .collect::<IndexSet<_>>()
    } else {
        let metadata_filepath = if let Some(path) = &args.flag_metadata_filepath {
            path
        } else {
            "../../apache-projects.xlsx"
        };
        let mut projects = list_projects(metadata_filepath);
        projects = projects
            .into_iter()
            .filter(|x| {
                x.name != "ODFToolkit" && x.name != "commons-ognl" && !x.name.contains("myfaces")
            })
            .collect::<indexmap::IndexSet<_>>();
        projects
    };

    let data_folder_path = if let Some(folder) = &args.flag_output_folder {
        folder.as_str()
    } else {
        "data"
    };

    if let Ok(_res) = std::fs::create_dir_all(data_folder_path) {
        log::info!("Created output folder: {}", data_folder_path);
    } else {
        log::error!("Cannot create folder {}", data_folder_path);
    }

    if args.flag_parse_single_project.is_some() {
        projects = projects
            .into_iter()
            .filter(|x| &x.name == args.flag_parse_single_project.as_ref().unwrap())
            .collect::<IndexSet<_>>();
    }

    if args.flag_list_projects {
        for p in projects {
            println!("{:#?}", p.name.to_lowercase());
        }
        return Ok(());
    }

    if args.flag_missing_emails {
        let metadata_filepath = if let Some(path) = &args.flag_metadata_filepath {
            path
        } else {
            "../../projects-info-from-podlings-xml-extra-metadata.xlsx"
        };
        check_for_missing_emails(&args, metadata_filepath);
        let duration = start.elapsed();
        let seconds = duration.as_secs() % 60;
        let minutes = (duration.as_secs() / 60) % 60;
        let hours = (duration.as_secs() / 60) / 60;
        log::info!("Analysis completed in {}h:{}m:{}s", hours, minutes, seconds);
        return Ok(());
    }

    if args.flag_print_incubation_dates {
        print_incubation_dates(projects, &args);
        return Ok(());
    }

    if args.flag_print_supported_languages {
        let exts = utils::find_lang_extensions()?;
        print_supported_languages(exts);
        return Ok(());
    }

    if args.flag_restrict_languages {
        let exts = utils::find_lang_extensions()?;
        print_supported_languages(exts);
    }

    if args.flag_commit_messages {
        let metadata_filepath = if let Some(path) = &args.flag_metadata_filepath {
            path
        } else {
            "../../projects-info-from-podlings-xml-extra-metadata.xlsx"
        };
        commits_messages(data_folder_path, &args, metadata_filepath);
        let duration = start.elapsed();
        let seconds = duration.as_secs() % 60;
        let minutes = (duration.as_secs() / 60) % 60;
        let hours = (duration.as_secs() / 60) / 60;
        log::info!("Analysis completed in {}h:{}m:{}s", hours, minutes, seconds);
        return Ok(());
    }

    if args.flag_manual_test_project.is_some() {
        manual_test_project();
        return Ok(());
    }

    let threads = args.flag_threads.unwrap_or(4);
    log::info!("Using {} threads", threads);
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .unwrap();

    if args.flag_download_emails {
        let emails_folder = "../../projects/emails";
        if let Err(result) = std::fs::create_dir_all(emails_folder) {
            log::error!("Cannot create project/emails directories. Make sure you have writing access. Original error: {}", result);
            return Ok(());
        }
        let projects_names_fix = IndexMap::from([
            ("apex-core", "apex"),
            ("ant-ivy", "ant"),
            ("derby", "db-derby"),
            ("empire-db", "empire"),
            ("lucene.net", "lucenenet"),
            ("mynewt-core", "mynewt"),
            ("pluto", "portals-pluto"),
            ("creadur-rat", "creadur"),
            ("warble-server", "warble"),
        ]);
        let agent = ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(15))
            .timeout_write(Duration::from_secs(300))
            .build();
        projects.iter().par_bridge().for_each(|p| {
            let git_repo = Repository::open(p.path.as_str());
            let end_date = if p.status == "graduated" {
                chrono::Local::now().format("%Y-%m-%d").to_string()
            } else {
                p.end_date.to_string()
            };
            if let Ok(git_repo) = git_repo {
                let repo = Repo::new(
                    &git_repo,
                    p.name.as_str(),
                    &p.start_date,
                    &end_date,
                    p.status.as_str(),
                    &args,
                );
                if let Ok(repo) = repo {
                    for (_, month) in repo.dates_to_months() {
                        let path = format!(
                            "{}/{}-dev-{}.mbox",
                            emails_folder,
                            repo.project.to_lowercase(),
                            month
                        );
                        let email_path = std::path::Path::new(&path);
                        if !email_path.exists() {
                            let url = format!(
                                "https://mail-archives.apache.org/mod_mbox/{}-dev/{}.mbox",
                                projects_names_fix
                                    .get(repo.project.to_lowercase().as_str())
                                    .unwrap_or(&repo.project.to_lowercase().as_str()),
                                month
                            );
                            let res = agent.get(&url).call();
                            if let Ok(res) = res {
                                if res.status() == 200 {
                                    let mut file = std::fs::File::create(&path)
                                        .expect("Cannot create file");
                                    let result = std::io::copy(&mut res.into_reader(), &mut file);
                                    if result.is_err() {
                                        log::error!("{} - cannot download email archive {}", repo.project.to_lowercase(), &month);
                                    } else {
                                        log::info!("{} - downloaded email archive {}", repo.project.to_lowercase(), &month);
                                    }
                                } else {
                                    log::error!("{} - cannot download email archive {}", repo.project.to_lowercase(), &month);
                                }
                            } else {
                                log::error!("{} - cannot download email archive {}", repo.project.to_lowercase(), &month);
                            }
                        }
                    }
                }
            }
        });
        let duration = start.elapsed();
        let seconds = duration.as_secs() % 60;
        let minutes = (duration.as_secs() / 60) % 60;
        let hours = (duration.as_secs() / 60) / 60;
        log::info!("Analysis completed in {}h:{}m:{}s", hours, minutes, seconds);
        return Ok(());
    }

    if args.flag_full_analysis {
        let java_path = java_path();
        pre_post_analysis(projects, &args, &java_path, data_folder_path);
        return Ok(());
    }

    log::info!("Analyzing {} projects", projects.len());
    let java_path = java_path();
    projects.iter().par_bridge().for_each(|p| {
        let git_repo = Repository::open(p.path.as_str());
        if let Ok(git_repo) = git_repo {
            let repo = Repo::new(
                &git_repo,
                p.name.as_str(),
                p.start_date.as_str(),
                p.end_date.as_str(),
                p.status.as_str(),
                &args,
            );
            if let Ok(mut repo) = repo {
                remove_sokrates_temp(&git_repo);
                let checkout = repo.checkout_master_main_trunk(&args);
                if let Ok(_checkout) = checkout {
                    log::info!("checkout {}", repo.commits.len());
                    if args.flag_commit_devs_files {
                        let dev_stats = DevStats::new(p.name.as_str(), &repo, &java_path);
                        let metrics = dev_stats.compute_individual_dev_stats(&args);
                        if let Ok(metrics) = metrics {
                            let mut writer = csv::WriterBuilder::default()
                                .has_headers(false)
                                .from_path(format!("{}/{}-commit-file-dev.csv", data_folder_path, p.name.as_str()))
                                .unwrap();
                            for m in metrics {
                                if let Err(e) = writer.serialize(m) {
                                    error!("{} - cannot serialize metric value: {}", p.name.as_str(), e);
                                }
                            }
                        } else {
                            error!("{} cannot extract the metrics", p.name.as_str());
                        }
                    } else {
                        let mut stats = Stats::new(
                            p.name.as_str(),
                            &repo.start_date,
                            &repo.end_date,
                            &p.status,
                            &java_path,
                        );
                        let metrics = stats.compute_statistics(&mut repo, &args);
                        if let Ok(metrics) = metrics {
                            let mut writer = csv::WriterBuilder::default()
                                .has_headers(true)
                                .from_path(format!("{}/{}.csv", data_folder_path, p.name.as_str()))
                                .unwrap();
                            for m in metrics {
                                if let Err(e) = writer.serialize(m) {
                                    error!("{} - cannot serialize metric value: {}", p.name.as_str(), e);
                                }
                            }
                        } else {
                            error!("{} cannot extract the metrics", p.name.as_str());
                        }
                    }
                } else {
                    error!("{} - cannot reset to main/master/trunk", p.name.as_str());
                }
            } else {
                error!("{} - cannot parse the repository and extract commits", p.name.as_str());
            }
        } else {
            error!("{} cannot find the git repository at {}", p.name.as_str(), p.path.as_str());
        }
    });
    let duration = start.elapsed();
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = (duration.as_secs() / 60) / 60;
    log::info!("Analysis completed in {}h:{}m:{}s", hours, minutes, seconds);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_git_path() {
        let repo = Repository::open("test_resources/git_repo");
        let cwd = format!(
            "{}/test_resources/git_repo",
            std::env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .replace("\\", "/")
        );
        if let Ok(r) = repo {
            assert_eq!(r.path().parent().unwrap().to_str().unwrap_or(""), cwd);
        }
    }
}
