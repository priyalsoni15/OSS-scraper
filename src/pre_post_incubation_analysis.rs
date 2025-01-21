use crate::{
    project::Project, remove_sokrates_temp, repo::Repo,
    statistics::Stats, utils::convert_time, Args,
};
use chrono::{DateTime, Utc};
use git2::Repository;
use log::error;
use rayon::iter::{ParallelBridge, ParallelIterator};

pub fn run(
    repo: &mut Repo,
    java_path: &str,
    args: &Args,
    data_folder_path: &str,
    p: &Project,
    analysis_name: &str,
) {
    remove_sokrates_temp(&repo.repo);
    let checkout = repo.checkout_master_main_trunk(&args);
    if let Ok(_checkout) = checkout {
        let mut stats = Stats::new(
            p.name.as_str(),
            &repo.start_date,
            &repo.end_date,
            &p.status,
            &java_path,
        );
        let metrics = stats.compute_statistics(repo, &args);
        if let Ok(metrics) = metrics {
            let mut writer = csv::WriterBuilder::default()
                .has_headers(true)
                .from_path(format!(
                    "{}/{}-{}.csv",
                    data_folder_path,
                    p.name.as_str(),
                    analysis_name
                ))
                .unwrap();

            for m in metrics {
                match writer.serialize(m) {
                    Ok(()) => {}
                    Err(_e) => {
                        error!("{} - cannot serialize metric value", p.name.as_str());
                    }
                }
            }
        } else {
            error!("{} cannot extract the metrics", p.name.as_str());
        }
    } else {
        error!("{} - cannot reset to main/master/trunk", p.name.as_str());
    }
}

///
/// This is the analysis for pre, during, and post incubation of projects.
/// The analysis considers only projects that had commits prior to joining
/// the incubator and after exiting the incubator
///
pub fn pre_post_analysis(
    projects: indexmap::IndexSet<Project>,
    args: &Args,
    java_path: &str,
    data_folder_path: &str,
) {
    let mut do_not_analyze = indexmap::IndexSet::<String>::new();

    do_not_analyze.insert("Cloudstack".to_string());
    do_not_analyze.insert("ODFToolkit".to_string());

    let projects_to_analyze = projects
        .iter()
        .filter(|x| !do_not_analyze.contains(&x.name))
        .collect::<indexmap::IndexSet<_>>();

    // let mut projects_to_analyze: Vec<&Project> = vec![];
    projects_to_analyze.iter().par_bridge().for_each(|p| {
        let git_repo = Repository::open(p.path.as_str());

        if let Ok(repo) = git_repo {
            #[allow(clippy::unwrap_used)]
            let start_timestamp = chrono::DateTime::parse_from_rfc3339(
                format!("{}{}", p.start_date, "T00:00:00+00:00").as_str(),
            )
            .unwrap()
            .timestamp();
            #[allow(clippy::unwrap_used)]
            let final_timestamp = chrono::DateTime::parse_from_rfc3339(
                format!("{}{}", p.end_date, "T23:59:59+00:00").as_str(),
            )
            .unwrap()
            .timestamp();

            let revwalk = repo.revwalk();
            let mut has_prior_commits = false;
            let mut first_commit_time = "".to_string();
            if let Ok(mut revwalk) = revwalk {
                // Prepare the revwalk based on CLI parameters

                revwalk.set_sorting(git2::Sort::REVERSE);
                revwalk.push_head();
                let first_commit_id = revwalk.nth(0);
                if let Some(id) = first_commit_id {
                    let commit = repo.find_commit(id.unwrap());

                    if let Ok(commit) = commit {
                        let commit_time = convert_time(&commit.committer().when());
                        first_commit_time = commit_time.format("%Y-%m-%d").to_string();

                        if commit_time.timestamp() < start_timestamp {
                            has_prior_commits = true;
                            log::info!(
                                "Prior commits: {}, {}, {}, {},",
                                p.name,
                                p.status,
                                first_commit_time,
                                p.path
                            );
                        }
                    }
                }
            }
            if has_prior_commits {
                let revwalk = repo.revwalk();

                if let Ok(mut revwalk) = revwalk {
                    revwalk.set_sorting(git2::Sort::NONE);

                    // Prepare the revwalk based on CLI parameters
                    revwalk.push_head();
                    let last_commit_id = revwalk.nth(0);
                    if let Some(id) = last_commit_id {
                        let commit = repo.find_commit(id.unwrap());

                        if let Ok(commit) = commit {
                            let commit_time = convert_time(&commit.committer().when());

                            log::info!(
                                "Post commits {}, {}, {}, {}",
                                p.status,
                                commit_time.to_rfc3339(),
                                p.name,
                                p.path
                            );

                            analyze_pre_incubation(&repo, p, first_commit_time, args, java_path, data_folder_path);
                            analyze_during_incubation(&repo, p, args, java_path, data_folder_path);
                            analyze_post_incubation(&repo, commit_time, final_timestamp, p, args, java_path, data_folder_path);

                        }
                    }
                }
            } else {
                if args.flag_force_full_analysis {
                    // find the last commit in the repository - that is our final end period, as we likely only have during and post
                    
                    let revwalk = repo.revwalk();

                    if let Ok(mut revwalk) = revwalk {
                        revwalk.set_sorting(git2::Sort::NONE);

                        // Prepare the revwalk based on CLI parameters
                        revwalk.push_head();
                        let last_commit_id = revwalk.nth(0);
                        if let Some(id) = last_commit_id {
                            let commit = repo.find_commit(id.unwrap());

                            if let Ok(commit) = commit {
                                let commit_time = convert_time(&commit.committer().when());

                                analyze_during_incubation(&repo, p, args, java_path, data_folder_path);
                                analyze_post_incubation(&repo, commit_time, final_timestamp, p, args, java_path, data_folder_path)
                            }
                        }
                    }
                    
                }
            }
        }
    });
}

fn analyze_pre_incubation(repo: &git2::Repository, p: &Project, first_commit_time: String, args: &Args, java_path: &str, data_folder_path: &str) {
    let pre_incubation_end_date =
    chrono::NaiveDate::parse_from_str(&p.start_date, "%Y-%m-%d")
        .unwrap()
        .pred();
    let pre_incub_end_date_str = pre_incubation_end_date.to_string();

    let mut pre_repo = Repo::new(
        &repo,
        p.name.as_str(),
        &first_commit_time,
        pre_incub_end_date_str.as_str(),
        //p.start_date.as_str(),
        p.status.as_str(),
        &args,
    )
    .unwrap();
    log::info!(
        "Analyzing pre incubation: {:?}, {:?}, {:?}, {:?}",
        pre_repo.project,
        pre_repo.start_date,
        pre_repo.end_date,
        pre_repo.status
    );

    run(
        &mut pre_repo,
        java_path,
        args,
        data_folder_path,
        p,
        "pre-incubation",
    );

}

fn analyze_during_incubation(repo: &git2::Repository, p: &Project, args: &Args, java_path: &str, data_folder_path: &str) {
    let mut incubation_repo = Repo::new(
        &repo,
        p.name.as_str(),
        p.start_date.as_str(),
        p.end_date.as_str(),
        p.status.as_str(),
        &args,
    )
    .unwrap();
    log::info!(
        "Analyzing during incubation: {:?}, {:?}, {:?}, {:?}",
        incubation_repo.project,
        incubation_repo.start_date,
        incubation_repo.end_date,
        incubation_repo.status
    );
    run(
        &mut incubation_repo,
        java_path,
        args,
        data_folder_path,
        p,
        "during-incubation",
    );
}

fn analyze_post_incubation(repo: &git2::Repository,  commit_time: DateTime<Utc>, final_timestamp: i64, p: &Project, args: &Args, java_path: &str, data_folder_path: &str) {
    if commit_time.timestamp() > final_timestamp {
        let time = commit_time.clone().format("%Y-%m-%d").to_string();

        let post_incubation_start_date =
            chrono::NaiveDate::parse_from_str(&p.end_date, "%Y-%m-%d")
                .unwrap()
                .succ();
        let post_incubation_start_date_str =
            post_incubation_start_date.to_string();
        let mut post_incubation_repo = Repo::new(
            &repo,
            p.name.as_str(),
            post_incubation_start_date_str.as_str(),
            &time,
            p.status.as_str(),
            &args,
        )
        .unwrap();
        // log::info!("{} - analyzing post incubation", p.name);
        log::info!(
            "Analyzing post incubation: {:?}, {:?}, {:?}, {:?}",
            post_incubation_repo.project,
            post_incubation_repo.start_date,
            post_incubation_repo.end_date,
            post_incubation_repo.status
        );
        run(
            &mut post_incubation_repo,
            java_path,
            args,
            data_folder_path,
            p,
            "post-incubation",
        );
    }
}
