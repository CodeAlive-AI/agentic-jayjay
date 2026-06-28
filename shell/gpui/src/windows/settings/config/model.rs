#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::windows::settings) struct JjConfigSnapshot {
    pub(super) path: String,
    pub(super) sections: Vec<JjConfigSection>,
    pub(super) error: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::windows::settings) struct JjConfigSection {
    pub(super) name: String,
    pub(super) entries: Vec<JjConfigEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::windows::settings) struct JjConfigEntry {
    pub(super) key: String,
    pub(super) value: String,
}

pub(super) fn parse_config_sections(raw: &str) -> Vec<JjConfigSection> {
    let mut grouped: Vec<JjConfigSection> = Vec::new();
    let mut current_name = String::new();
    let mut current_entries = Vec::new();

    for line in raw.lines() {
        let Some((full_key, value)) = line.split_once('=') else {
            continue;
        };
        let full_key = full_key.trim();
        let value = value.trim();
        let (section, key) = full_key.split_once('.').unwrap_or(("general", full_key));
        if section != current_name {
            if !current_entries.is_empty() {
                grouped.push(JjConfigSection {
                    name: std::mem::take(&mut current_name),
                    entries: std::mem::take(&mut current_entries),
                });
            }
            current_name = section.to_owned();
        }
        current_entries.push(JjConfigEntry {
            key: key.to_owned(),
            value: value.to_owned(),
        });
    }

    if !current_entries.is_empty() {
        grouped.push(JjConfigSection {
            name: current_name,
            entries: current_entries,
        });
    }

    grouped
}

#[cfg(test)]
mod tests {
    use super::parse_config_sections;

    #[test]
    fn parse_config_sections_groups_by_prefix() {
        let sections = parse_config_sections(
            "user.name = Alice\nuser.email = a@example.com\nui.diff = split\n",
        );

        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].name, "user");
        assert_eq!(sections[0].entries[0].key, "name");
        assert_eq!(sections[0].entries[1].value, "a@example.com");
        assert_eq!(sections[1].name, "ui");
        assert_eq!(sections[1].entries[0].value, "split");
    }
}
