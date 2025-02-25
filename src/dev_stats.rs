// dev_stats.rs
use crate::utils::*;
use crate::{repo::Repo, Args};
use git2::{DiffFindOptions, DiffOptions, Error};
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;

#[derive(Serialize, Clone)]
pub struct DevStats<'a> {
    pub project: &'a str,
    pub start_date: &'a str,
    pub end_date: &'a str,
    pub status: &'a str,
    #[serde(skip_serializing)]
    pub repo: &'a Repo<'a>,
    #[serde(skip_serializing)]
    pub java_path: &'a str,
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

    // NEW FIELD: can be populated by external code (e.g., commit_metrics_clone)
    #[serde(default)]
    pub commit_url: String,
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

                    for (filename, (added, deleted, change_type)) in file_data {
                        let m = CommitFileMetrics {
                            incubation_month: *month,
                            commit_sha: commit_sha.clone(),
                            email: email.to_string(),
                            name: name.to_string(),
                            date: date.clone(),
                            timestamp: commit.time().seconds(),
                            filename,
                            change_type: change_type.to_string(),
                            lines_added: added,
                            lines_deleted: deleted,
                            commit_message: if args.flag_ignore_commit_message {
                                "".to_string()
                            } else {
                                commit
                                    .message()
                                    .unwrap_or("")
                                    .replace("\n", " _nl_ ")
                                    .to_string()
                            },
                            commit_url: String::new(), // default empty; can be set externally
                        };
                        output.push(DevStats {
                            project: self.project,
                            status: self.status,
                            start_date: self.start_date,
                            end_date: self.end_date,
                            java_path: self.java_path,
                            repo: self.repo,
                            metrics: m,
                        });
                    }
                }
            }
        }
        Ok(output)
    }

    /// Writes grouped developer statistics into separate CSV files
    pub fn write_dev_stats_grouped_by_developer(&self, args: &Args) -> Result<(), Box<dyn std::error::Error>> {
        let stats = self.compute_individual_dev_stats(args)?;

        let mut grouped_stats: HashMap<String, Vec<&CommitFileMetrics>> = HashMap::new();
        for stat in &stats {
            grouped_stats.entry(stat.metrics.name.clone()).or_default().push(&stat.metrics);
        }

        let output_folder = args.flag_output_folder.as_deref().unwrap_or("output");
        std::fs::create_dir_all(output_folder)?;

        for (dev_name, metrics) in grouped_stats {
            let file_path = format!("{}/{}.csv", output_folder, dev_name);
            let mut writer = csv::Writer::from_writer(File::create(&file_path)?);

            // Example header row
            writer.write_record(&[
                "date_time",
                "file",
                "committer_name",
                "committer_email",
                "commit_link",
                "month"
            ])?;

            for metric in metrics {
                // We'll build a link in the same style:
                let commit_link = if metric.commit_url.is_empty() {
                    // fallback if not set
                    format!("https://github.com/{}/commit/{}", self.project, metric.commit_sha)
                } else {
                    metric.commit_url.clone()
                };

                writer.write_record(&[
                    metric.date.clone(),
                    metric.filename.clone(),
                    metric.name.clone(),
                    metric.email.clone(),
                    commit_link,
                    metric.incubation_month.to_string(),
                ])?;
            }

            writer.flush()?;
            log::info!("Developer stats written to {}", file_path);
        }

        Ok(())
    }
}
