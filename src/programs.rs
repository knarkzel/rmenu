use std::path::PathBuf;
use walkdir::WalkDir;

pub struct Programs {
    pub paths: Vec<PathBuf>,
    pub binaries: Vec<String>,
}

impl Programs {
    pub fn new() -> Self {
        // obtain paths
        let mut paths = std::env::var("PATH")
            .expect("Failed to get $PATH")
            .split(':')
            .map(|entry| PathBuf::from(entry))
            .collect::<Vec<_>>();
        paths.sort();
        paths.dedup();

        // obtain unique binaries
        let mut temp_binaries = vec![];
        for path in paths.iter() {
            let temp = WalkDir::new(path)
                .into_iter()
                .filter_map(|entry| entry.ok())
                .map(|entry| {
                    entry
                        .path()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                })
                .collect::<Vec<_>>();
            temp_binaries.push(temp);
        }
        let binaries = temp_binaries.into_iter().flatten().collect::<Vec<_>>();

        Self { paths, binaries }
    }
    pub fn get_filtered_matches(&self, search: &str) -> Vec<&String> {
        self.binaries
            .iter()
            .filter(|entry| entry.contains(search))
            .collect::<Vec<_>>()
    }
}
