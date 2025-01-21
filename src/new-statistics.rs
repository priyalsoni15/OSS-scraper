use crate::commits_metrics::CommitsMetrics;
use crate::emails::EmailsMetrics;
use crate::metrics::Metrics;
use crate::repo::{IncubationMonth, Repo};
use crate::sokrates_metrics::{Sokrates, SokratesMetrics};
use crate::tokei_metrics::TokeiMetrics;
use crate::{utils::*, Args};
use git2::{Commit, Error};
use indexmap::IndexMap;
use serde::Serialize;
#[derive(Serialize, Debug, Default)]
pub struct Stats<'a> {
    /// Project's name
    project: &'a str,
    /// Project's start date - usually formatted as Y-m-d
    start_date: &'a str,
    /// Project's end date - usually formatted as Y-m-d
    end_date: &'a str,
    /// Project status: graduated, retired
    status: &'a str,
    /// Git repo
    // #[serde(skip_serializing)]
    // repo: &'a mut Repo<'a>,
    /// Java path
    #[serde(skip_serializing)]
    java_path: &'a str,
    /// Metrics
    #[serde(flatten)]
    metrics: Metrics,
}

impl<'a> Stats<'a> {
    pub fn new(
        project: &'a str,
        start_date: &'a str,
        end_date: &'a str,
        status: &'a str,
        java_path: &'a str,
    ) -> Self {
        Self {
            project,
            start_date,
            end_date,
            status,
            metrics: Metrics::default(),
            java_path,
            // repo,
        }
    }
    /// Compute statistics from a set of incubation-commits map
    pub fn compute_statistics_from_commits(
        &self,
        repo: &mut Repo,
        args: &Args,
    ) -> Result<Vec<Stats>, Error> {
        let inc_months_commits = if let None = args.flag_time_window {
            repo.inc_month_commits.clone()
        } else {
            repo.commits_to_inc_months_with_time_windows(
                args.flag_time_window.unwrap(),
                &repo.commits,
            )?
        };

        // let inc_months_commits = commits;
        let incubation_months_time_window = if let Some(time_window) = args.flag_time_window {
            repo.parse_date_to_inc_months_with_time_window(time_window)
        } else {
            indexmap::IndexMap::<usize, IncubationMonth>::new()
        };

        log::info!("{}", format!("{} - computing stats", self.project));
        log::info!(
            "{} - found {} commits split across {} incubation months, where an incubation month is {} days",
            self.project,
            inc_months_commits
                .values()
                .fold(0, |sum, val| sum + val.len()),
            inc_months_commits.keys().max().unwrap(),
            args.flag_time_window.unwrap_or(30)

        );

        let months = repo.dates_to_months();
        let mut last_email_metrics = EmailsMetrics {
            emails: 0,
            devs: 0,
            emails_thread_starter: 0,
            emails_thread_starter_word_count: 0,
            emails_thread_starter_characters: 0,
            emails_threads: 0,
            emails_threads_word_count: 0,
            emails_threads_characters: 0,
            emails_no_replies: 0,
            emails_no_replies_word_count: 0,
            emails_no_replies_characters: 0,
            emails_jira: 0,
        };
        let mut last_metrics = Metrics {
            active_days: 0,
            added_lines: 0,
            authors: 0,
            avg_files_modified_commit: 0.0,
            blanks: 0,
            code: 0,
            comments: 0,
            commits: 0,
            committers: 0,
            deleted_lines: 0,
            directories: 0,
            email_metrics: last_email_metrics,
            files: 0,
            files_added: 0,
            files_deleted: 0,
            files_modified: 0,
            files_renamed: 0,
            incubation_month: 1,
            lines: 0,
            major_contributors: 0,
            minor_contributors: 0,
            new_contributors: 0,
            releases: 0,
            top_level_dirs: 0,
            sokrates_metrics: SokratesMetrics::default(),
            programming_lang: "".to_string(),
        };
        let mut output: Vec<Stats> = vec![];
        let mut existing_contributors = indexmap::IndexSet::<String>::new();
        for (month, commits) in inc_months_commits.iter() {
            log::info!(
                "{}",
                format!(
                    "{} month: {} - analyzing {} commits",
                    self.project,
                    month,
                    commits.len()
                )
            );
            // we have no commits this month, we need to use the last know commits data
            if commits.is_empty() {
                last_metrics.incubation_month = *month;
                // we might not have commits, but we might have emails

                // emails check with time window and without
                // if we have months where they start in the middle of the month and
                let email_data = if let Some(_) = args.flag_time_window {
                    let incubation_month_dates = incubation_months_time_window.get(month);

                    if let Some(incubation_month_dates) = incubation_month_dates {
                        EmailsMetrics::metrics_time_window(
                            incubation_month_dates.start_date,
                            incubation_month_dates.end_date,
                            // the root path is the project's name + dev. The emails function will process the rest
                            format!("../../projects/emails/{}-dev-", self.project.to_lowercase()),
                        )
                    } else {
                        EmailsMetrics::metrics("".to_string())
                    }
                } else {
                    EmailsMetrics::metrics(format!(
                        "../../projects/emails/{}-dev-{}.mbox",
                        self.project.to_lowercase(),
                        months.get(month).unwrap()
                    ))
                };

                // process metrics are 0 because we did not have any activity
                last_metrics.active_days = 0;
                last_metrics.added_lines = 0;
                last_metrics.authors = 0;
                last_metrics.avg_files_modified_commit = 0.0;
                last_metrics.commits = 0;
                last_metrics.committers = 0;
                last_metrics.deleted_lines = 0;
                last_metrics.files_added = 0;
                last_metrics.files_deleted = 0;
                last_metrics.files_renamed = 0;
                last_metrics.files_modified = 0;
                last_metrics.major_contributors = 0;
                last_metrics.minor_contributors = 0;
                last_metrics.new_contributors = 0;
                last_metrics.releases = 0;
                last_metrics.email_metrics = email_data;

                output.push(Stats {
                    project: self.project,
                    start_date: self.start_date,
                    end_date: self.end_date,
                    status: self.status,
                    metrics: last_metrics.clone(),
                    java_path: self.java_path,
                })
            } else {
                let commit_last_month = &commits.last();
                log::debug!("{:?}", &commits);
                let month_metrics = CommitsMetrics::new(repo, &commits)?;

                // ***** COMMIT METRICS ***** //
                let active_days = month_metrics.active_days();
                let added_lines = month_metrics.added_lines();
                let authors = month_metrics.authors_emails().len();
                let commits = month_metrics.commits.len();
                let committers = month_metrics.committers_emails().len();
                let deleted_lines = month_metrics.deleted_lines();
                let files_added = month_metrics.files_added();
                let files_deleted = month_metrics.files_deleted();
                let files_renamed = month_metrics.files_renamed();
                let files_modified = month_metrics.files_modified();

                let avg_files_modified_commit = files_modified as f64 / commits as f64;

                let prev_contributors = existing_contributors
                    .iter()
                    .cloned()
                    .collect::<indexmap::IndexSet<String>>();
                let current_month_contributors = month_metrics.authors_emails();

                let contributors = current_month_contributors
                    .difference(&prev_contributors)
                    .collect::<Vec<_>>();
                let mut new_contributors = 0;
                for e in contributors {
                    new_contributors += 1;
                    existing_contributors.insert(e.to_string());
                }

                let (minor_contributors, major_contributors) =
                    month_metrics.major_minor_contributors();

                let email_data = if let Some(_) = args.flag_time_window {
                    let incubation_month_dates = incubation_months_time_window.get(month);

                    if let Some(incubation_month_dates) = incubation_month_dates {
                        EmailsMetrics::metrics_time_window(
                            incubation_month_dates.start_date,
                            incubation_month_dates.end_date,
                            // the root path is the project's name + dev. The emails function will process the rest
                            format!("../../projects/emails/{}-dev-", self.project.to_lowercase()),
                        )
                    } else {
                        EmailsMetrics::metrics("".to_string())
                    }
                } else {
                    EmailsMetrics::metrics(format!(
                        "../../projects/emails/{}-dev-{}.mbox",
                        self.project.to_lowercase(),
                        months.get(month).unwrap()
                    ))
                };

                // skip checking out at the last month's commit, to speed up the process
                if args.flag_skip_tokei {
                    let sokrates = Sokrates::new(
                        repo.repo.path().parent().unwrap().to_str().unwrap_or(""),
                        self.java_path.to_string(),
                    );
                    let current_metrics = Metrics {
                        active_days,
                        added_lines,
                        authors,
                        avg_files_modified_commit,
                        blanks: 0,
                        code: 0,
                        comments: 0,
                        commits,
                        committers,
                        deleted_lines,
                        directories: 0,
                        files: 0,
                        files_added,
                        files_deleted,
                        files_modified,
                        files_renamed,
                        incubation_month: *month,
                        lines: 0,
                        major_contributors,
                        minor_contributors,
                        new_contributors,
                        programming_lang: "".to_string(),
                        releases: 0,
                        sokrates_metrics: sokrates.metrics,
                        top_level_dirs: 0,
                        email_metrics: email_data,
                    };
                    last_metrics = current_metrics;

                    output.push(Stats {
                        project: self.project,
                        start_date: self.start_date,
                        end_date: self.end_date,
                        status: self.status,
                        metrics: last_metrics.clone(),
                        java_path: self.java_path,
                    });
                } else {
                    // we checkout at the last commit of this month as we need the source code analysis for this month
                    if let Some(hash) = commit_last_month {
                        let hash = hash.id().to_string();

                        let checkout = repo.checkout_commit(&hash);

                        if let Ok(_checkout) = checkout {
                            // ***** CODE METRICS ***** //
                            let tokei_metrics = TokeiMetrics::new(repo, &args);
                            let (code, files, lines, blanks, comments, programming_lang) =
                                match tokei_metrics {
                                    Some(tm) => (
                                        tm.code(),
                                        tm.files(),
                                        tm.lines(),
                                        tm.blanks(),
                                        tm.comments(),
                                        tm.programming_language(),
                                    ),
                                    None => (0, 0, 0, 0, 0, "".to_string()),
                                };

                            let directories =
                                directories(repo.repo.path().parent().unwrap().to_str().unwrap());

                            let top_level_dirs = top_level_directories(
                                repo.repo.path().parent().unwrap().to_str().unwrap(),
                            );

                            // ***** SOKRATES METRICS ***** //
                            let mut sokrates = Sokrates::new(
                                repo.repo.path().parent().unwrap().to_str().unwrap_or(""),
                                self.java_path.to_string(),
                            );

                            if !args.flag_skip_sokrates {
                                let history_output =
                                    sokrates.extract_history(self.project, month, &hash);

                                if history_output.is_ok() {
                                    let init_output = sokrates.init(self.project, month, &hash);
                                    // remove some basic analysis on duplication, dependencies, caching source fiels
                                    sokrates.adjust_analysis();

                                    if args.flag_restrict_languages {
                                        // change the files we're analyzing, skip duplication, and skip caching source files
                                        sokrates.adjust_files_to_be_analyzed();
                                    }

                                    if init_output.is_ok() {
                                        let reports_output =
                                            sokrates.generate_reports(self.project, month, &hash);

                                        if reports_output.is_ok() {
                                            let metrics = sokrates.metrics();
                                            if let Ok(m) = metrics {
                                                // sokrates_metrics = m;
                                                sokrates.metrics = m;
                                            }
                                        } else {
                                            log::error!(
                                            "{} month: {} - sokrates failed to generate reports",
                                            self.project,
                                            month
                                        );
                                            continue;
                                        }
                                    } else {
                                        log::error!(
                                            "{} month: {} - sokrates failed initialization",
                                            self.project,
                                            month
                                        );
                                        continue;
                                    }
                                } else {
                                    log::error!(
                                        "{} month: {} - sokrates failed to extract history",
                                        self.project,
                                        month
                                    );
                                    continue;
                                };

                                let cleanup = sokrates.cleanup(self.project, month, &hash);
                                if let Err(_cleanup) = cleanup {
                                    log::error!(
                                        "{} month: {} - cannot cleanup after sokrates",
                                        self.project,
                                        month
                                    );
                                }
                            }

                            let current_metrics = Metrics {
                                active_days,
                                added_lines,
                                authors,
                                avg_files_modified_commit,
                                blanks,
                                code,
                                comments,
                                commits,
                                committers,
                                deleted_lines,
                                directories,
                                files,
                                files_added,
                                files_deleted,
                                files_modified,
                                files_renamed,
                                incubation_month: *month,
                                lines,
                                major_contributors,
                                minor_contributors,
                                new_contributors,
                                programming_lang,
                                releases: 0,
                                sokrates_metrics: sokrates.metrics,
                                top_level_dirs,
                                email_metrics: email_data,
                            };
                            last_metrics = current_metrics;

                            output.push(Stats {
                                project: self.project,
                                start_date: self.start_date,
                                end_date: self.end_date,
                                status: self.status,
                                metrics: last_metrics.clone(),
                                // repo: self.repo,
                                java_path: self.java_path,
                            });
                        } else {
                            log::error!(
                                "{} month: {} - cannot do a checkout at hash {}",
                                self.project,
                                month,
                                hash
                            );
                            continue;
                        }
                    } else {
                        // if we cannot do a checkout, even though we know we should, we skip this month
                    }
                }
            }
        }
        // reset repository to main/master/trunk
        repo.checkout_master_main_trunk(&args)?;

        Ok(output)
    }

    /// The main function for getting the statistics
    /// Return a hashmap with incubation months as keys and metrics as values
    pub fn compute_statistics(
        &mut self,
        repo: &mut Repo,
        args: &Args,
    ) -> Result<Vec<Stats>, Error> {
        //Result<IndexMap<usize, Metrics>, Error> {
        log::info!("{}", format!("{} - computing stats", self.project));

        let _projects_names_fix_emails = indexmap::IndexMap::from([
            ("apex-core", "apex"),
            ("blur", "incubator-blur"),
            ("derby", "db-derby"),
            ("empire-db", "empire"),
            ("ftpserver", "incubator-ftpserver"),
            ("hcatalog", "incubator-hcatalog"),
            ("ant-ivy", "incubator-ivy"),
            ("kalumet", "incubator-kalumet"),
            ("lucene.net", "lucenenet"),
            ("mynewt-core", "mynewt"),
            ("npanday", "incubator-npanday"),
            ("nuvem", "incubator-nuvem"),
            ("odftoolkit", "incubator-odf"),
            ("photark", "incubator-photark"),
            ("pluto", "portals-pluto"),
            ("creadur-rat", "creadur"),
            ("s4", "incubator-s4"),
            ("sanselan", "incubator-sanselan"),
            ("servicecomb-java-chassis", "servicecomb"),
            ("tashi", "incubator-tashi"),
            ("warble-server", "warble"),
            ("wave", "incubator-wave"),
            ("zetacomponents", "incubator-zeta"),
        ]);

        // if let Some(time_window) = args.flag_time_window {
        //     let commits =
        //         repo.commits_to_inc_months_with_time_windows(time_window, &repo.commits)?;
        //     // for c in commits {
        //     //     println!("Month: {:?}", c.0);
        //     //     println!("Commits: {:?}", c.1);
        //     // }
        //     self.compute_statistics_from_commits(repo, &commits, args)
        // } else {
        // let commits = &repo.inc_month_commits;
        self.compute_statistics_from_commits(repo, args)
        // }

        // let inc_months_commits = ;
    }
}
