use itertools::Itertools;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn get_programs() -> Vec<String> {
    // obtain unique paths
    let paths = {
        let mut temp = std::env::var("PATH")
            .expect("Failed to get $PATH")
            .split(':')
            .map(|entry| PathBuf::from(entry))
            .collect_vec();
        temp.sort_unstable();
        temp.dedup();
        temp
    };

    // obtain unique binaries
    let mut binaries = paths
        .into_iter()
        .map(|path| {
            WalkDir::new(path)
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
                .collect_vec()
        })
        .collect_vec();
    binaries.sort_unstable();
    binaries.dedup();
    binaries.into_iter().flatten().collect_vec()
}
