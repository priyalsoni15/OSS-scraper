use crate::{repo::Repo, Args};
use tokei::{Config, Language, Languages};
pub struct TokeiMetrics {
    stats: Language,
    programming_lang: String,
}

impl<'a> TokeiMetrics {
    /// Runs tokei on a given folder
    pub fn new(repo: &'a Repo<'a>, args: &Args) -> Option<Self> {
        // The paths to search. Accepts absolute, relative, and glob paths.
        let path = repo.repo.path().parent();
        if let Some(path) = path {
            // `Config` allows you to configure what is searched and counted.
            let config = if args.flag_restrict_languages {
                log::info!("restrict-languages flag is on. Configuring tokei to use the languages defined in tokei.toml file.");
                Config::from_config_files()
            } else {
                Config::default()
            };

            let mut languages = Languages::new();

            languages.get_statistics(&vec![path], &vec![], &config);
            let total = languages.total().clone();

            for (_, ref mut language) in &mut languages {
                language.sort_by(tokei::Sort::Code);
            }

            let mut languages: Vec<_> = languages.iter().collect();
            languages.sort_by(|a, b| b.1.code.cmp(&a.1.code));

            if !languages.is_empty() {
                Some(TokeiMetrics {
                    stats: total,
                    programming_lang: languages.first().unwrap().0.name().to_string(),
                })
            } else {
                Some(TokeiMetrics {
                    stats: total,
                    programming_lang: "".to_string(),
                })
            }
        } else {
            None
        }
    }

    pub fn code(&self) -> usize {
        self.stats.code
    }

    pub fn comments(&self) -> usize {
        self.stats.comments
    }

    pub fn blanks(&self) -> usize {
        self.stats.blanks
    }

    pub fn lines(&self) -> usize {
        self.stats.lines()
    }

    pub fn files(&self) -> usize {
        self.stats.children.values().map(Vec::len).sum::<usize>()
    }

    pub fn programming_language(self) -> String {
        self.programming_lang
    }
}
