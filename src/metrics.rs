use crate::{emails::EmailsMetrics, sokrates_metrics::SokratesMetrics};
use serde::Serialize;
#[derive(Clone, Debug, Default, Serialize)]

pub struct Metrics {
    /// Incubation month
    pub measurement_month: usize,
    pub window_start_date: String,
    pub window_end_date: String,
    // repo metrics
    pub commits: usize,
    pub authors: usize,
    pub committers: usize,
    pub minor_contributors: usize,
    pub major_contributors: usize,
    pub directories: usize,
    pub top_level_dirs: usize,
    pub releases: usize,

    // might want to add the commit type metrics
    // perfective: usize,
    // corrective: usize,
    // features: usize,
    // unknown: usize,
    // ....: usize,

    // process metrics
    /// The number of days in which at least one commit was recorded
    pub active_days: usize,
    /// The number of files modified (excluding added or deleted files)
    pub files_modified: usize,
    /// The number of files that were added
    pub files_added: usize,
    /// The number of files that were deleted
    pub files_deleted: usize,
    /// The number of files that were renamed
    pub files_renamed: usize,
    /// The number of added lines
    pub added_lines: usize,
    /// The number of deleted lines
    pub deleted_lines: usize,
    /// Email metrics
    #[serde(flatten)]
    pub email_metrics: EmailsMetrics,
    // /// The number of emails
    // pub emails: usize,
    // /// The number of developers involved in these emails
    // pub emails_devs: usize,
    /// The number of new contributors that have not contributed before this incubation month
    pub new_contributors: usize,
    /// The number of files that were modified per commit, on average - excludes added or deleted files
    pub avg_files_modified_commit: f64,

    // tokei metrics
    /// SLOC
    pub code: usize,
    /// The number of blank lines
    pub blanks: usize,
    /// The number of files
    pub files: usize,
    /// The number of comments
    pub comments: usize,
    /// The number of lines
    pub lines: usize,
    /// The programming language with the most code
    pub programming_lang: String,

    // sokrates metrics
    #[serde(flatten)]
    pub sokrates_metrics: SokratesMetrics,
}
