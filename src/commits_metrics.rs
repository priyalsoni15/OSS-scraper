use core::panic;

use crate::utils::{self, convert_time};
use crate::Repo;
use git2::{Commit, Diff, DiffFindOptions, DiffFormat, DiffOptions, Error};
use indexmap::map::Entry;
use indexmap::{IndexMap, IndexSet};
#[derive(Default, Clone)]
pub struct DiffData {
    added_lines: usize,
    deleted_lines: usize,
    files_added: usize,
    files_deleted: usize,
    files_renamed: usize,
    files_modified: usize,
}

impl DiffData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse_diff(&self, mut diff: Diff) -> Self {
        let mut added_lines = 0;
        let mut deleted_lines = 0;
        let mut files_added = 0;
        let mut files_deleted = 0;
        let mut files_renamed = 0;
        let mut files_modified = 0;

        let mut new_file = false;
        let mut filename = String::from("");

        // set the rename threshold to 50, the default Git one
        let mut diff_find_options = DiffFindOptions::new();
        let diff_rename_threshold = 50;
        let rename_threshold = diff.find_similar(Some(
            diff_find_options.rename_threshold(diff_rename_threshold),
        ));
        if rename_threshold.is_err() {
            panic!(
                "Cannot set rename threshold for Git diff options to {:?}",
                diff_rename_threshold
            );
        }
        let diff_result = diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
            match line.origin() {
                '+' => {
                    added_lines += 1;
                    //let e = result.get_mut(&filename).unwrap();
                    //e.0 += 1
                }
                '-' => {
                    deleted_lines += 1;
                }
                'F' => {
                    new_file = true;
                    if let Some(p) = _delta.new_file().path() {
                        filename = String::from(p.to_string_lossy());
                    } else {
                        // filename = String::from(_delta.old_file().path().unwrap().to_string_lossy())
                        filename = if let Some(p) = _delta.old_file().path() {
                            p.to_string_lossy().to_string()
                        } else {
                            "".to_string()
                        }
                    }
                }
                _ => {}
            }

            if new_file {
                match _delta.status() {
                    git2::Delta::Added => {
                        files_added += 1;
                        // result.insert(filename.to_owned(), (0, 0, "A".to_string()));
                        new_file = false;
                    }
                    git2::Delta::Deleted => {
                        // println!("File deleted, {:?}", _delta.old_file().path());
                        files_deleted += 1;
                        // result.insert(filename.to_owned(), (0, 0, "D".to_string()));
                        new_file = false;
                    }
                    git2::Delta::Modified => {
                        files_modified += 1;
                        // println!("File modified, {:?}", _delta.new_file().path());
                        // result.insert(filename.to_owned(), (0, 0, "M".to_string()));
                        new_file = false;
                    }
                    git2::Delta::Renamed => {
                        files_renamed += 1;
                        // println!("File renamed, {:?}", _delta.new_file().path());
                        // result.insert(filename.to_owned(), (0, 0, "R".to_string()));
                        new_file = false;
                    }
                    _ => {}
                }
            }
            true
        });
        if let Err(_e) = diff_result {
            log::error!("Cannot parse the diff to extract metadata");
        }
        Self {
            added_lines,
            deleted_lines,
            files_added,
            files_deleted,
            files_modified,
            files_renamed,
        }
    }

    pub fn parse_diff_restricted_langs(
        &self,
        diff: &Diff,
        extensions: &IndexSet<String>,
    ) -> Option<Self> {
        let mut added_lines = 0;
        let mut deleted_lines = 0;
        let mut files_added = 0;
        let mut files_deleted = 0;
        let mut files_renamed = 0;
        let mut files_modified = 0;

        let mut new_file = false;
        let mut filename = String::from("");
        let mut source_code = false;
        let diff_result = diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
            if utils::is_source_file(_delta.new_file().path(), extensions)
                || utils::is_source_file(_delta.old_file().path(), extensions)
            {
                // log::info!("{:?}", _delta);
                source_code = true;
                log::debug!(
                    "Analyzing file with extension: {:?}",
                    _delta.old_file().path().unwrap().extension()
                );
                match line.origin() {
                    '+' => {
                        added_lines += 1;
                        //let e = result.get_mut(&filename).unwrap();
                        //e.0 += 1
                    }
                    '-' => {
                        deleted_lines += 1;
                    }
                    'F' => {
                        new_file = true;
                        if let Some(p) = _delta.new_file().path() {
                            filename = String::from(p.to_string_lossy());
                        } else {
                            // filename = String::from(_delta.old_file().path().unwrap().to_string_lossy())
                            filename = if let Some(p) = _delta.old_file().path() {
                                p.to_string_lossy().to_string()
                            } else {
                                "".to_string()
                            }
                        }
                    }
                    _ => {}
                }

                if new_file {
                    match _delta.status() {
                        git2::Delta::Added => {
                            files_added += 1;
                            // result.insert(filename.to_owned(), (0, 0, "A".to_string()));
                            new_file = false;
                        }
                        git2::Delta::Deleted => {
                            // println!("File deleted, {:?}", _delta.old_file().path());
                            files_deleted += 1;
                            // result.insert(filename.to_owned(), (0, 0, "D".to_string()));
                            new_file = false;
                        }
                        git2::Delta::Modified => {
                            files_modified += 1;
                            // println!("File modified, {:?}", _delta.new_file().path());
                            // result.insert(filename.to_owned(), (0, 0, "M".to_string()));
                            new_file = false;
                        }
                        git2::Delta::Renamed => {
                            files_renamed += 1;
                            // println!("File renamed, {:?}", _delta.new_file().path());
                            // result.insert(filename.to_owned(), (0, 0, "R".to_string()));
                            new_file = false;
                        }
                        _ => {}
                    }
                }
                true
                // });
            } else {
                true
            }
        });
        if let Err(_e) = diff_result {
            log::error!("Cannot parse the diff to extract metadata");
        }

        if source_code {
            Some(Self {
                added_lines,
                deleted_lines,
                files_added,
                files_deleted,
                files_modified,
                files_renamed,
            })
        } else {
            None
        }
    }
}

pub struct CommitsMetrics<'a> {
    pub commits: Vec<Commit<'a>>,
    diffs: Vec<DiffData>,
}

impl<'a> CommitsMetrics<'a> {
    pub fn new(repo: &'a Repo<'a>, commits: &[Commit<'a>]) -> Result<Self, Error> {
        let (_diffopts, mut diffopts2) = (DiffOptions::new(), DiffOptions::new());
        let diffs = commits
            .iter()
            .filter_map(|c| {
                let a = if c.parents().len() >= 1 {
                    let parent = c.parent(0).ok()?;
                    Some(parent.tree().ok()?)
                } else {
                    None
                };
                let b = c.tree().ok()?;
                let diff = repo
                    .repo
                    .diff_tree_to_tree(a.as_ref(), Some(&b), Some(&mut diffopts2))
                    .ok()?;
                let diff_data = DiffData::new();
                Some(diff_data.parse_diff(diff))
            })
            .collect::<Vec<_>>();
        Ok(Self {
            commits: commits.to_vec(),
            diffs,
        })
        // }
    }

    /// The number of active days. An active day is a (calendar) day that had at least one commit
    pub fn active_days(&self) -> usize {
        let commits = &self.commits;
        commits
            .iter()
            .map(|c| {
                convert_time(&c.committer().when())
                    .format("%Y-%m-%d")
                    .to_string()
            })
            .collect::<IndexSet<_>>()
            .len()
    }

    /// The number of added lines over all commits
    pub fn added_lines(&self) -> usize {
        let diffs = &self.diffs;
        diffs.iter().map(|d| d.added_lines).sum()
    }

    /// The number of deleted lines over all commits
    pub fn deleted_lines(&self) -> usize {
        let diffs = &self.diffs;
        diffs.iter().map(|d| d.deleted_lines).sum()
    }

    /// The number of added files over all commits
    pub fn files_added(&self) -> usize {
        let diffs = &self.diffs;
        diffs.iter().map(|d| d.files_added).sum()
    }

    /// The number of deleted files over all commits
    pub fn files_deleted(&self) -> usize {
        let diffs = &self.diffs;
        diffs.iter().map(|d| d.files_deleted).sum()
    }

    /// The number of modified files over all commits
    pub fn files_modified(&self) -> usize {
        let diffs = &self.diffs;
        diffs.iter().map(|d| d.files_modified).sum()
    }

    /// The number of renamed files in all these commits
    pub fn files_renamed(&self) -> usize {
        let diffs = &self.diffs;
        diffs.iter().map(|d| d.files_renamed).sum()
    }

    /// A set of authors' names
    pub fn _authors_names(&self) -> usize {
        let commits = &self.commits;
        commits
            .iter()
            .map(|c| c.author().name().unwrap_or("").to_string())
            .collect::<IndexSet<_>>()
            .len()
    }

    /// A set of authors' emails
    pub fn authors_emails(&self) -> IndexSet<String> {
        let commits = &self.commits;
        commits
            .iter()
            .map(|c| {
                log::debug!(
                    "Author: {} - {}",
                    c.author().email().unwrap_or("").to_string(),
                    c.id()
                );
                c.author().email().unwrap_or("").to_string()
            })
            .collect::<IndexSet<_>>()
    }

    /// A set of committers' names
    pub fn _committers_names(&self) -> usize {
        let commits = &self.commits;
        commits
            .iter()
            .map(|c| c.committer().name().unwrap_or("").to_string())
            .collect::<IndexSet<_>>()
            .len()
    }
    /// A set of committers' emails
    pub fn committers_emails(&self) -> IndexSet<String> {
        let commits = &self.commits;
        commits
            .iter()
            .map(|c| {
                log::debug!(
                    "Committer {} - {}",
                    c.committer().name().unwrap_or("").to_string(),
                    c.id()
                );
                c.committer().email().unwrap_or("").to_string()
            })
            .collect::<IndexSet<_>>()
    }

    fn _authors_commits(&self) -> IndexMap<String, usize> {
        let commits = &self.commits;

        let mut committers_commits = IndexMap::<String, usize>::new();
        for c in commits {
            let author = c.author().name().unwrap_or("").to_owned();
            match committers_commits.entry(author) {
                Entry::Occupied(mut entry) => {
                    entry.insert(entry.get() + 1);
                }
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            }
        }
        committers_commits
    }

    /// The number of minor and major contributors - those contributors that together contributed 5% or less of the commits
    /// Return a tuple (minor, major)
    pub fn major_minor_contributors(&self) -> (usize, usize) {
        let authors_commits = self._authors_commits();
        let five_percent_commits = ((self.commits.len() as f64) * 0.05) as usize;
        let mut minor = 0;
        let mut major = 0;
        for (_name, commits) in authors_commits {
            if commits <= five_percent_commits {
                minor += 1;
            } else {
                major += 1;
            }
        }

        (minor, major)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_minor_major_contributors() {
        let mut authors_commits = indexmap::IndexMap::<String, usize>::new();
        authors_commits.insert("DevA".to_string(), 5);
        authors_commits.insert("DevB".to_string(), 15);
        authors_commits.insert("DevC".to_string(), 2);
        authors_commits.insert("DevD".to_string(), 20);

        let five_percent_commits = ((42 as f64) * 0.05) as usize;
        let mut minor = 0;
        let mut major = 0;
        for (_name, commits) in authors_commits {
            if commits <= five_percent_commits {
                minor += 1;
            } else {
                major += 1;
            }
        }
        assert_eq!(1, minor);
        assert_eq!(3, major)
    }
}
