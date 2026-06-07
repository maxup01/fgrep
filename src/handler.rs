use colored::Colorize;
use walkdir::WalkDir;

pub struct Handler;

impl Handler {
    pub fn run(root_dir: String, pattern: String, case_insensitive_matching: bool) {
        let mut pattern = pattern;

        if case_insensitive_matching {
            pattern = pattern.to_lowercase();
        }

        for entry in WalkDir::new(root_dir.as_str())
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if let Some(path_str) = entry.path().to_str() {
                Self::handle_matching(path_str, pattern.as_str(), case_insensitive_matching);
            }
        }
    }

    fn handle_matching(path: &str, pattern: &str, case_insensitive_matching: bool) {
        let mut path_to_check = path.to_string();

        if case_insensitive_matching {
            path_to_check = path_to_check.to_lowercase();
        }

        if let Some(index) = path_to_check.find(pattern) {
            println!("{}", Self::highlight(path, index, pattern.len()));
        }
    }

    fn highlight(s: &str, start: usize, len: usize) -> String {
        let before = &s[..start];
        let matched = &s[start..start + len];
        let after = &s[start + len..];

        format!("{}{}{}", before, matched.green().bold(), after)
    }
}
