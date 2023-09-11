use std::collections::HashSet;
use std::collections::HashMap;

use crate::Settings;
use crate::core::source::Source;
use crate::utils::io::{file_write_lines, file_read_lines};

use crate::core::models::source_config_model::Automation;

pub struct SourcesManager {
    pub settings: Settings,
    pub sources_file: String,
    pub sources: HashSet<Source>,
}

impl SourcesManager {
    pub fn new(settings: Settings) -> Self {
        let sources_file = settings.sources_file.clone();
        let sources = Self::load_sources(&sources_file, &settings);
        Self {
            settings,
            sources_file,
            sources,
        }
    }

    pub fn remove_source(&mut self, source: Source) {
        self.sources.remove(&source);
        self.save();
    }

    pub fn add_source(&mut self, source: Source) {
        self.sources.insert(source);
        self.save();
    }

    pub fn save(&self) {
        Self::save_sources(&self.sources_file, &self.sources);
    }

    fn save_sources(sources_file: &str, sources: &HashSet<Source>) {
        let source_strings: Vec<String> = sources.iter().map(|s| s.to_string()).collect();
        let _ = file_write_lines(sources_file, &source_strings);
    }

    fn load_sources(sources_file: &str, settings: &Settings) -> HashSet<Source> {
        match file_read_lines(sources_file) {
            Ok(lines) => {
                let mut sources = HashSet::new();
                for line in lines {
                    let source = Source::new(line.as_str(), settings);
                    match source {
                        Ok(source_obj) => {
                            sources.insert(source_obj);
                        }
                        Err(_) => todo!(),
                    }
                }
                sources
            }
            Err(_) => HashSet::new(),
        }
    }

    pub fn automations(&self) -> HashMap<String, Automation> {
        let mut automations = HashMap::new();
        for source in self.sources.iter() {
            for automation in source.automations(&self.settings) {
                let key = format!("{}.{}", source.name(), automation.name);
                automations.insert(key, automation);
            }
        }
        automations
    }
}
