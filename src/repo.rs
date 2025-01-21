use crate::{convert_time, utils, Args};
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use git2::{Commit, DiffOptions, Error, Repository};
use indexmap::map::Entry;
use indexmap::{IndexMap, IndexSet};
use serde::Serialize;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IncubationMonth {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub incubation_month: usize,
}

#[derive(Serialize, PartialEq, Eq, Hash, Debug)]
pub struct CommitMessage {
    pub project: String,
    pub status: String,
    pub inc_month: usize,
    pub sha: String,
    pub message: String,
}

pub struct Repo<'a> {
    pub repo: &'a Repository,
    pub project: &'a str,
    pub start_date: &'a str,
    pub end_date: &'a str,
    pub status: &'a str,
    pub commits: Vec<Commit<'a>>,
    pub inc_month_commits: IndexMap<usize, Vec<Commit<'a>>>,
}

impl<'a> Repo<'a> {
    fn string_to_static_str(s: String) -> &'static str {
        Box::leak(s.into_boxed_str())
    }
    /// Create a new repository object with all the metadata
    pub fn new(
        repo: &'a Repository,
        project: &'a str,
        start_date: &'a str,
        end_date: &'a str,
        status: &'a str,
        args: &'a Args,
    ) -> Result<Self, Error> {
        let (start, end) = if args.flag_ignore_start_end_dates {
            let first_commit_date = Self::find_first_commit_timestamp(repo);
            let last_commit_date = Self::find_last_commit_timestamp(repo);
            log::info!("{}: Ignore start end dates option enabled. First commit timestamp: {}. Last commit timestamp: {}", project, first_commit_date, last_commit_date);
            (first_commit_date, last_commit_date)
        } else {
            (start_date.to_string(), end_date.to_string())
        };

        let commits = Self::commits(repo, &start, &end, args)?;
        let inc_month_commits = Self::commits_to_inc_months(&start, &end, &commits)?;
        let start_date = Self::string_to_static_str(start);
        let end_date = Self::string_to_static_str(end);
        Ok(Self {
            repo,
            project,
            start_date: start_date,
            end_date: end_date,
            status,
            commits,
            inc_month_commits,
        })
    }

    fn find_first_commit_timestamp(repo: &'a Repository) -> String {
        // start from the top

        let revwalk = repo.revwalk();
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
                    let time = commit_time.format("%Y-%m-%d").to_string();
                    first_commit_time = time.clone();
                }
            }
        }
        first_commit_time
    }

    fn find_last_commit_timestamp(repo: &'a Repository) -> String {
        let revwalk = repo.revwalk();
        let mut last_commit_time = "".to_string();
        if let Ok(mut revwalk) = revwalk {
            // Prepare the revwalk based on CLI parameters
            revwalk.set_sorting(git2::Sort::NONE);
            revwalk.push_head();
            let last_commit_id = revwalk.nth(0);
            if let Some(id) = last_commit_id {
                let commit = repo.find_commit(id.unwrap());

                if let Ok(commit) = commit {
                    let commit_time = convert_time(&commit.committer().when());
                    let time = commit_time.format("%Y-%m-%d").to_string();
                    last_commit_time = time.clone();
                }
            }
        }
        last_commit_time
    }

    /// Retrives commits from the repository between project's start date and project's end date, excluding merge commits
    fn commits(
        repo: &'a Repository,
        _start_date: &str,
        _end_date: &str,
        args: &Args,
    ) -> Result<Vec<Commit<'a>>, Error> {
        let start_date = if _start_date.is_empty() {
            "1970-01-01"
        } else {
            _start_date
        };

        let end_date = if _end_date.is_empty() {
            "2100-01-01"
        } else {
            _end_date
        };

        // starting timestamp for dropping commits previous to incubation start
        #[allow(clippy::unwrap_used)]
        let final_timestamp = if let Ok(d) = chrono::DateTime::parse_from_rfc3339(
            format!("{}{}", end_date, "T23:59:59+00:00").as_str(),
        ) {
            d.timestamp()
        } else {
            log::error!(
                "{:?}: Cannot convert finaltimestamp for extracting commits. End date is: {}",
                repo.path(),
                end_date
            );
            4102473600
        };

        // ending timestamp for dropping commits after incubation ended
        #[allow(clippy::unwrap_used)]
        let start_timestamp = if let Ok(d) = chrono::DateTime::parse_from_rfc3339(
            format!("{}{}", start_date, "T00:00:00+00:00").as_str(),
        ) {
            d.timestamp()
        } else {
            log::error!(
                "{:?}: Cannot convert start timestamp for extracting commits. Date is {}",
                repo.path(),
                start_date
            );
            28800
        };

        let mut revwalk = repo.revwalk()?;
        // start from the top
        let reverse_sorting = revwalk.set_sorting(git2::Sort::REVERSE);
        if let Err(_reverse_sorting) = reverse_sorting {
            log::error!("cannot iterate commits in reverse order")
        }
        let mut first_commit = true;

        revwalk.push_head()?;
        let commits: Vec<Commit<'a>> = revwalk
            .filter_map(|r| {
                match r {
                    Err(_) => None,
                    Ok(r) => repo.find_commit(r).ok().filter(|commit| {
                        // exclude merge commits
                        let commit_time = convert_time(&commit.committer().when()).timestamp();
                        // if this is first commit, then we keep it as it might not have parents
                        if first_commit {
                            first_commit = false;
                            (commit.parent_count() == 0
                                || commit.parent_count() == 1) && commit_time >= start_timestamp //drop any commits that are before project start
                            && commit_time <= final_timestamp // drop any commits that are after project start
                        } else {
                            commit.parent_count() == 1 // drop any merge commits
                            && commit_time >= start_timestamp //drop any commits that are before project start
                            && commit_time <= final_timestamp // drop any commits that are after project start
                        }
                    }),
                }
            })
            .collect();

        if args.flag_restrict_languages {
            let (_diffopts, mut diffopts2) = (DiffOptions::new(), DiffOptions::new());

            let extensions = utils::find_lang_extensions().unwrap();
            let mut filtered_commits = vec![];

            commits.iter().for_each(|c| {
                let a = if c.parents().len() == 1 {
                    let parent = c.parent(0).ok();
                    if parent.is_some() {
                        parent.unwrap().tree().ok()
                    } else {
                        None
                    }
                } else {
                    None
                };
                let b = c.tree().ok();
                if b.is_some() {
                    let diff = repo
                        .diff_tree_to_tree(a.as_ref(), Some(&b.unwrap()), Some(&mut diffopts2))
                        .ok();
                    let diff_data = crate::commits_metrics::DiffData::new();
                    if diff.is_some() {
                        let parsed_diff =
                            diff_data.parse_diff_restricted_langs(&diff.unwrap(), &extensions);
                        if parsed_diff.is_some() {
                            filtered_commits.push(c.clone());
                        }
                    }
                }
            });

            Ok(filtered_commits)
        } else {
            Ok(commits)
        }
    }

    pub fn commits_to_inc_months_with_time_windows(
        &self,
        time_window: i64,
        commits: &Vec<Commit<'a>>,
    ) -> Result<IndexMap<usize, Vec<Commit<'a>>>, Error> {
        let incubation_months = self.parse_date_to_inc_months_with_time_window(time_window);

        let mut output = IndexMap::<usize, Vec<Commit<'a>>>::new();

        // we add all our incubation months with no commits yet to the output
        for (incubation_month, _) in &incubation_months {
            output.insert(*incubation_month, vec![]);
        }

        let all_commits = commits.clone();

        for commit in all_commits {
            let commit_time = NaiveDateTime::from_timestamp(commit.committer().when().seconds(), 0);
            let commit_date =
                NaiveDate::from_ymd(commit_time.year(), commit_time.month(), commit_time.day());
            let mut commit_inc_month = None;

            for (month, inc_month_type) in &incubation_months {
                if inc_month_type.start_date <= commit_date
                    && commit_date <= inc_month_type.end_date
                {
                    commit_inc_month = Some(*month)
                }
            }

            // if we find commits in a particular incubation month, then we add it to our output list
            match output.entry(commit_inc_month.unwrap()) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(commit);
                }
                Entry::Vacant(entry) => {
                    entry.insert(vec![commit]);
                }
            }
        }
        Ok(output)
    }

    /// Returns an index map of commits per incubation month; the key is the incubation month as an integer
    /// and the value is the vector of commits for that respective incubation month
    fn commits_to_inc_months(
        start_date: &str,
        end_date: &str,
        commits: &Vec<Commit<'a>>,
    ) -> Result<IndexMap<usize, Vec<Commit<'a>>>, Error> {
        let incubation_months = Self::parse_dates_to_inc_months(start_date, end_date);
        // println!("{:?}", incubation_months);
        let mut output = IndexMap::<usize, Vec<Commit<'a>>>::new();

        // we add all our incubation months with no commits yet to the output
        for (_, v) in &incubation_months {
            output.insert(*v, vec![]);
        }

        let all_commits = commits.clone();

        for commit in all_commits {
            let commit_time = convert_time(&commit.committer().when());
            let commit_inc_month = incubation_months
                .get(format!("{}{}", commit_time.year(), commit_time.month()).as_str());

            // if we find commits in a particular incubation month, then we add it to our output list
            match output.entry(*commit_inc_month.unwrap()) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(commit);
                }
                Entry::Vacant(entry) => {
                    entry.insert(vec![commit]);
                }
            }
        }
        Ok(output)
    }

    /// Checkout the repository at the given commit
    pub fn checkout_commit(&self, refname: &str) -> Result<(), Error> {
        let revparse = self.repo.revparse_ext(refname);

        if let Ok((object, reference)) = revparse {
            let try_checkout = self.repo.checkout_tree(&object, None);
            if let Ok(_checkout) = try_checkout {
                log::info!("{} - succesfully checked out at {}", self.project, &refname);
                match reference {
                    // gref is an actual reference like branches or tags
                    Some(gref) => self.repo.set_head(gref.name().unwrap())?,
                    // this is a commit, not a reference
                    None => self.repo.set_head_detached(object.id())?,
                }
            } else {
                log::error!("{} - failed to checkout at {}", self.project, &refname);
                return Err(try_checkout.err().unwrap());
            }
        } else {
            log::error!("{} - failed to checkout at {}", self.project, &refname);
            return Err(revparse.err().unwrap());
        }

        Ok(())
    }

    /// Try to checkout the repository to its main branch (master, main, trunk)
    pub fn checkout_master_main_trunk(&mut self, args: &Args) -> Result<(), Error> {
        let repo_branches = self.repo.branches(Some(git2::BranchType::Local));

        let repo_branches = if let Ok(branches) = repo_branches {
            branches
                .into_iter()
                .filter_map(|x| {
                    if let Ok(x) = x {
                        if let Ok(name) = x.0.name() {
                            if let Some(name) = name {
                                Some(name.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<IndexSet<String>>()
        } else {
            IndexSet::<String>::new()
        };
        log::info!(
            "{} - found the following branches {}",
            self.project,
            &repo_branches
                .clone()
                .into_iter()
                .collect::<Vec<_>>()
                .join(",")
        );

        if self.project == "FreeMarker" {
            match self.checkout_commit("2.3-gae") {
                Ok(()) => {
                    self.update_repo_state_after_checkout(args)?;
                    return Ok(());
                }
                Err(_e) => {
                    log::error!(
                        "{} - has a branch named 2.3-gae, but I cannot check it out",
                        self.project
                    );
                    return Ok(());
                }
            };
        } else if self.project == "Dubbo" {
            match self.checkout_commit("3.0") {
                Ok(()) => {
                    self.update_repo_state_after_checkout(args)?;
                    return Ok(());
                }
                Err(_e) => {
                    log::error!(
                        "{} - has a branch named 3.0, but I cannot check it out",
                        self.project
                    );
                    return Ok(());
                }
            };
        } else if self.project == "DolphinScheduler" {
            match self.checkout_commit("dev") {
                Ok(()) => {
                    self.update_repo_state_after_checkout(args)?;
                    return Ok(());
                }
                Err(_e) => {
                    log::error!(
                        "{} - has a branch named dev, but I cannot check it out",
                        self.project
                    );
                    return Ok(());
                }
            };
        } else if repo_branches.contains("master") {
            match self.checkout_commit("master") {
                Ok(()) => {
                    self.update_repo_state_after_checkout(args)?;
                    return Ok(());
                }
                Err(_e) => {
                    log::error!(
                        "{} - has a branch named master, but I cannot check it out",
                        self.project
                    );
                    return Ok(());
                }
            };
        } else if repo_branches.contains("main") {
            match self.checkout_commit("main") {
                Ok(()) => {
                    self.update_repo_state_after_checkout(args)?;
                    return Ok(());
                }
                Err(_e) => {
                    log::error!(
                        "{} - has a branch named main, but I cannot check it out",
                        self.project
                    );
                    return Ok(());
                }
            };
        } else if repo_branches.contains("trunk") {
            match self.checkout_commit("trunk") {
                Ok(()) => {
                    self.update_repo_state_after_checkout(args)?;
                    return Ok(());
                }
                Err(_e) => {
                    log::error!(
                        "{} - has a branch named main, but I cannot check it out",
                        self.project
                    );
                    return Ok(());
                }
            };
        } else if repo_branches.contains("develop") {
            match self.checkout_commit("develop") {
                Ok(()) => {
                    self.update_repo_state_after_checkout(args)?;
                    return Ok(());
                }
                Err(_e) => {
                    log::error!(
                        "{} - has a branch named develop, but I cannot check it out",
                        self.project
                    );
                    return Ok(());
                }
            };
        } else {
            log::error!(
                "{} - has no branch named master/main/trunk/develop... cannot reset to main branch",
                self.project
            );
        }

        Ok(())
    }

    fn update_repo_state_after_checkout(&mut self, args: &Args) -> Result<(), Error> {
        let (start, end) = if args.flag_ignore_start_end_dates {
            let first_commit_date = Self::find_first_commit_timestamp(self.repo);
            let last_commit_date = Self::find_last_commit_timestamp(self.repo);
            (first_commit_date, last_commit_date)
        } else {
            (self.start_date.to_string(), self.end_date.to_string())
        };

        self.commits = Self::commits(self.repo, &start, &end, args)?;
        self.inc_month_commits = Self::commits_to_inc_months(&start, &end, &self.commits)?;
        Ok(())
    }
    /// Transform the start date and end date into a map of incubation months and date
    ///
    /// E.g., 2010-01-01 to 2010-03-05 => 1 -> 201001, 2 -> 201002, 3 -> 201003
    pub fn dates_to_months(&self) -> IndexMap<usize, String> {
        let mut result = IndexMap::<usize, String>::new();

        let sd = chrono::NaiveDate::parse_from_str(self.start_date, "%Y-%m-%d").unwrap();
        let ed = chrono::NaiveDate::parse_from_str(self.end_date, "%Y-%m-%d").unwrap();

        fn next_month(year: i32, month: u32) -> (i32, u32) {
            assert!(month >= 1 && month <= 12);

            if month == 12 {
                (year + 1, 1)
            } else {
                (year, month + 1)
            }
        }

        let mut year = sd.year();
        let mut month = sd.month();
        let mut inc_month = 0;

        loop {
            let (next_year, next_month) = next_month(year, month);

            if year < ed.year() || (year == ed.year() && month <= ed.month()) {
                inc_month += 1;
                if month < 10 {
                    result.insert(inc_month, format!("{}0{}", year, month));
                } else {
                    result.insert(inc_month, format!("{}{}", year, month));
                }

                year = next_year;
                month = next_month;
            } else {
                break;
            }
        }
        result
    }

    /// Parses the incubation start and end dates to a list of incubation months,
    /// where the time window defines the number of days for each incubation month
    /// The returned data is a hash map with the incubation month as keys. The date reflects the
    /// last date of the incubation month. Values are the incubation month index
    /// For example: time window = 30; start date = 2010-01-01, end_date = 2010-03-05
    /// The result will be: 1 -> 2010-01-01, 2010-01-30, 2 -> 2010-01-31 - 2010-03-01, 3 -> 2010-03-02 - 2010-03-05, .
    /// This is an ordered HashMap
    pub fn parse_date_to_inc_months_with_time_window(
        &self,
        time_window: i64,
    ) -> IndexMap<usize, IncubationMonth> {
        utils::parse_date_to_inc_months_with_time_window(
            self.start_date,
            self.end_date,
            time_window,
        )
    }

    /// Parses the incubation start and end dates to a list of incubation months.
    /// The returned data is a hash map with the date as 20101 - Jan 2010, as keys
    /// and integers (incubation month) as values
    /// This is a ordered HashMap
    pub fn parse_dates_to_inc_months(start_date: &str, end_date: &str) -> IndexMap<String, usize> {
        let mut result = IndexMap::<String, usize>::new();

        let sd = chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d").unwrap();
        let ed = chrono::NaiveDate::parse_from_str(end_date, "%Y-%m-%d").unwrap();

        fn next_month(year: i32, month: u32) -> (i32, u32) {
            assert!(month >= 1 && month <= 12);

            if month == 12 {
                (year + 1, 1)
            } else {
                (year, month + 1)
            }
        }

        let mut year = sd.year();
        let mut month = sd.month();
        let mut inc_month = 0;

        loop {
            let (next_year, next_month) = next_month(year, month);

            if year < ed.year() || (year == ed.year() && month <= ed.month()) {
                inc_month += 1;
                result.insert(format!("{}{}", year, month), inc_month);

                year = next_year;
                month = next_month;
            } else {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use crate::{repo::IncubationMonth, Args};
    use structopt::StructOpt;

    use super::{Repo, Repository};

    #[test]
    fn test_parse_date_to_inc_months() {
        let months = Repo::parse_dates_to_inc_months("2010-10-30", "2010-10-31");
        assert!(months.contains_key("201010"));
        assert!(!months.contains_key("201011"));

        assert_eq!(Some(&1), months.get("201010"));
        assert_eq!(None, months.get("201011"));

        let months = Repo::parse_dates_to_inc_months("2010-10-30", "2011-01-30");
        println!("{:?}", months);
        assert!(months.contains_key("201010"));
        assert!(months.contains_key("201011"));
        assert!(months.contains_key("20111"));
        assert!(!months.contains_key("20112"));
        assert!(!months.contains_key("20102"));

        assert_eq!(Some(&1), months.get("201010"));
        assert_eq!(Some(&2), months.get("201011"));
        assert_eq!(Some(&4), months.get("20111"));
    }

    #[test]
    fn test_dates_to_months() {
        let args = Args::from_iter(&["threads=1"]);

        let repo = Repository::open("test_resources/test_repo/.my_git_repo").unwrap();
        let actual = Repo::new(
            &repo,
            "test",
            "2010-01-01",
            "2010-03-05",
            "graduated",
            &args,
        );

        if let Ok(actual) = actual {
            assert_eq!(
                actual.dates_to_months(),
                indexmap::IndexMap::from([(1, "201001"), (2, "201002"), (3, "201003")])
            );
        } else {
            assert!(false)
        }

        let actual = Repo::new(
            &repo,
            "test",
            "2010-12-30",
            "2011-01-05",
            "graduated",
            &args,
        );

        if let Ok(actual) = actual {
            assert_eq!(
                actual.dates_to_months(),
                indexmap::IndexMap::from([(1, "201012"), (2, "201101")])
            );
        } else {
            assert!(false)
        }
    }

    #[test]
    fn test_parse_date_to_inc_months_with_time_window() {
        let args = Args::from_iter(&["threads=1"]);

        let start_date = "2022-01-01";
        let end_date = "2022-02-15";
        let git_repo = Repository::open("test_resources/test_repo/.my_git_repo").unwrap();
        let repository = Repo::new(&git_repo, "test", start_date, end_date, "graduated", &args);
        let inc_months = repository
            .unwrap()
            .parse_date_to_inc_months_with_time_window(10);
        println!("{:?}", inc_months);
        let mut expected = indexmap::IndexMap::<usize, IncubationMonth>::new();
        expected.insert(
            1,
            IncubationMonth {
                start_date: chrono::NaiveDate::from_ymd(2022, 01, 01),
                end_date: chrono::NaiveDate::from_ymd(2022, 01, 10),
                incubation_month: 1,
            },
        );
        expected.insert(
            2,
            IncubationMonth {
                start_date: chrono::NaiveDate::from_ymd(2022, 01, 11),
                end_date: chrono::NaiveDate::from_ymd(2022, 01, 20),
                incubation_month: 2,
            },
        );
        expected.insert(
            3,
            IncubationMonth {
                start_date: chrono::NaiveDate::from_ymd(2022, 01, 21),
                end_date: chrono::NaiveDate::from_ymd(2022, 01, 30),
                incubation_month: 3,
            },
        );
        expected.insert(
            4,
            IncubationMonth {
                start_date: chrono::NaiveDate::from_ymd(2022, 01, 31),
                end_date: chrono::NaiveDate::from_ymd(2022, 02, 09),
                incubation_month: 4,
            },
        );
        expected.insert(
            5,
            IncubationMonth {
                start_date: chrono::NaiveDate::from_ymd(2022, 02, 10),
                end_date: chrono::NaiveDate::from_ymd(2022, 02, 15),
                incubation_month: 5,
            },
        );
        assert_eq!(expected, inc_months);

        let start_date = "2022-01-01";
        let end_date = "2022-01-08";
        let repository =
            Repo::new(&git_repo, "test", start_date, end_date, "graduated", &args).unwrap();
        let inc_months = repository.parse_date_to_inc_months_with_time_window(10);
        expected.clear();
        expected.insert(
            1,
            IncubationMonth {
                start_date: chrono::NaiveDate::from_ymd(2022, 01, 01),
                end_date: chrono::NaiveDate::from_ymd(2022, 01, 08),
                incubation_month: 1,
            },
        );
        assert_eq!(expected, inc_months);

        let start_date = "2022-01-01";
        let end_date = "2022-01-10";
        let repository =
            Repo::new(&git_repo, "test", start_date, end_date, "graduated", &args).unwrap();
        let inc_months = repository.parse_date_to_inc_months_with_time_window(10);
        expected.clear();
        expected.insert(
            1,
            IncubationMonth {
                start_date: chrono::NaiveDate::from_ymd(2022, 01, 01),
                end_date: chrono::NaiveDate::from_ymd(2022, 01, 10),
                incubation_month: 1,
            },
        );
        assert_eq!(expected, inc_months);

        let start_date = "2022-01-01";
        let end_date = "2022-01-11";
        let repository =
            Repo::new(&git_repo, "test", start_date, end_date, "graduated", &args).unwrap();
        let inc_months = repository.parse_date_to_inc_months_with_time_window(10);
        expected.clear();
        expected.insert(
            1,
            IncubationMonth {
                start_date: chrono::NaiveDate::from_ymd(2022, 01, 01),
                end_date: chrono::NaiveDate::from_ymd(2022, 01, 10),
                incubation_month: 1,
            },
        );

        expected.insert(
            2,
            IncubationMonth {
                start_date: chrono::NaiveDate::from_ymd(2022, 01, 11),
                end_date: chrono::NaiveDate::from_ymd(2022, 01, 11),
                incubation_month: 2,
            },
        );
        assert_eq!(expected, inc_months);
    }

    #[test]
    fn test1() {
        let args = Args::from_iter(&["threads=1"]);
        let repo = Repository::open("test_resources/test_repo/.my_git_repo").unwrap();
        let actual = Repo::new(
            &repo,
            "test",
            "2022-03-15",
            "2022-07-08",
            "graduated",
            &args,
        );

        let commits = &actual.as_ref().unwrap().commits;
        let commits_inc_months = actual
            .as_ref()
            .unwrap()
            .commits_to_inc_months_with_time_windows(30, &commits);
        let actual_commits = commits_inc_months
            .as_ref()
            .unwrap()
            .iter()
            .map(|x| x.1)
            .flatten()
            .collect::<Vec<_>>();

        // println!("{:?}", commits_inc_months);
        let expected_months = 4;
        let expected_nr_commits = 5;
        let keys = &commits_inc_months
            .as_ref()
            .unwrap()
            .keys()
            .collect::<Vec<_>>();
        assert_eq!(keys.len(), expected_months);
        assert_eq!(actual_commits.len(), expected_nr_commits);
    }
}
