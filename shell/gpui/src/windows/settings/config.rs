use std::path::Path;
use std::process::Command;
use std::sync::OnceLock;

mod model;

use gpui::{
    AnyElement, App, ClickEvent, ClipboardItem, InteractiveElement, IntoElement, ParentElement,
    SharedString, StatefulInteractiveElement, Styled, div, px, rgb,
};
use jayjay_core::check_jj_environment;

use super::shared::{row_container, section_title};
use crate::app::theme::Theme;
use crate::ui::icons::{self, glyph};
pub(super) use model::JjConfigSnapshot;
use model::{JjConfigEntry, JjConfigSection, parse_config_sections};

static JJ_CONFIG_CACHE: OnceLock<JjConfigSnapshot> = OnceLock::new();

pub(super) fn load_jj_config_snapshot() -> JjConfigSnapshot {
    JJ_CONFIG_CACHE.get_or_init(load_jj_config).clone()
}

pub(super) fn jujutsu_section(
    snapshot: Option<&JjConfigSnapshot>,
    loading: bool,
    t: &Theme,
) -> AnyElement {
    let mut root = div()
        .debug_selector(|| "settings-jujutsu-section".to_owned())
        .flex()
        .flex_col()
        .w_full()
        .gap(px(16.))
        .child(section_title("Jujutsu", t));

    if loading {
        return root
            .child(status_message("Loading jj config...", t))
            .into_any_element();
    }
    let Some(snapshot) = snapshot else {
        return root
            .child(status_message("jj config has not been loaded.", t))
            .into_any_element();
    };

    if let Some(error) = snapshot.error.as_ref() {
        return root
            .child(status_message(error.as_str(), t))
            .into_any_element();
    }

    if !snapshot.path.is_empty() {
        root = root.child(config_path_row(&snapshot.path, t));
    }
    for section in &snapshot.sections {
        root = root.child(config_section(section, t));
    }
    root.into_any_element()
}

fn config_path_row(path: &str, t: &Theme) -> AnyElement {
    row_container(t)
        .debug_selector(|| "jj-config-path-row".to_owned())
        .py(px(6.))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .truncate()
                .font_family(crate::app::fonts::mono())
                .text_size(px(11.))
                .text_color(rgb(t.fg_dim))
                .child(SharedString::from(path.to_owned())),
        )
        .child(open_button(path.to_owned(), t))
        .child(copy_button(path.to_owned(), t))
        .into_any_element()
}

fn config_section(section: &JjConfigSection, t: &Theme) -> AnyElement {
    let mut group = div().flex().flex_col().w_full().gap(px(4.)).child(
        div()
            .w_full()
            .pt(px(4.))
            .text_size(px(11.))
            .text_color(rgb(t.fg_faint))
            .child(SharedString::from(section.name.clone())),
    );
    for entry in &section.entries {
        group = group.child(config_row(entry, t));
    }
    group.into_any_element()
}

fn config_row(entry: &JjConfigEntry, t: &Theme) -> AnyElement {
    row_container(t)
        .debug_selector(|| "jj-config-row".to_owned())
        .py(px(5.))
        .child(icons::icon(entry_icon(&entry.key), 14., t.fg_dim))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .truncate()
                .text_size(px(12.))
                .text_color(rgb(t.fg))
                .child(SharedString::from(entry.key.clone())),
        )
        .child(
            div()
                .flex_none()
                .max_w(px(360.))
                .min_w_0()
                .truncate()
                .font_family(crate::app::fonts::mono())
                .text_size(px(12.))
                .text_color(rgb(t.fg_dim))
                .child(SharedString::from(entry.value.clone())),
        )
        .into_any_element()
}

fn status_message(message: &str, t: &Theme) -> AnyElement {
    div()
        .debug_selector(|| "jj-config-status".to_owned())
        .w_full()
        .px(px(8.))
        .py(px(8.))
        .rounded_sm()
        .bg(rgb(t.row_alt_bg))
        .text_size(px(12.))
        .text_color(rgb(t.fg_dim))
        .child(SharedString::from(message.to_owned()))
        .into_any_element()
}

fn open_button(path: String, t: &Theme) -> AnyElement {
    div()
        .id(SharedString::from("jj-config-open"))
        .px(px(10.))
        .py(px(4.))
        .rounded_sm()
        .bg(rgb(t.toggle_inactive_bg))
        .text_size(px(11.))
        .text_color(rgb(t.toggle_inactive_fg))
        .cursor_pointer()
        .hover(|style| style.bg(rgb(t.row_alt_bg)))
        .on_click(move |_: &ClickEvent, _, cx: &mut App| {
            let cwd = Path::new(&path)
                .parent()
                .and_then(Path::to_str)
                .unwrap_or(".");
            if !crate::app::tools::open_in_editor(cwd, &path, cx) {
                cx.open_url(&format!("file://{path}"));
            }
        })
        .child("Open")
        .into_any_element()
}

fn copy_button(value: String, t: &Theme) -> AnyElement {
    div()
        .id(SharedString::from("jj-config-copy-path"))
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .w(px(24.))
        .h(px(20.))
        .rounded_sm()
        .cursor_pointer()
        .text_color(rgb(t.fg_faint))
        .on_click(move |_: &ClickEvent, _, cx| {
            cx.write_to_clipboard(ClipboardItem::new_string(value.clone()));
        })
        .child(icons::icon(glyph::COPY, 12., t.fg_faint))
        .into_any_element()
}

fn load_jj_config() -> JjConfigSnapshot {
    let status = check_jj_environment();
    if !status.is_installed {
        return JjConfigSnapshot {
            path: String::new(),
            sections: Vec::new(),
            error: Some("jj is not installed.".to_owned()),
        };
    }
    let binary = if status.path.is_empty() {
        "jj"
    } else {
        status.path.as_str()
    };
    let raw = run(binary, &["config", "list"]);
    let path = run(binary, &["config", "path", "--user"]);
    JjConfigSnapshot {
        path,
        sections: parse_config_sections(&raw),
        error: None,
    }
}

fn run(binary: &str, args: &[&str]) -> String {
    Command::new(binary)
        .args(args)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .unwrap_or_default()
}

fn entry_icon(key: &str) -> &'static str {
    match key {
        "name" => glyph::INFO,
        "email" => glyph::TAG,
        "hostname" => glyph::TERMINAL,
        "username" => glyph::INFO,
        "backend" => glyph::PACKAGE,
        "behavior" => glyph::PENCIL_CIRCLE,
        "key" => glyph::GEAR,
        _ if key.contains("command") => glyph::TERMINAL,
        _ if key.contains("pattern") => glyph::SEARCH,
        _ if key.contains("sign") => glyph::PENCIL_CIRCLE,
        _ => glyph::GEAR,
    }
}
