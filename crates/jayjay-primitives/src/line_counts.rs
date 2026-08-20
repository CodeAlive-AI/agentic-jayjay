use super::diff::FileDiffStats;
use super::source_path::is_source_path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LineCounts {
    pub insertions: u32,
    pub deletions: u32,
}

impl LineCounts {
    pub fn is_zero(self) -> bool {
        self.insertions == 0 && self.deletions == 0
    }

    fn add(&mut self, insertions: u32, deletions: u32) {
        self.insertions = self.insertions.saturating_add(insertions);
        self.deletions = self.deletions.saturating_add(deletions);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ChangeLineCounts {
    pub source: LineCounts,
    pub total: LineCounts,
}

impl ChangeLineCounts {
    pub fn from_path_stats<I, S>(files: I) -> Self
    where
        I: IntoIterator<Item = (S, u32, u32)>,
        S: AsRef<str>,
    {
        let mut counts = Self::default();
        for (path, insertions, deletions) in files {
            counts.total.add(insertions, deletions);
            if is_source_path(path.as_ref()) {
                counts.source.add(insertions, deletions);
            }
        }
        counts
    }

    pub fn from_file_stats(files: &[FileDiffStats]) -> Self {
        Self::from_path_stats(
            files
                .iter()
                .map(|file| (file.path.as_str(), file.insertions, file.deletions)),
        )
    }

    pub fn extra_total(self) -> Option<LineCounts> {
        (self.source != self.total).then_some(self.total)
    }
}

#[cfg(test)]
mod tests {
    use super::{ChangeLineCounts, LineCounts};

    #[test]
    fn source_only_change_hides_extra_total() {
        let counts = ChangeLineCounts::from_path_stats([("lib.rs", 4, 1)]);
        assert_eq!(
            counts.source,
            LineCounts {
                insertions: 4,
                deletions: 1
            }
        );
        assert_eq!(counts.extra_total(), None);
    }

    #[test]
    fn json_extra_keeps_source_separate() {
        let counts = ChangeLineCounts::from_path_stats([("lib.rs", 2, 0), ("data.json", 40, 3)]);
        assert_eq!(
            counts.source,
            LineCounts {
                insertions: 2,
                deletions: 0
            }
        );
        assert_eq!(
            counts.extra_total(),
            Some(LineCounts {
                insertions: 42,
                deletions: 3
            })
        );
    }

    #[test]
    fn media_with_zero_lines_does_not_create_extras() {
        let counts = ChangeLineCounts::from_path_stats([("main.swift", 8, 2), ("beep.wav", 0, 0)]);
        assert_eq!(counts.extra_total(), None);
        assert_eq!(counts.source.insertions, 8);
    }

    #[test]
    fn data_only_change_is_gray_total() {
        let counts = ChangeLineCounts::from_path_stats([("lock.json", 12, 0)]);
        assert!(counts.source.is_zero());
        assert_eq!(
            counts.extra_total(),
            Some(LineCounts {
                insertions: 12,
                deletions: 0
            })
        );
    }
}
