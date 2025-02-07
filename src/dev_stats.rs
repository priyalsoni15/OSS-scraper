use crate::utils::*;
use crate::{repo::Repo, Args};
use git2::{DiffFindOptions, DiffOptions, Error};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct DevStats<'a> {
    /// Project's name
    pub project: &'a str,
    /// Project's start date - usually formatted as Y-m-d
    pub start_date: &'a str,
    /// Project's end date - usually formatted as Y-m-d
    pub end_date: &'a str,
    /// Project status: graduated, retired
    pub status: &'a str,
    /// Git repo
    #[serde(skip_serializing)]
    repo: &'a Repo<'a>,
    /// Java path
    #[serde(skip_serializing)]
    pub java_path: &'a str,
    /// Metrics
    #[serde(flatten)]
    pub metrics: CommitFileMetrics,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct CommitFileMetrics {
    pub incubation_month: usize,
    pub commit_sha: String,
    pub email: String,
    pub name: String,
    pub date: String,
    pub timestamp: i64,
    pub filename: String,
    pub change_type: String,
    pub lines_added: usize,
    pub lines_deleted: usize,
    pub commit_message: String,
}

impl<'a> DevStats<'a> {
    pub fn new(project: &'a str, repo: &'a Repo, java_path: &'a str) -> Self {
        Self {
            project,
            start_date: repo.start_date,
            end_date: repo.end_date,
            status: repo.status,
            metrics: CommitFileMetrics::default(),
            java_path,
            repo,
        }
    }

    /// Compute statistics per developer from each commit
    /// We want:
    /// incubation_month, commit sha, email, name, date, timestamp (unix), filename, change_type, loc added, loc deleted
    pub fn compute_individual_dev_stats(&self, args: &Args) -> Result<Vec<DevStats>, Error> {
        let inc_months_commits = if let None = args.flag_time_window {
            self.repo.inc_month_commits.clone()
        } else {
            self.repo.commits_to_inc_months_with_time_windows(
                args.flag_time_window.unwrap(),
                &self.repo.commits,
            )?
        };

        log::info!("{}", format!("{} - computing stats", self.project));
        log::info!(
            "{} - found {} commits",
            self.project,
            inc_months_commits
                .values()
                .fold(0, |sum, val| sum + val.len())
        );

        let mut output: Vec<DevStats> = vec![];
        for (month, commits) in inc_months_commits.iter() {
            for commit in commits {
                let author = commit.author();
                let name = author.name().unwrap_or("");
                let email = author.email().unwrap_or("");
                let commit_sha = commit.id().to_string();
                let date = convert_time(&commit.time()).to_string();

                let (_diffopts, mut diffopts2) = (DiffOptions::new(), DiffOptions::new());

                let a = if commit.parents().len() == 1 {
                    let parent = commit.parent(0);
                    if let Ok(parent) = parent {
                        let tree = parent.tree();
                        if let Ok(tree) = tree {
                            Some(tree)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };
                let b = commit.tree();
                let tree = if let Ok(b) = b { Some(b) } else { None };

                let diff = self.repo.repo.diff_tree_to_tree(
                    a.as_ref(),
                    tree.as_ref(),
                    Some(&mut diffopts2),
                );

                if let Ok(mut diff) = diff {
                    let mut current_filename = String::from("");
                    let mut lines_added = 0;
                    let mut lines_deleted = 0;
                    let mut diff_find_options = DiffFindOptions::new();

                    // set the rename threshold to 50, the default Git one
                    diff.find_similar(Some(diff_find_options.rename_threshold(50)))?;

                    let mut file_data = indexmap::IndexMap::<String, (usize, usize, String)>::new();

                    let _diff_result =
                        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
                            if let Some(p) = _delta.new_file().path() {
                                current_filename = String::from(p.to_string_lossy());
                            } else {
                                current_filename = if let Some(p) = _delta.old_file().path() {
                                    String::from(p.to_string_lossy())
                                } else {
                                    "".to_string()
                                }
                            }

                            let change_type = match _delta.status() {
                                git2::Delta::Added => "A",
                                git2::Delta::Deleted => "D",
                                git2::Delta::Modified => "M",
                                git2::Delta::Renamed => "R",
                                _ => "U",
                            };

                            match line.origin() {
                                '+' => {
                                    if let Some(x) = file_data.get_mut(&current_filename) {
                                        x.0 += 1;
                                    } else {
                                        file_data.insert(
                                            current_filename.clone(),
                                            (1, 0, change_type.to_string()),
                                        );
                                    }
                                    lines_added += 1;
                                }
                                '-' => {
                                    if let Some(x) = file_data.get_mut(&current_filename) {
                                        x.1 += 1;
                                    } else {
                                        file_data.insert(
                                            current_filename.clone(),
                                            (0, 1, change_type.to_string()),
                                        );
                                    }
                                    lines_deleted += 1;
                                }

                                _ => {}
                            }

                            true
                        });

                    for (k, (lines_added, lines_deleted, change_type)) in file_data {
                        let m = CommitFileMetrics {
                            incubation_month: *month,
                            commit_sha: commit_sha.clone(),
                            email: email.to_string(),
                            name: name.to_string(),
                            date: date.clone(),
                            timestamp: commit.time().seconds(),
                            filename: k,
                            change_type: change_type.to_string(),
                            lines_added,
                            lines_deleted,
                            commit_message: if args.flag_ignore_commit_message {
                                "".to_string()
                            } else {
                                commit
                                    .message()
                                    .unwrap_or("")
                                    .replace("\n", " _nl_ ")
                                    .to_string()
                            },
                        };
                        output.push(DevStats {
                            project: self.project,
                            status: self.status,
                            start_date: self.start_date,
                            end_date: self.end_date,
                            java_path: self.java_path,
                            repo: self.repo,
                            metrics: m.clone(),
                        });
                    }
                }
            }
        }
        Ok(output)
    }
}
