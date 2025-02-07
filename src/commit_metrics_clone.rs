use git2::Repository;
use std::error::Error;
use std::fs;
use tempfile::TempDir;
use crate::repo::Repo;
use crate::dev_stats::DevStats;
use crate::Args;
use std::sync::{Arc, RwLock};
use log::info;
use chrono::{Utc, DateTime};

pub fn analyze_online_repo(
    online_url: &str,
    args: &Args,
    status: &str,
) -> Result<(), Box<dyn Error>> {
    // Create a temporary directory to clone the repository.
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path().join("repo");
    info!("Cloning repository {} into {:?}", online_url, temp_path);
    let repo = Repository::clone(online_url, &temp_path)?;
    
    // Derive project name from the URL (take last path segment, remove ".git")
    let parts: Vec<&str> = online_url.split('/').collect();
    let repo_name = parts.last().unwrap_or(&"unknown").trim_end_matches(".git").to_string();

    // Create a mutable Repo object using your existing logic.
    let mut repo_obj = Repo::new(&repo, &repo_name, "", "", status, args)?;
    // Checkout the proper branch (master/main/trunk)
    repo_obj.checkout_master_main_trunk(args)?;

    // Compute commit metrics via DevStats
    let java_path = crate::java_path();
    let dev_stats = DevStats::new(&repo_name, &repo_obj, &java_path);
    let metrics = dev_stats.compute_individual_dev_stats(args)?;

    // Determine dynamic start and end dates based on first and last commit
    let start_date = metrics.first().map(|m| m.date.clone()).unwrap_or_else(|| Utc::now().format("%Y-%m-%d").to_string());
    let end_date = metrics.last().map(|m| m.date.clone()).unwrap_or_else(|| Utc::now().format("%Y-%m-%d").to_string());

    // Write the CSV output to the output folder.
    let output_folder = args.flag_output_folder.as_deref().unwrap_or("data");
    fs::create_dir_all(output_folder)?;
    let csv_path = format!("{}/{}-commit-file-dev.csv", output_folder, repo_name);
    let writer = Arc::new(RwLock::new(csv::WriterBuilder::default().has_headers(true).from_path(&csv_path)?));
    {
        let mut guard = writer.write().expect("Unable to lock writer");
        for m in metrics {
            guard.serialize(m)?;
        }
    }
    info!("Clone-based commit analysis completed for repository {}", repo_name);
    // The TempDir is automatically removed when it goes out of scope.
    Ok(())
}