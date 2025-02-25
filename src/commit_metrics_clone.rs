// commit_metrics_clone.rs
use git2::Repository;
use std::error::Error;
use std::fs;
use tempfile::TempDir;
use crate::repo::Repo;
use crate::dev_stats::DevStats;
use crate::Args;
use std::sync::{Arc, RwLock};
use log::{info, error};

pub fn analyze_online_repo(
    online_url: &str,
    args: &Args,
    start_date: &str,
    end_date: &str,
    status: &str,
) -> Result<(), Box<dyn Error>> {
    // 1) Create a temporary directory to clone the repository.
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path().join("repo");
    info!("Cloning repository {} into {:?}", online_url, temp_path);
    let repo = Repository::clone(online_url, &temp_path)?;

    // 2) Derive project name & owner from the URL
    //    e.g. https://github.com/apache/hunter.git => owner = "apache", repo_name = "hunter"
    let url_no_dotgit = online_url.trim_end_matches(".git");
    let parts: Vec<&str> = url_no_dotgit.split('/').collect();
    let repo_name = parts.last().unwrap_or(&"unknown").to_string();
    let owner = if parts.len() >= 2 {
        parts[parts.len() - 2]
    } else {
        "unknown"
    };

    // 3) Create a mutable Repo object using your existing logic.
    let mut repo_obj = Repo::new(&repo, &repo_name, start_date, end_date, status, args)?;
    //    Checkout the proper branch (master/main/trunk).
    repo_obj.checkout_master_main_trunk(args)?;

    // 4) Compute commit metrics via DevStats
    let java_path = crate::java_path();
    let dev_stats = DevStats::new(&repo_name, &repo_obj, &java_path);
    let mut stats_output = dev_stats.compute_individual_dev_stats(args)?;

    // 5) For each commit row, fill in the commit_url
    for ds in stats_output.iter_mut() {
        let sha = &ds.metrics.commit_sha;
        ds.metrics.commit_url = format!(
            "https://github.com/{}/{}/commit/{}",
            owner, repo_name, sha
        );
    }

    // 6) Write the CSV output to the output folder.
    let output_folder = args.flag_output_folder.as_deref().unwrap_or("data");
    fs::create_dir_all(output_folder)?;
    let csv_path = format!("{}/{}-commit-file-dev.csv", output_folder, repo_name);

    let writer = Arc::new(RwLock::new(csv::WriterBuilder::default().has_headers(true).from_path(&csv_path)?));
    {
        let mut guard = writer.write().expect("Unable to lock CSV writer");
        // We can directly serialize the `DevStats` rows now (which embed `commit_url`)
        for ds in &stats_output {
            if let Err(e) = guard.serialize(ds) {
                error!("Cannot serialize a dev_stats row: {}", e);
            }
        }
    }

    info!("Clone-based commit analysis completed for repository {}. CSV => {}", repo_name, csv_path);
    // The TempDir is automatically removed when it goes out of scope.

    Ok(())
}
