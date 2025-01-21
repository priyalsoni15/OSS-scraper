use indexmap::IndexMap;
use serde::Serialize;
use serde_json::{json, Value};
use std::process::Command;

use crate::utils;
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct SokratesMetrics {
    most_complex_unit_loc: f64,
    most_complex_unit_mcabe_index: f64,
    total_number_of_files: f64,
    number_of_files_main: f64,
    lines_of_code_main: f64,
    number_of_files_test: f64,
    lines_of_code_test: f64,
    test_vs_main_lines_of_code_percentage: f64,
    number_of_files_generated: f64,
    lines_of_code_generated: f64,
    number_of_files_build_and_deployment: f64,
    lines_of_code_build_and_deployment: f64,
    negligible_risk_file_size_count: f64,
    low_risk_file_size_count: f64,
    medium_risk_file_size_count: f64,
    high_risk_file_size_count: f64,
    very_high_risk_file_size_count: f64,
    negligible_risk_file_size_loc: f64,
    low_risk_file_size_loc: f64,
    medium_risk_file_size_loc: f64,
    high_risk_file_size_loc: f64,
    very_high_risk_file_size_loc: f64,
    number_of_units: f64,
    lines_of_code_in_units: f64,
    lines_of_code_outside_units: f64,
    unit_size_negligible_risk_loc: f64,
    unit_size_negligible_risk_count: f64,
    unit_size_low_risk_loc: f64,
    unit_size_low_risk_count: f64,
    unit_size_medium_risk_loc: f64,
    unit_size_medium_risk_count: f64,
    unit_size_high_risk_loc: f64,
    unit_size_high_risk_count: f64,
    unit_size_very_high_risk_loc: f64,
    unit_size_very_high_risk_count: f64,
    conditional_complexity_negligible_risk_loc: f64,
    conditional_complexity_negligible_risk_count: f64,
    conditional_complexity_low_risk_loc: f64,
    conditional_complexity_low_risk_count: f64,
    conditional_complexity_medium_risk_loc: f64,
    conditional_complexity_medium_risk_count: f64,
    conditional_complexity_high_risk_loc: f64,
    conditional_complexity_high_risk_count: f64,
    conditional_complexity_very_high_risk_loc: f64,
    conditional_complexity_very_high_risk_count: f64,
    conditional_complexity_high_plus_risk_count: f64,
    conditional_complexity_high_plus_risk_loc: f64,
    number_of_contributors: f64,
    duplication_number_of_duplicates: f64,
    duplication_number_of_files_with_duplicates: f64,
    duplication_number_of_duplicated_lines: f64,
    duplication_percentage: f64,
    unit_duplicates_count: f64,
}

impl From<IndexMap<String, f64>> for SokratesMetrics {
    fn from(map: IndexMap<String, f64>) -> Self {
        SokratesMetrics {
            most_complex_unit_loc: *map.get("most_complex_unit_loc").unwrap_or(&0.0),
            most_complex_unit_mcabe_index: *map
                .get("most_complex_unit_mcabe_index")
                .unwrap_or(&0.0),
            conditional_complexity_high_plus_risk_count: *map
                .get("conditional_complexity_high_plus_risk_count")
                .unwrap_or(&0.0),

            conditional_complexity_high_plus_risk_loc: *map
                .get("conditional_complexity_high_plus_risk_loc")
                .unwrap_or(&0.0),

            conditional_complexity_high_risk_count: *map
                .get("conditional_complexity_high_risk_count")
                .unwrap_or(&0.0),
            conditional_complexity_high_risk_loc: *map
                .get("conditional_complexity_high_risk_loc")
                .unwrap_or(&0.0),
            conditional_complexity_low_risk_count: *map
                .get("conditional_complexity_low_risk_count")
                .unwrap_or(&0.0),
            conditional_complexity_low_risk_loc: *map
                .get("conditional_complexity_low_risk_loc")
                .unwrap_or(&0.0),
            conditional_complexity_medium_risk_count: *map
                .get("conditional_complexity_medium_risk_count")
                .unwrap_or(&0.0),
            conditional_complexity_medium_risk_loc: *map
                .get("conditional_complexity_medium_risk_loc")
                .unwrap_or(&0.0),
            conditional_complexity_negligible_risk_count: *map
                .get("conditional_complexity_negligible_risk_count")
                .unwrap_or(&0.0),
            conditional_complexity_negligible_risk_loc: *map
                .get("conditional_complexity_negligible_risk_loc")
                .unwrap_or(&0.0),
            conditional_complexity_very_high_risk_count: *map
                .get("conditional_complexity_very_high_risk_count")
                .unwrap_or(&0.0),
            conditional_complexity_very_high_risk_loc: *map
                .get("conditional_complexity_very_high_risk_loc")
                .unwrap_or(&0.0),
            // duplication_number_of_cleaned_lines_primary_root: *map
            //     .get("duplication_number_of_cleaned_lines_primary_root")
            //     .unwrap_or(&0.0),
            duplication_number_of_duplicated_lines: *map
                .get("duplication_number_of_duplicated_lines")
                .unwrap_or(&0.0),
            // duplication_number_of_duplicated_lines_primary_root: *map
            //     .get("duplication_number_of_duplicated_lines_primary_root")
            //     .unwrap_or(&0.0),
            duplication_number_of_duplicates: *map
                .get("duplication_number_of_duplicates")
                .unwrap_or(&0.0),
            duplication_number_of_files_with_duplicates: *map
                .get("duplication_number_of_files_with_duplicates")
                .unwrap_or(&0.0),
            duplication_percentage: *map.get("duplication_percentage").unwrap_or(&0.0),
            // duplication_percentage_primary_root: *map
            //     .get("duplication_percentage_primary_root")
            //     .unwrap_or(&0.0),
            high_risk_file_size_count: *map.get("high_risk_file_size_count").unwrap_or(&0.0),
            high_risk_file_size_loc: *map.get("high_risk_file_size_loc").unwrap_or(&0.0),
            lines_of_code_build_and_deployment: *map
                .get("lines_of_code_build_and_deployment")
                .unwrap_or(&0.0),
            lines_of_code_generated: *map.get("lines_of_code_generated").unwrap_or(&0.0),
            lines_of_code_in_units: *map.get("lines_of_code_in_units").unwrap_or(&0.0),
            lines_of_code_main: *map.get("lines_of_code_main").unwrap_or(&0.0),
            lines_of_code_outside_units: *map.get("lines_of_code_outside_units").unwrap_or(&0.0),
            lines_of_code_test: *map.get("lines_of_code_test").unwrap_or(&0.0),
            low_risk_file_size_count: *map.get("low_risk_file_size_count").unwrap_or(&0.0),
            low_risk_file_size_loc: *map.get("low_risk_file_size_loc").unwrap_or(&0.0),
            medium_risk_file_size_count: *map.get("medium_risk_file_size_count").unwrap_or(&0.0),
            medium_risk_file_size_loc: *map.get("medium_risk_file_size_loc").unwrap_or(&0.0),
            negligible_risk_file_size_count: *map
                .get("negligible_risk_file_size_count")
                .unwrap_or(&0.0),
            negligible_risk_file_size_loc: *map
                .get("negligible_risk_file_size_loc")
                .unwrap_or(&0.0),
            number_of_contributors: *map.get("number_of_contributors").unwrap_or(&0.0),
            number_of_files_build_and_deployment: *map
                .get("number_of_files_build_and_deployment")
                .unwrap_or(&0.0),
            number_of_files_generated: *map.get("number_of_files_generated").unwrap_or(&0.0),
            number_of_files_main: *map.get("number_of_files_main").unwrap_or(&0.0),
            number_of_files_test: *map.get("number_of_files_test").unwrap_or(&0.0),
            number_of_units: *map.get("number_of_units").unwrap_or(&0.0),
            test_vs_main_lines_of_code_percentage: *map
                .get("test_vs_main_lines_of_code_percentage")
                .unwrap_or(&0.0),
            total_number_of_files: *map.get("total_number_of_files").unwrap_or(&0.0),
            unit_duplicates_count: *map.get("unit_duplicates_count").unwrap_or(&0.0),
            unit_size_high_risk_count: *map.get("unit_size_high_risk_count").unwrap_or(&0.0),
            unit_size_high_risk_loc: *map.get("unit_size_high_risk_loc").unwrap_or(&0.0),
            unit_size_low_risk_count: *map.get("unit_size_low_risk_count").unwrap_or(&0.0),
            unit_size_low_risk_loc: *map.get("unit_size_low_risk_loc").unwrap_or(&0.0),
            unit_size_medium_risk_count: *map.get("unit_size_medium_risk_count").unwrap_or(&0.0),
            unit_size_medium_risk_loc: *map.get("unit_size_medium_risk_loc").unwrap_or(&0.0),
            unit_size_negligible_risk_count: *map
                .get("unit_size_negligible_risk_count")
                .unwrap_or(&0.0),
            unit_size_negligible_risk_loc: *map
                .get("unit_size_negligible_risk_loc")
                .unwrap_or(&0.0),
            unit_size_very_high_risk_count: *map
                .get("unit_size_very_high_risk_count")
                .unwrap_or(&0.0),
            unit_size_very_high_risk_loc: *map.get("unit_size_very_high_risk_loc").unwrap_or(&0.0),
            very_high_risk_file_size_count: *map
                .get("very_high_risk_file_size_count")
                .unwrap_or(&0.0),
            very_high_risk_file_size_loc: *map.get("very_high_risk_file_size_loc").unwrap_or(&0.0),
        }
    }
}

pub struct Sokrates {
    java_path: String,
    path: String,
    pub metrics: SokratesMetrics,
}

impl Sokrates {
    pub fn new(path: &str, java_path: String) -> Self {
        Sokrates {
            java_path,
            path: path.to_string(),
            metrics: SokratesMetrics::default(),
        }
    }

    pub fn extract_history(
        &self,
        project: &str,
        month: &usize,
        hash: &str,
    ) -> Result<std::process::Output, std::io::Error> {
        log::info!(
            "{} month: {} - sokrates extracting history at {}",
            project,
            month,
            hash
        );

        let output = Command::new(&self.java_path)
            .arg("-jar")
            .arg("-Xmx2g")
            .arg("-Xms2g")
            .arg("tools/sokrates.jar")
            .arg("extractGitHistory")
            .arg("-analysisRoot")
            .arg(self.path.as_str())
            .output()?;

        Ok(output)
    }

    pub fn init(
        &self,
        project: &str,
        month: &usize,
        hash: &str,
    ) -> Result<std::process::Output, std::io::Error> {
        log::info!(
            "{} month: {} - sokrates initialize at {}",
            project,
            month,
            hash
        );
        Ok(Command::new(&self.java_path)
            .arg("-jar")
            .arg("-Xmx2g")
            .arg("-Xms2g")
            .arg("tools/sokrates.jar")
            .arg("init")
            .arg("-srcRoot")
            .arg(self.path.as_str())
            .output()?)
    }

    pub fn adjust_analysis(&self) -> Result<(), std::io::Error> {
        let cfg_path = format!("{}/_sokrates/config.json", &self.path);
        let cfg_contents = std::fs::read_to_string(&cfg_path);

        if let Ok(contents) = cfg_contents {
            let mut json: Value = serde_json::from_str(&contents)?;
            // json["analysis"]["skipDuplication"] = json!(true);
            json["analysis"]["skipDependencies"] = json!(true);
            json["analysis"]["cacheSourceFiles"] = json!(false);
            json["analysis"]["saveCodeFragments"] = json!(false);

            std::fs::write(cfg_path, json.to_string())?;
        }

        Ok(())
    }

    pub fn adjust_files_to_be_analyzed(&self) -> Result<(), std::io::Error> {
        let cfg_path = format!("{}/_sokrates/config.json", &self.path);
        let cfg_contents = std::fs::read_to_string(&cfg_path);

        if let Ok(contents) = cfg_contents {
            let mut json: Value = serde_json::from_str(&contents)?;
            let extensions = utils::find_lang_extensions().unwrap();
            let old_extensions = &json["extensions"];

            // find which extensions from Sokrates initial config file overlap with the restricted languages we've set
            let mut new_exts = vec![];
            for ext in old_extensions.as_array().unwrap() {
                if extensions.contains(ext.as_str().unwrap_or("")) {
                    new_exts.push(ext.as_str().unwrap());
                }
            }
            json["extensions"] = json!(new_exts);

            std::fs::write(cfg_path, json.to_string())?;
        }

        Ok(())
    }

    pub fn generate_reports(
        &self,
        project: &str,
        month: &usize,
        hash: &str,
    ) -> Result<std::process::Output, std::io::Error> {
        log::info!(
            "{} month: {} - sokrates generate reports at {}",
            project,
            month,
            hash
        );
        Ok(Command::new(&self.java_path)
            .arg("-jar")
            .arg("-Xmx2g")
            .arg("-Xms2g")
            .arg("tools/sokrates.jar")
            .arg("generateReports")
            .arg("-confFile")
            .arg(format!("{}/_sokrates/config.json", self.path).as_str())
            .arg("-outputFolder")
            .arg(format!("{}/_sokrates", self.path).as_str())
            .output()?)
    }

    pub fn cleanup(&self, project: &str, month: &usize, hash: &str) -> Result<(), std::io::Error> {
        log::info!(
            "{} month: {} - sokrates cleanup at {}",
            project,
            month,
            hash
        );
        std::fs::remove_dir_all(format!("{}/_sokrates", self.path))?;
        std::fs::remove_file(format!("{}/git-history.txt", self.path))?;

        Ok(())
    }

    pub fn metrics(&self) -> Result<SokratesMetrics, std::io::Error> {
        let analysis_filename = format!("{}/_sokrates/data/analysisResults.json", self.path);

        let data = std::fs::read_to_string(analysis_filename)?;
        let metrics_vals_map = self._parse_json(&data)?;
        let metrics = SokratesMetrics::from(metrics_vals_map);
        Ok(metrics)
    }

    /// This method parses the analysis result file of Sokrates and builds a key-value map, where keys are the metrics
    fn _parse_json(&self, data: &str) -> Result<IndexMap<String, f64>, std::io::Error> {
        let json_vals: Value = serde_json::from_str(data)?;
        let metrics_list = &json_vals["metricsList"];

        let metrics = &metrics_list["metrics"];

        let mut metrics_vals_map = metrics
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let metric_name = x["id"].as_str().unwrap().to_string().to_lowercase();
                let value = {
                    if x["value"].is_f64() || x["value"].is_u64() {
                        x["value"].as_f64().unwrap_or(0.0)
                    // } else if x["value"].is_u64() {
                    // x["value"].as_f64().unwrap_or(0.0)
                    } else {
                        0.0
                    }
                };
                (metric_name, value)
            })
            .collect::<IndexMap<String, f64>>();

        let (most_complex_unit_loc, most_complex_unit_mcabe_index) =
            self._parse_complex_units(&json_vals);

        metrics_vals_map.insert("most_complex_unit_loc".to_string(), most_complex_unit_loc);
        metrics_vals_map.insert(
            "most_complex_unit_mcabe_index".to_string(),
            most_complex_unit_mcabe_index,
        );
        Ok(metrics_vals_map)
    }

    fn _parse_complex_units(&self, json_vals: &Value) -> (f64, f64) {
        let most_complex_units =
            &json_vals["unitsAnalysisResults".to_string()]["mostComplexUnits"][0];
        let most_complex_unit_loc = most_complex_units["linesOfCode".to_string()]
            .as_f64()
            .unwrap_or(0.0);

        let most_complex_unit_mcabe_index = most_complex_units["mcCabeIndex".to_string()]
            .as_f64()
            .unwrap_or(0.0);
        (most_complex_unit_loc, most_complex_unit_mcabe_index)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_complex_unit_loc() {
        let java_path = crate::java_path();
        let analysis_filename = format!("test_resources/analysisResults.json");
        let sokrates = Sokrates::new("test_resources/git_repo", java_path);

        let data = std::fs::read_to_string(analysis_filename);
        if let Ok(data) = data {
            let json_vals: Value = serde_json::from_str(&data).unwrap();
            let (loc, mcabe) = sokrates._parse_complex_units(&json_vals);
            assert_eq!(83.0, loc);
            assert_eq!(59.0, mcabe);
        }
    }

    #[test]
    fn test_sokrates_commands() -> Result<(), std::io::Error> {
        let java_path = crate::java_path();

        let sokrates = Sokrates::new("test_resources/git_repo", java_path);
        let history = sokrates.extract_history("git_repo", &1, "hash")?;
        assert!(history.status.success());

        let init = sokrates.init("git_repo", &1, "hash")?;
        assert!(init.status.success());

        let reports = sokrates.generate_reports("git_repo", &1, "hash")?;
        assert!(reports.status.success());
        std::fs::remove_file("test_resources/git_repo/git-history.txt")?;
        std::fs::remove_dir_all("test_resources/git_repo/_sokrates")?;
        Ok(())
    }

    #[test]
    fn test_parse_json() {
        let analysis_filename = format!("test_resources/analysisResults.json");
        let java_path = crate::java_path();

        let sokrates = Sokrates::new("test_resources/git_repo", java_path);
        let data = std::fs::read_to_string(analysis_filename).unwrap();
        let metrics_vals_map = sokrates._parse_json(&data).unwrap();
        let metrics = SokratesMetrics::from(metrics_vals_map);

        let expected_metrics_map = IndexMap::from([
            ("TOTAL_NUMBER_OF_FILES".to_lowercase(), 1450.0),
            ("NUMBER_OF_FILES_MAIN".to_lowercase(), 841.0),
            ("LINES_OF_CODE_MAIN".to_lowercase(), 143260.0),
            ("NUMBER_OF_FILES_TEST".to_lowercase(), 171.0),
            ("LINES_OF_CODE_TEST".to_lowercase(), 18890.0),
            (
                "TEST_VS_MAIN_LINES_OF_CODE_PERCENTAGE".to_lowercase(),
                13.18,
            ),
            ("NUMBER_OF_FILES_GENERATED".to_lowercase(), 97.0),
            ("LINES_OF_CODE_GENERATED".to_lowercase(), 133310.0),
            ("NUMBER_OF_FILES_BUILD_AND_DEPLOYMENT".to_lowercase(), 40.0),
            ("LINES_OF_CODE_BUILD_AND_DEPLOYMENT".to_lowercase(), 4176.0),
            ("NEGLIGIBLE_RISK_FILE_SIZE_COUNT".to_lowercase(), 572.0),
            ("LOW_RISK_FILE_SIZE_COUNT".to_lowercase(), 155.0),
            ("MEDIUM_RISK_FILE_SIZE_COUNT".to_lowercase(), 78.0),
            ("HIGH_RISK_FILE_SIZE_COUNT".to_lowercase(), 20.0),
            ("VERY_HIGH_RISK_FILE_SIZE_COUNT".to_lowercase(), 16.0),
            ("NEGLIGIBLE_RISK_FILE_SIZE_LOC".to_lowercase(), 21631.0),
            ("LOW_RISK_FILE_SIZE_LOC".to_lowercase(), 21962.0),
            ("MEDIUM_RISK_FILE_SIZE_LOC".to_lowercase(), 24015.0),
            ("HIGH_RISK_FILE_SIZE_LOC".to_lowercase(), 13118.0),
            ("VERY_HIGH_RISK_FILE_SIZE_LOC".to_lowercase(), 62534.0),
            ("NUMBER_OF_UNITS".to_lowercase(), 10221.0),
            ("LINES_OF_CODE_IN_UNITS".to_lowercase(), 112059.0),
            ("LINES_OF_CODE_OUTSIDE_UNITS".to_lowercase(), 31201.0),
            ("UNIT_SIZE_NEGLIGIBLE_RISK_LOC".to_lowercase(), 31581.0),
            ("UNIT_SIZE_NEGLIGIBLE_RISK_COUNT".to_lowercase(), 6986.0),
            ("UNIT_SIZE_LOW_RISK_LOC".to_lowercase(), 26763.0),
            ("UNIT_SIZE_LOW_RISK_COUNT".to_lowercase(), 1826.0),
            ("UNIT_SIZE_MEDIUM_RISK_LOC".to_lowercase(), 36563.0),
            ("UNIT_SIZE_MEDIUM_RISK_COUNT".to_lowercase(), 1189.0),
            ("UNIT_SIZE_HIGH_RISK_LOC".to_lowercase(), 12151.0),
            ("UNIT_SIZE_HIGH_RISK_COUNT".to_lowercase(), 182.0),
            ("UNIT_SIZE_VERY_HIGH_RISK_LOC".to_lowercase(), 5001.0),
            ("UNIT_SIZE_VERY_HIGH_RISK_COUNT".to_lowercase(), 38.0),
            (
                "CONDITIONAL_COMPLEXITY_NEGLIGIBLE_RISK_LOC".to_lowercase(),
                67976.0,
            ),
            (
                "CONDITIONAL_COMPLEXITY_NEGLIGIBLE_RISK_COUNT".to_lowercase(),
                9104.0,
            ),
            (
                "CONDITIONAL_COMPLEXITY_LOW_RISK_LOC".to_lowercase(),
                27848.0,
            ),
            (
                "CONDITIONAL_COMPLEXITY_LOW_RISK_COUNT".to_lowercase(),
                864.0,
            ),
            (
                "CONDITIONAL_COMPLEXITY_MEDIUM_RISK_LOC".to_lowercase(),
                13562.0,
            ),
            (
                "CONDITIONAL_COMPLEXITY_MEDIUM_RISK_COUNT".to_lowercase(),
                232.0,
            ),
            (
                "CONDITIONAL_COMPLEXITY_HIGH_RISK_LOC".to_lowercase(),
                2475.0,
            ),
            (
                "CONDITIONAL_COMPLEXITY_HIGH_RISK_COUNT".to_lowercase(),
                19.0,
            ),
            (
                "CONDITIONAL_COMPLEXITY_VERY_HIGH_RISK_LOC".to_lowercase(),
                198.0,
            ),
            (
                "CONDITIONAL_COMPLEXITY_VERY_HIGH_RISK_COUNT".to_lowercase(),
                2.0,
            ),
            (
                "CONDITIONAL_COMPLEXITY_HIGH_PLUS_RISK_COUNT".to_lowercase(),
                21.0,
            ),
            (
                "CONDITIONAL_COMPLEXITY_HIGH_PLUS_RISK_LOC".to_lowercase(),
                2673.0,
            ),
            ("NUMBER_OF_CONTRIBUTORS".to_lowercase(), 37.0),
            ("DUPLICATION_NUMBER_OF_DUPLICATES".to_lowercase(), 153611.0),
            (
                "DUPLICATION_NUMBER_OF_FILES_WITH_DUPLICATES".to_lowercase(),
                249.0,
            ),
            (
                "DUPLICATION_NUMBER_OF_DUPLICATED_LINES".to_lowercase(),
                40932.0,
            ),
            (
                "DUPLICATION_NUMBER_OF_DUPLICATED_LINES".to_lowercase(),
                40932.0,
            ),
            ("DUPLICATION_PERCENTAGE".to_lowercase(), 35.83234120036417),
            ("UNIT_DUPLICATES_COUNT".to_lowercase(), 366.0),
            ("most_complex_unit_loc".to_lowercase(), 83.0),
            ("most_complex_unit_mcabe_index".to_lowercase(), 59.0),
        ]);

        let expected_metrics = SokratesMetrics::from(expected_metrics_map);

        assert_eq!(metrics, expected_metrics);
    }
}
