use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Utc};
use git2::Time;
use indexmap::{IndexMap, IndexSet};
use serde::Deserialize;
use serde_json::Value;
use walkdir::{DirEntry, WalkDir};

use crate::repo::IncubationMonth;
pub fn convert_time(time: &Time) -> DateTime<Utc> {
    let tz = chrono::FixedOffset::east(time.offset_minutes() * 60);
    tz.timestamp(time.seconds(), 0).with_timezone(&Utc)
}

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn is_source_file<'a>(
    path: Option<&'a std::path::Path>,
    extensions: &IndexSet<String>,
) -> bool {
    if let Some(path) = path {
        if let Some(p) = path.extension() {
            extensions.contains(p.to_str().unwrap())
        } else {
            false
        }
    } else {
        false
    }
}

pub fn directories(path: &str) -> usize {
    let walker = WalkDir::new(path).into_iter();
    let count = walker
        .filter_entry(|e| !is_hidden(e) && e.file_type().is_dir())
        .count();
    if count == 1 {
        // only the path we gave is a dir
        0
    } else {
        count - 1
    }
}

pub fn top_level_directories(path: &str) -> usize {
    let mut count = 0;
    let walker = WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        if let Ok(entry) = entry {
            if entry.file_type().is_dir() && entry.depth() == 1 {
                count += 1;
            }
        } else {
            log::error!("{} - directory entry is not parsable?", path);
        }
    }

    count
}

/// Parses the incubation start and end dates to a list of incubation months,
/// where the time window defines the number of days for each incubation month
/// The returned data is a hash map with the date as keys. The date reflects the
/// last date of the incubation month. Values are the incubation month index
/// For example: time window = 30; start date = 2010-01-01, end_date = 2010-03-05
/// The result will be: 1 -> 2010-01-01, 2010-01-30, 2 -> 2010-01-31 - 2010-03-01, 3 -> 2010-03-02 - 2010-03-05, .
/// This is an ordered HashMap
pub fn parse_date_to_inc_months_with_time_window(
    start_date: &str,
    end_date: &str,
    time_window: i64,
) -> IndexMap<usize, IncubationMonth> {
    let mut result = IndexMap::<usize, IncubationMonth>::new();

    let sd = chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d").unwrap();
    let ed = chrono::NaiveDate::parse_from_str(end_date, "%Y-%m-%d").unwrap();

    if sd + Duration::days(time_window - 1) >= ed {
        let incubation_month = IncubationMonth {
            start_date: sd,
            end_date: ed,
            incubation_month: 1,
        };
        let mut set = IndexMap::new();
        set.insert(1, incubation_month);
        return set;
    }

    // keep track of the start date and the end date of the first incubation month
    let mut start_date_current_month = sd;
    let mut end_date_current_month = sd + chrono::Duration::days(time_window - 1);
    let mut current_month = 0;

    loop {
        // println!("{}", end_date_current_month);
        if end_date_current_month > ed {
            let incubation_month = IncubationMonth {
                start_date: start_date_current_month,
                end_date: ed,
                incubation_month: current_month + 1,
            };
            result.insert(current_month + 1, incubation_month);
            break;
        } else {
            current_month += 1;
            let incubation_month = IncubationMonth {
                start_date: start_date_current_month,
                end_date: end_date_current_month,
                incubation_month: current_month,
            };
            result.insert(current_month, incubation_month);

            // shift the window, and start with +1 day from the previous last end date of the incubation month
            start_date_current_month = end_date_current_month + Duration::days(1);
            // we might be reaching the last day
            if start_date_current_month > ed {
                break;
            } else if start_date_current_month == ed {
                result.insert(
                    current_month + 1,
                    IncubationMonth {
                        start_date: ed,
                        end_date: ed,
                        incubation_month: current_month + 1,
                    },
                );
                break;
            }
            end_date_current_month = end_date_current_month + Duration::days(time_window);
        }
    }

    result
}

#[derive(Deserialize, Debug)]
struct LanguageExtensions {
    languages: Languages,
}

#[derive(Deserialize, Debug)]
struct Languages {
    types: Vec<String>,
}

pub(crate) fn find_lang_extensions() -> Result<IndexSet<String>, serde_json::Error> {
    let mut extensions: IndexSet<String> = IndexSet::new();

    let exts_filename = "extensions.toml";
    let languages_json_exts = "languages.json";
    let contents = match std::fs::read_to_string(exts_filename) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            log::error!("Could not read file `{}`", exts_filename);
            return Ok(extensions);
        }
    };

    let data: LanguageExtensions = match toml::from_str(&contents) {
        // If successful, return data as `Data` struct.
        Ok(d) => d,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            log::error!("Unable to load data from `{}`", exts_filename);
            // Exit the program with exit code `1`.
            // exit(1);
            return Ok(extensions);
        }
    };

    let json_contents = match std::fs::read_to_string(languages_json_exts) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            log::error!("Could not read file `{}`", languages_json_exts);
            return Ok(extensions);
        }
    };

    let v: Value = serde_json::from_str(&json_contents)?;

    for e in &data.languages.types {
        let exts = &v["languages"][e]["extensions"];
        for a in exts.as_array() {
            for ext in a {
                extensions.insert(ext.as_str().unwrap().to_string());
            }
        }
    }

    Ok(extensions)
}

fn _incubation_days(start_date: &str, end_date: &str) -> i64 {
    let sd = chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d").unwrap();
    let ed = chrono::NaiveDate::parse_from_str(end_date, "%Y-%m-%d").unwrap();

    let days = ed.signed_duration_since(sd);
    days.num_days()
}

fn _incubation_month(start_date: &str, date: &str) -> i64 {
    let sd = chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d").unwrap();
    let tz_offset = chrono::FixedOffset::east(1 * 3600);
    let time = chrono::NaiveTime::from_hms(12, 0, 0);
    let datetime = chrono::NaiveDateTime::new(sd, time);
    let dt_with_tz: chrono::DateTime<chrono::FixedOffset> =
        tz_offset.from_local_datetime(&datetime).unwrap();

    let ed = chrono::DateTime::parse_from_rfc3339(date).unwrap();

    let days = ed.signed_duration_since(dt_with_tz);
    // println!("{}", days.num_days());

    let month = (days.num_days() as f64 / 30 as f64).ceil();
    month as i64
}

pub fn inc_month_to_date(start_date: &str, inc_month: usize) -> String {
    let sd = NaiveDate::parse_from_str(start_date, "%Y-%m-%d").unwrap();
    let mut year = sd.year();
    let mut month = sd.month();

    let mut desired_month = inc_month;

    while desired_month > 1 {
        if month == 12 {
            year += 1;
            month = 1;
        } else {
            month += 1;
        }
        desired_month -= 1
    }
    if month < 10 {
        format!("{}-0{}", year, month)
    } else {
        format!("{}-{}", year, month)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_directories() {
        assert_eq!(4, directories("test_resources/test_directories"));
        assert_eq!(0, directories("test_resources/test_directories/dir2"));
        assert_eq!(1, directories("test_resources/test_directories/dir1"));
    }

    #[test]
    fn test_top_level_directories() {
        assert_eq!(3, top_level_directories("test_resources/test_directories"));
        assert_eq!(
            0,
            top_level_directories("test_resources/test_directories/dir2")
        );
        assert_eq!(
            1,
            top_level_directories("test_resources/test_directories/dir1")
        );
    }
    #[test]
    fn test_incubation_months() {
        assert_eq!(
            1,
            _incubation_month("2010-10-30", "2010-11-01T16:39:57+00:00")
        );

        assert_eq!(
            2,
            _incubation_month("2010-10-30", "2010-11-30T16:39:57+00:00")
        );

        assert_ne!(
            1,
            _incubation_month("2010-10-30", "2010-11-30T16:39:57+00:00")
        );

        assert_eq!(
            14,
            _incubation_month("2010-10-30", "2011-11-30T16:39:57+00:00")
        );
    }
    #[test]
    fn test_inc_month_to_date() {
        let start_date = "2010-10-15";
        let month = 3;
        let expected = "2010-12";

        assert_eq!(inc_month_to_date(start_date, month), expected.to_string());

        assert_eq!(inc_month_to_date(start_date, 1), "2010-10".to_string());
        assert_eq!(inc_month_to_date(start_date, 4), "2011-01".to_string());

        assert_eq!(inc_month_to_date(start_date, 20), "2012-05".to_string());
    }

    #[test]
    fn test_parse_date_to_inc_months_with_time_window() {
        let start_date = "2009-09-22";
        let end_date = "2010-12-15";
        let time_window = 30;

        let inc_months =
            parse_date_to_inc_months_with_time_window(start_date, end_date, time_window);

        let mut expected = indexmap::IndexMap::<usize, IncubationMonth>::new();
        expected.insert(
            1,
            IncubationMonth {
                start_date: chrono::NaiveDate::from_ymd(2010, 10, 17),
                end_date: chrono::NaiveDate::from_ymd(2010, 11, 15),
                incubation_month: 1,
            },
        );
        expected.insert(
            2,
            IncubationMonth {
                start_date: chrono::NaiveDate::from_ymd(2010, 11, 16),
                end_date: chrono::NaiveDate::from_ymd(2010, 12, 15),
                incubation_month: 2,
            },
        );

        assert_eq!(expected, inc_months);
    }
}
