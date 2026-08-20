use std::collections::HashSet;
use std::sync::OnceLock;

fn extensions() -> &'static HashSet<&'static str> {
    static TABLE: OnceLock<HashSet<&str>> = OnceLock::new();
    TABLE.get_or_init(|| load(include_str!("source_path/extensions.txt")))
}

fn filenames() -> &'static HashSet<&'static str> {
    static TABLE: OnceLock<HashSet<&str>> = OnceLock::new();
    TABLE.get_or_init(|| load(include_str!("source_path/filenames.txt")))
}

fn load(raw: &'static str) -> HashSet<&'static str> {
    raw.lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect()
}

/// Linguist programming/markup/prose path. Data files and unclassified media are excluded.
pub fn is_source_path(path: &str) -> bool {
    let name = path.rsplit(['/', '\\']).next().unwrap_or(path);
    let lower = name.to_ascii_lowercase();
    if filenames().contains(lower.as_str()) {
        return true;
    }
    let mut rest = lower.as_str();
    while let Some(dot) = rest.find('.') {
        if extensions().contains(&rest[dot..]) {
            return true;
        }
        rest = &rest[dot + 1..];
    }
    false
}

#[cfg(test)]
mod tests {
    use super::is_source_path;

    #[test]
    fn programming_and_markup_are_source() {
        assert!(is_source_path("src/main.rs"));
        assert!(is_source_path("App.swift"));
        assert!(is_source_path("dir\\view.ts"));
        assert!(is_source_path("index.html"));
        assert!(is_source_path("README.md"));
        assert!(is_source_path("Dockerfile"));
        assert!(is_source_path("CMakeLists.txt"));
    }

    #[test]
    fn data_and_media_are_not_source() {
        assert!(!is_source_path("song.mp3"));
        assert!(!is_source_path("clip.wav"));
        assert!(!is_source_path("photo.png"));
        assert!(!is_source_path("package-lock.json"));
        assert!(!is_source_path("data.yaml"));
        assert!(!is_source_path("notes.csv"));
        assert!(!is_source_path("unknown.bin"));
        assert!(!is_source_path("random_blob"));
    }

    #[test]
    fn longest_suffix_wins_for_compound_extensions() {
        assert!(is_source_path("page.blade.php"));
        assert!(is_source_path("config.h.in"));
    }
}
