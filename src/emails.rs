use std::fs::File;

use crate::utils::inc_month_to_date;
use chrono::NaiveDate;
use indexmap::IndexSet;
use mail_parser::mailbox::mbox::Message;

use mail_parser::{mailbox::mbox::MessageIterator};
use mail_parser::{DateTime, HeaderValue};
use scraper::{Html, Selector};
use serde::Serialize;
use std::borrow::Cow;
use ureq::Agent;

pub struct EmailStats {
    pub words: usize,
    pub characters: usize,
}

#[derive(Clone, Default, Debug, Serialize, PartialEq, Eq)]
pub struct EmailsMetrics {
    /// Number of total emails
    pub emails: usize,
    /// Number of developers involved in all email exchanges
    pub devs: usize,
    /// Number of thread starter emails
    pub emails_thread_starter: usize,
    /// Word count for emails that started a thread (the first email in a thread)
    pub emails_thread_starter_word_count: usize,
    /// Number of characters for emails that started a thread (the first email in a thread)
    pub emails_thread_starter_characters: usize,
    /// Number of thread emails (this does no count the emails that started a thread)
    pub emails_threads: usize,
    /// Word count for thread emails. These do not count the email that started the thread
    pub emails_threads_word_count: usize,
    /// Number of characters for thread emails. These do not count the email that started the thread
    pub emails_threads_characters: usize,
    /// Number of emails with no replies
    pub emails_no_replies: usize,
    /// The word count for emails that had no replies
    pub emails_no_replies_word_count: usize,
    /// Number of characters for emails that had no replies
    pub emails_no_replies_characters: usize,
    /// Number of jira emails
    pub emails_jira: usize,
}

impl EmailsMetrics {
    /// Compute the months that are between start date and end date
    /// This outputs a set of Strings that have the following format yyyymm
    ///
    fn dates_to_mbox_months(start_date: NaiveDate, end_date: NaiveDate) -> IndexSet<String> {
        // find how many months are between start date and end date.
        // for each month, we want to analyze the inbox and get the emails that are between start date and end date
        let mut date = end_date.format("%Y%m").to_string();

        // 201001, 201002, etc
        let mut months_to_check = IndexSet::<String>::new();

        let mut year = end_date.format("%Y").to_string().parse::<usize>().unwrap();
        let mut month = end_date.format("%m").to_string().parse::<usize>().unwrap();

        while date != start_date.format("%Y%m").to_string() {
            if month == 1 {
                months_to_check.insert(format!("{:04}{:02}", year, month));
                year -= 1;
                month = 12;
                date = format!("{:04}{:02}", year, month)
            } else {
                months_to_check.insert(format!("{:04}{:02}", year, month));
                month -= 1;
                date = format!("{:04}{:02}", year, month);
            }
        }
        months_to_check.insert(date);
        months_to_check
    }

    pub fn parse_mbox_to_emails(
        path: String,
        _incubation_month_start_date: Option<NaiveDate>,
        _incubation_month_end_date: Option<NaiveDate>,
    ) -> Vec<Option<mail_parser::mailbox::mbox::Message>> {
        let mbox_file = std::fs::File::open(&path);
        match mbox_file {
            Ok(val) => MessageIterator::new(val)
                .into_iter()
                .map(|x| x.ok())
                .collect::<Vec<_>>(),
            Err(_) => {
                log::error!("Cannot open file {}", &path);
                vec![]
            }
        }
    }

    pub fn preprocess_emails(text: Option<Cow<str>>) -> EmailStats {
        if let Some(text) = text {
            // preprocessing the text

            // removing punctuation
            let text = text.replace(".", "");
            let text = text.replace(",", "");

            // Filter out empty lines, and lines that start with >
            let text_lines = text
                .lines()
                .filter(|x| !x.is_empty())
                .filter(|x| *x != " ")
                .filter(|x| !x.starts_with(">"))
                .collect::<Vec<_>>()
                .clone();

            // Remove all the lines that come after, if first lines starts with -----Original Message-----
            let drop_from_index = text_lines
                .clone()
                .into_iter()
                .position(|x| x.starts_with("-----Original Message-----")); //|| x.starts_with(">"));

            let text_lines = if let Some(idx) = drop_from_index {
                text_lines[..idx].to_vec()
            } else {
                text_lines
            };

            let last_line = text_lines.last();
            // if last line starts with On XXXXXX XXXXXX and ends with wrote: we're dealing with a previous email, and we need to remove it
            let lines = match last_line {
                Some(last_line) => {
                    if last_line.starts_with("On") && last_line.ends_with("wrote:") {
                        text_lines[0..text_lines.len() - 1].to_vec()
                    } else {
                        text_lines
                    }
                }
                None => text_lines,
            };

            let lines = lines
                .into_iter()
                .flat_map(|x| x.split_whitespace())
                //.filter(|x| !stop_words.contains(&x.to_string()))
                .collect::<Vec<_>>();

            // println!("{:?}", lines);

            // probably not the best to clone, but ...
            let words = lines.clone().into_iter().count();
            let characters = lines.join(" ").chars().count();
            EmailStats { words, characters }
        } else {
            EmailStats {
                words: 0,
                characters: 0,
            }
        }
    }
    /// Extracts word count and characters from email
    /// Does some preprocessing removing in-line emails
    /// Remove all the lines that come after, if first lines starts with -----Original Message-----
    pub fn extract_email_stats(text: Option<Cow<str>>) -> EmailStats {
        if let Some(text) = text {
            // preprocessing the text

            // removing punctuation
            let text = text.replace(".", "");
            let text = text.replace(",", "");

            // Filter out empty lines, and lines that start with >
            let text_lines = text
                .lines()
                .filter(|x| !x.is_empty())
                .filter(|x| *x != " ")
                .filter(|x| !x.starts_with(">"))
                .collect::<Vec<_>>()
                .clone();

            // Remove all the lines that come after, if first lines starts with -----Original Message-----
            let drop_from_index = text_lines
                .clone()
                .into_iter()
                .position(|x| x.starts_with("-----Original Message-----")); //|| x.starts_with(">"));

            let text_lines = if let Some(idx) = drop_from_index {
                text_lines[..idx].to_vec()
            } else {
                text_lines
            };

            let last_line = text_lines.last();
            // if last line starts with On XXXXXX XXXXXX and ends with wrote: we're dealing with a previous email, and we need to remove it
            let lines = match last_line {
                Some(last_line) => {
                    if last_line.starts_with("On") && last_line.ends_with("wrote:") {
                        text_lines[0..text_lines.len() - 1].to_vec()
                    } else {
                        text_lines
                    }
                }
                None => text_lines,
            };

            let lines = lines
                .into_iter()
                .flat_map(|x| x.split_whitespace())
                //.filter(|x| !stop_words.contains(&x.to_string()))
                .collect::<Vec<_>>();

            // println!("{:?}", lines);

            // probably not the best to clone, but ...
            let words = lines.clone().into_iter().count();
            let characters = lines.join(" ").chars().count();
            EmailStats { words, characters }
        } else {
            EmailStats {
                words: 0,
                characters: 0,
            }
        }
    }

    pub fn parse_emails(
        emails: Vec<Option<Message>>,
        incubation_month_start_date: Option<NaiveDate>,
        incubation_month_end_date: Option<NaiveDate>,
        _ignore_emails_from_addresses: Vec<&str>,
        _ignore_emails_with_subject: Vec<&str>,
    ) -> EmailsMetrics {
        let mut emails_devs = indexmap::IndexSet::<String>::new();
        let mut num_emails = 0;
        let mut emails_threads = 0;
        let mut emails_thread_starter = 0;
        let mut emails_thread_starter_word_count = 0;
        let mut emails_threads_word_count = 0;
        let emails_no_replies = 0;
        let emails_no_replies_word_count = 0;
        let emails_no_replies_characters = 0;
        let mut emails_thread_starter_characters = 0;
        let mut emails_threads_characters = 0;
        let mut emails_jira = 0;
        for parsed_email in emails {
            match parsed_email {
                Some(e) => {
                    let parsed_email = mail_parser::Message::parse(&e.contents()).unwrap();
                    let subject = parsed_email.subject().unwrap_or("");

                    let email_date = parsed_email.date();

                    let from = match parsed_email.from() {
                        HeaderValue::Address(x) => x.name.as_deref().unwrap_or(""),
                        _ => "",
                    };
                    let from = from.replace(",", "");

                    let from_email = match parsed_email.from() {
                        HeaderValue::Address(x) => x.address.as_deref().unwrap_or(""),
                        _ => "",
                    };
                    let from_email = from_email.replace(",", "");

                    if from_email == "jira@apache.org" {
                        emails_jira += 1;
                        continue;
                    }
                    // if ignore_emails_from_addresses.contains(&from_email.as_str()) {
                    //     continue;
                    // }

                    // Ignore emails with these subjects: svn & cvs commits, [jira],
                    if subject.starts_with("svn commit")
                        || subject.starts_with("cvs commit")
                        || subject.contains("[jira]")
                    {
                        continue;
                    }

                    // References can be either Empty -- no reference, so likely an email that is not a reply to another email
                    // or can be Text (one reference) or TextList -- a list of references to message ids -- which means they are replying to another email.
                    let references = parsed_email.references();

                    let dev = from.replace("(Commented) (JIRA)", "").trim().to_string();
                    let dev = dev.replace("(JIRA)", "").trim().to_string();
                    if dev == "jiraposter@reviews.apache.org" {
                        // we cannot differentiate between devs, so let's skip them
                        continue;
                    }
                    if incubation_month_end_date.is_some() && incubation_month_start_date.is_some()
                    {
                        if let Some(date) = email_date {
                            // TODO
                            // we need to catch the panic and stop the unwind because sometimes we get weird dates?
                            // streams-dev-201409 we get 2014-15-09 which is an invalid date
                            let email_date_nd = std::panic::catch_unwind(|| {
                                chrono::NaiveDate::from_ymd(
                                    date.year.into(),
                                    date.month.into(),
                                    date.day.into(),
                                )
                            });

                            if let Ok(email_date_nd) = email_date_nd {
                                if incubation_month_start_date.unwrap() <= email_date_nd
                                    && email_date_nd <= incubation_month_end_date.unwrap()
                                {
                                    num_emails += 1;
                                    emails_devs.insert(dev);
                                }
                            }
                        }
                    } else {
                        num_emails += 1;
                        emails_devs.insert(dev);
                    }

                    let (thread_starter, is_thread_reply) = match references {
                        mail_parser::HeaderValue::Empty => {
                            emails_thread_starter += 1;
                            (true, false)
                        }
                        _ => {
                            emails_threads += 1;
                            (false, true)
                        }
                    };
                    let text = parsed_email.body_text(0);
                    let email_stats = Self::extract_email_stats(text);
                    if thread_starter {
                        emails_thread_starter_word_count += email_stats.words;
                        emails_thread_starter_characters += email_stats.characters;
                    }
                    if is_thread_reply {
                        emails_threads_word_count += email_stats.words;
                        emails_threads_characters += email_stats.characters;
                    }
                }
                None => {
                    log::error!("Cannot parse an email");
                }
            }
        }
        EmailsMetrics {
            emails: num_emails,
            devs: emails_devs.len(),
            emails_thread_starter,
            emails_thread_starter_word_count,
            emails_thread_starter_characters,
            emails_threads,
            emails_threads_word_count,
            emails_threads_characters,
            emails_no_replies,
            emails_no_replies_word_count,
            emails_no_replies_characters,
            emails_jira,
        }
    }

    pub fn metrics_time_window(
        start_date: NaiveDate,
        end_date: NaiveDate,
        root_path: String,
    ) -> EmailsMetrics {
        // println!("{:?} {:?}", start_date, end_date);
        let mut emails = vec![];
        for month in Self::dates_to_mbox_months(start_date, end_date) {
            emails.push(Self::parse_mbox_to_emails(
                format!("{}{}.mbox", root_path, month),
                Some(start_date),
                Some(end_date),
            ));
        }

        let email_metrics = Self::parse_emails(
            emails.into_iter().flatten().collect::<Vec<_>>(),
            Some(start_date),
            Some(end_date),
            vec!["jira@apache.org"],
            vec!["cvs commit", "svn commit", "[jira]"],
        );
        email_metrics
    }

    pub fn metrics(path: String) -> Self {
        // Self::parse_mbox_file(path, None, None)
        Self::parse_emails(
            Self::parse_mbox_to_emails(path, None, None),
            None,
            None,
            vec!["jira@apache.org"],
            vec!["cvs commit", "svn commit", "[jira]"],
        )
    }

    //     pub fn parse_mbox_file(
    //         path: String,
    //         incubation_month_start_date: Option<NaiveDate>,
    //         incubation_month_end_date: Option<NaiveDate>,
    //     ) -> EmailsMetrics {
    //         let mbox_file = std::fs::File::open(&path);
    //         let mut emails = 0;
    //         let mut emails_devs = indexmap::IndexSet::<String>::new();
    //         log::info!("{}", &path);
    //         match mbox_file {
    //             Ok(val) => {
    //                 // let emails = MBoxParser::new(val).collect::<Vec<_>>();
    //                 for raw_message in MBoxParser::new(val) {
    //                     let parsed_email = Message::parse(&raw_message);
    //                     match parsed_email {
    //                         Some(e) => {
    //                             let email_date = e.get_date();

    //                             let from = match e.get_from() {
    //                                 HeaderValue::Address(x) => x.name.as_deref().unwrap_or(""),
    //                                 _ => "",
    //                             };
    //                             let from = from.replace(",", "");

    //                             let from_email = match e.get_from() {
    //                                 HeaderValue::Address(x) => x.address.as_deref().unwrap_or(""),
    //                                 _ => "",
    //                             };
    //                             let from_email = from_email.replace(",", "");

    //                             if from_email == "jira@apache.org" && !from.contains("(Commented)") {
    //                                 continue;
    //                             } else {
    //                                 let dev = from.replace("(Commented) (JIRA)", "").trim().to_string();
    //                                 let dev = dev.replace("(JIRA)", "").trim().to_string();
    //                                 if dev == "jiraposter@reviews.apache.org" {
    //                                     // we cannot differentiate between devs, so let's skip them
    //                                     continue;
    //                                 }
    //                                 if incubation_month_end_date.is_some()
    //                                     && incubation_month_start_date.is_some()
    //                                 {
    //                                     if let Some(date) = email_date {
    //                                         // TODO
    //                                         // we need to catch the panic and stop the unwind because sometimes we get weird dates?
    //                                         // streams-dev-201409 we get 2014-15-09 which is an invalid date
    //                                         let email_date_nd = std::panic::catch_unwind(|| {
    //                                             NaiveDate::from_ymd(
    //                                                 date.year.try_into().unwrap(),
    //                                                 date.month,
    //                                                 date.day,
    //                                             )
    //                                         });

    //                                         if let Ok(email_date_nd) = email_date_nd {
    //                                             if incubation_month_start_date.unwrap() <= email_date_nd
    //                                                 && email_date_nd
    //                                                     <= incubation_month_end_date.unwrap()
    //                                             {
    //                                                 emails += 1;
    //                                                 emails_devs.insert(dev);
    //                                             }
    //                                         }
    //                                     }
    //                                 } else {
    //                                     emails += 1;
    //                                     emails_devs.insert(dev);
    //                                 }
    //                             }
    //                         }
    //                         None => {
    //                             log::error!("Cannot parse an email {}", &path);
    //                         }
    //                     }
    //                 }
    //             }
    //             Err(_) => {
    //                 log::error!("Cannot open file {}", &path);
    //             }
    //         }
    //         EmailsMetrics {
    //             emails,
    //             devs: emails_devs.len(),
    //         }
    //     }
    // }
}

pub fn _local_mboxes_ids(project: &str, emails_storage_folder: &str) -> Vec<String> {
    let mbox_path = emails_storage_folder;
    let files = std::fs::read_dir(&mbox_path);
    let paths = match files {
        Ok(f) => f
            .into_iter()
            .filter_map(|x| if let Ok(p) = x { Some(p.path()) } else { None })
            .collect::<Vec<_>>(),
        Err(ref _e) => {
            // println!("Cannot parse filename, {:?}", &files);
            log::error!("Cannot read mbox directory {mbox_path}");
            vec![]
        }
    };

    #[allow(clippy::unwrap_used)]
    let path_ids = paths.into_iter().filter(|x| {
        x.file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with(&project.to_lowercase())
    });
    // .collect::<Vec<_>>();
    #[allow(clippy::unwrap_used)]
    let mut ids = path_ids
        .into_iter()
        .map(|x| x.as_path().to_str().unwrap().to_string())
        .collect::<Vec<_>>();

    ids.sort();
    let mut output = ids
        .into_iter()
        .map(|x| x.chars().rev().take(11).collect::<String>())
        .collect::<Vec<_>>()
        .into_iter()
        .map(|x| x.chars().rev().take(6).collect::<String>())
        .collect::<Vec<_>>();

    output.reverse();
    output.to_vec()
}

/// Downloads an email from the apache mailing lists, https://lists.apache.org/api/mbox.lua?list=dev&domain={}.apache.org&d={}
///
/// The new apache mailing list server returns an empty file if the file does not exist on their server. Therefore, this function
/// will still create an empty mbox file if there is no mail archive for that month on the server.
pub fn download_email<'a>(
    project: &'a str,
    start_date: &'a str,
    month: usize,
    emails_folder: &'a str,
    agent: &Agent,
) -> Option<String> {
    let date = inc_month_to_date(start_date, month);
    let url = format!(
        // "http://mail-archives.apache.org/mod_mbox/{}-dev/{}.mbox",
        "https://lists.apache.org/api/mbox.lua?list=dev&domain={}.apache.org&d={}",
        project.to_lowercase(),
        &date,
    );

    let filename = format!(
        "{}/{}-dev-{}.mbox",
        emails_folder,
        project.to_lowercase(),
        date.replace('-', "") // we store all the emails as project-dev-yearmonth.mbox, so we need to remove the dash between year and month
    );
    let res = agent.get(&url).call();

    if let Ok(res) = res {
        if res.status() == 200 {
            let mut file = File::create(&filename).expect("Cannot create file {filename}");
            let _res = std::io::copy(&mut res.into_reader(), &mut file);

            Some(filename)
        } else {
            None
        }
    } else {
        None
    }
}

fn check_valid_date(date: &DateTime) -> bool {
    date.month >= 1 && date.month <= 12
}

/// This used to work with the older mail archives of apache. As of Jan 2022, the way they store/serve the mbox files has changed, and thus this does not work anymore
pub fn _mboxes_ids(project: &str) -> Vec<String> {
    let url = format!("http://mail-archives.apache.org/mod_mbox/{}-dev", project);

    let res = ureq::get(&url).call();

    let html = match res {
        Ok(e) => e.into_string().unwrap_or_else(|_| "".to_string()),
        Err(_) => "".to_string(),
    };

    let fragment = Html::parse_fragment(&html);
    let spans = Selector::parse("span");
    if let Ok(spans) = spans {
        let mut ids = vec![];
        for s in fragment
            .select(&spans)
            .into_iter()
            .filter(|s| s.value().classes().any(|a| a == "links"))
        {
            if let Some(id) = s.value().attr("id") {
                ids.push(id.to_string());
            };
        }
        ids.to_vec()
    } else {
        log::error!("Cannot parse span for mboxes ids");
        vec![]
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_download_email() {
//         let agent = ureq::AgentBuilder::new()
//             .timeout_read(std::time::Duration::from_secs(15))
//             .timeout_write(std::time::Duration::from_secs(300))
//             .build();
//         let emails_folder = "../../projects/emails";
//         let project = "samoa";
//         let start_date = "2014-12-15";
//         let month = 2;

//         assert_eq!(
//             Some(format!("{}/{}-dev-201501.mbox", emails_folder, project)),
//             crate::emails::download_email(project, start_date, month, emails_folder, &agent)
//         );

//         assert_eq!(
//             Some(format!("{}/{}-dev-200202.mbox", emails_folder, project)),
//             crate::emails::download_email(project, "2002-01-01", month, emails_folder, &agent)
//         )
//     }

//     #[test]
//     fn test_time_window_emails() {
//         let months_to_check = EmailsMetrics::dates_to_mbox_months(
//             NaiveDate::from_ymd(2000, 10, 1),
//             NaiveDate::from_ymd(2001, 02, 05),
//         );
//         let expected = IndexSet::<String>::from_iter([
//             "200102".to_string(),
//             "200101".to_string(),
//             "200012".to_string(),
//             "200011".to_string(),
//             "200010".to_string(),
//         ]);

//         assert_eq!(months_to_check, expected);

//         let months_to_check = EmailsMetrics::dates_to_mbox_months(
//             NaiveDate::from_ymd(2022, 10, 15),
//             NaiveDate::from_ymd(2022, 11, 05),
//         );
//         let expected = IndexSet::<String>::from_iter(["202211".to_string(), "202210".to_string()]);

//         assert_eq!(months_to_check, expected);

//         let months_to_check = EmailsMetrics::dates_to_mbox_months(
//             NaiveDate::from_ymd(2022, 01, 01),
//             NaiveDate::from_ymd(2022, 01, 05),
//         );
//         let expected = IndexSet::<String>::from_iter(["202201".to_string()]);

//         assert_eq!(months_to_check, expected);
//     }

//     #[test]
//     fn test_metrics_time_window() {
//         let start_date = NaiveDate::from_ymd(2021, 03, 01);
//         let end_date = NaiveDate::from_ymd(2021, 04, 16);

//         let metrics = EmailsMetrics::metrics_time_window(
//             start_date,
//             end_date,
//             "test_resources/mbox/ant-user-".to_string(),
//         );

//         let expected = EmailsMetrics { emails: 5, devs: 3 };
//         assert_eq!(metrics, expected);

//         let start_date = NaiveDate::from_ymd(2021, 03, 01);
//         let end_date = NaiveDate::from_ymd(2021, 05, 16);

//         let metrics = EmailsMetrics::metrics_time_window(
//             start_date,
//             end_date,
//             "test_resources/mbox/ant-user-".to_string(),
//         );

//         let expected = EmailsMetrics { emails: 8, devs: 4 };
//         assert_eq!(metrics, expected);
//     }

//     #[test]
//     fn test_problematic_naive_date_email() {
//         // the email streams-dev-201409.mbox seems to fail when parsing to NaiveDate. It looks
//         // like it's coming from_ymd(u32,u32,u32) call.
//         // DateTime { year: 2014, month: 15, day: 26, hour: 0, minute: 0, second: 0, tz_before_gmt: false, tz_hour: 0, tz_minute: 0 }
//         EmailsMetrics::parse_mbox_file(
//             "test_resources/mbox/streams-dev-201409.mbox".to_string(),
//             Some(NaiveDate::from_ymd(2014, 08, 12)),
//             Some(NaiveDate::from_ymd(2014, 09, 10)),
//         );
//     }
// }

// fn parse_email(args: &Args) -> Result<(), std::io::Error> {
//     let all_projects = list_projects();

//     match std::fs::create_dir_all("../data/emails-csvs") {
//         Ok(()) => println!("Created folder data/emails-csvs"),
//         Err(e) => println!("Cannot create folder data/emails-csvs... {}", e),
//     }

//     let projects_names_fix = IndexMap::from([
//         ("apex-core", "apex"),
//         ("blur", "incubator-blur"),
//         ("derby", "db-derby"),
//         ("empire-db", "empire"),
//         ("ftpserver", "incubator-ftpserver"),
//         ("hcatalog", "incubator-hcatalog"),
//         ("ant-ivy", "incubator-ivy"),
//         ("kalumet", "incubator-kalumet"),
//         ("lucene.net", "lucenenet"),
//         ("mynewt-core", "mynewt"),
//         ("npanday", "incubator-npanday"),
//         ("nuvem", "incubator-nuvem"),
//         ("odftoolkit", "incubator-odf"),
//         ("photark", "incubator-photark"),
//         ("pluto", "portals-pluto"),
//         ("creadur-rat", "creadur"),
//         ("s4", "incubator-s4"),
//         ("sanselan", "incubator-sanselan"),
//         ("servicecomb-java-chassis", "servicecomb"),
//         ("tashi", "incubator-tashi"),
//         ("warble-server", "warble"),
//         ("wave", "incubator-wave"),
//         ("zetacomponents", "incubator-zeta"),
//     ]);

//     let mut project_list: Vec<String> = vec![];
//     // we fixed the naming issue with some projects
//     for p in all_projects {
//         let mut split = p.split(",").collect::<Vec<_>>();
//         let name = split[0];
//         if projects_names_fix.contains_key(name.to_lowercase().as_str()) {
//             let fixed_name = projects_names_fix
//                 .get(name.to_lowercase().as_str())
//                 .unwrap();
//             split[0] = fixed_name;
//             // println!("{:?}", split);
//             let new_val = split.join(",");
//             project_list.push(new_val);
//         } else {
//             project_list.push(p);
//         }
//     }
//     // if we want only a single project, we need to filter it
//     let projects = if let Some(n) = &args.flag_parse_single_project {
//         let project_name = if projects_names_fix.contains_key(n.to_lowercase().as_str()) {
//             projects_names_fix
//                 .get(n.to_lowercase().as_str())
//                 .unwrap()
//                 .to_string()
//         } else {
//             n.to_string()
//         };
//         project_list
//             .into_iter()
//             .filter(|x| {
//                 let split = x.split(",").collect::<Vec<_>>(); // extract the repo name
//                 let repo_name = split[0];
//                 repo_name == project_name
//             })
//             .collect::<Vec<_>>()
//     } else {
//         project_list
//     };
//     if args.flag_show_projects {
//         println!("{:?}", projects);
//     }

//     let mut cwd = std::env::current_dir()
//         .unwrap()
//         .parent()
//         .unwrap()
//         .parent()
//         .unwrap()
//         .to_path_buf();
//     cwd.push("projects");
//     cwd.push("emails");
//     let emails_storage_folder = cwd.as_path().to_str().unwrap();

// println!("{:?}", emails_storage_folder);
// let outputs = projects
//     .par_iter()
//     .map(|p| {
//         // let p = &s.clone();
//         let split = p.split(",").collect::<Vec<_>>();
//         let repo_name = split[0];
//         let _repo_path = split[1];
//         let start_date = split[2];
//         let end_date = split[3];
//         let status = split[4];
//         // download email archives

//         // let ids = mboxes_ids(&repo_name.to_lowercase());
//         // we already have the mboxes locally
//         let ids = local_mboxes_ids(&repo_name, &emails_storage_folder);
//         let mut available_mboxes = vec![];
//         println!("Checking for a local mail archive for {}", repo_name);
//         for id in ids.into_iter() {
//             let path = format!(
//                 "{}/{}-dev-{}.mbox",
//                 emails_storage_folder,
//                 repo_name.to_lowercase(),
//                 id
//             );
//             let file_path = path.replace("\\", "/");
//             let file_path_on_disk = file_path.clone();
//             // if we don't have the mbox file, need to download it
//             let file_exists = Path::new(&file_path).is_file();

//             if file_exists {
//                 available_mboxes.push(file_path.to_string());
//             }
//             if file_exists == false && args.flag_skip_emails_download == true {
//                 continue;
//             }
//             if file_exists == false && args.flag_skip_emails_download == false {
//                 let url = format!(
//                     "http://mail-archives.apache.org/mod_mbox/{}-dev/{}.mbox",
//                     repo_name.to_lowercase(),
//                     id
//                 );
//                 println!(
//                     "{}: {} does not exist locally. Trying to download from {}",
//                     repo_name, &file_path, &url
//                 );
//                 let val = ureq::get(&url)
//                     .timeout(std::time::Duration::new(20, 0))
//                     .call();

//                 match val {
//                     Ok(s) => {
//                         let resp = s.into_string().unwrap();
//                         let f = File::create(file_path_on_disk);
//                         if let Ok(mut ff) = f {
//                             ff.write(resp.as_bytes());
//                             ff.flush();
//                             available_mboxes.push(file_path.to_string());
//                         }
//                     }
//                     Err(_) => println!("Error: {} - request timed out {}", repo_name, url),
//                 }
//             }
//         }
//         println!("Finished downloading mail archive for {}", repo_name);
// println!("{:?}", available_mboxes.clone());
//     available_mboxes.reverse();
//     let output = parse_mbox_files(
//         args,
//         repo_name,
//         status,
//         start_date,
//         end_date,
//         available_mboxes,
//     );

//     let mut f = File::create(format!(
//         "../data/emails-csvs/{}.csv",
//         repo_name.to_lowercase()
//     ))
//     .expect("Unable to create file");
//     match writeln!(f, "{}", output.join("\n")) {
//         Ok(()) => {}
//         Err(e) => println!("{} error occured during writing the file. {}", repo_name, e),
//     }
//     let r = repo_name.to_string().clone();
//     (r, output.len())
// })
// .collect::<Vec<_>>();

// let mut f = File::create("../data/emails-stats.csv").expect("Unable to create file");

// for (key, value) in &outputs {
//     write!(f, "{},{}\n", key, value);
// }
// match f.flush() {
//     Ok(()) => {}
//     Err(e) => println!(
//         "Writing the emails-stats.csv: error occured during flushing the file. {}",
//         e
//     ),
// }

// let mut f = File::create("../data/emails-data.csv").expect("Unable to create file");

// write!(
//     f,
//     "project,status,start_date,end_date,string_date,from,from_email,sender,sender_email,to_name,to_email\n"
// );

// match f.flush() {
//     Ok(()) => {}
//     Err(e) => println!(
//         "Writing the emails-stats.csv: error occured during flushing the file. {}",
//         e
//     ),
// }

// println!(
//     "Total emails analyzed: {:?}",
//     outputs
//         .into_iter()
//         .map(|s| s.1)
//         .collect::<Vec<_>>()
//         .iter()
//         .sum::<usize>()
// );
// Ok(())
// }
