use std::process::Command;

use jayjay_core::{CliStatus, check_gh_environment, check_glab_environment, check_jj_environment};

use crate::app::config::AppConfig;
use crate::app::tools::{EDITOR_OPTIONS, TERMINAL_OPTIONS};
use gpui::{
    AnyElement, Context, InteractiveElement, IntoElement, MouseButton, MouseDownEvent,
    ParentElement, SharedString, Styled, div, px, rgb,
};

use super::SettingsView;
use super::shared::{current_value, field_row, row_container, section_title, subsection_title};
use crate::app::theme::Theme;
use crate::platform::{CUSTOM_TERMINAL_HINT, CUSTOM_TERMINAL_LABEL};
use crate::ui::icons::{self, glyph};

pub(super) fn tools_section(
    cfg: &AppConfig,
    t: &Theme,
    cx: &mut Context<SettingsView>,
) -> AnyElement {
    let mut section = div()
        .flex()
        .flex_col()
        .w_full()
        .gap(px(16.))
        .child(section_title("Tools", t))
        .child(field_row(
            "External editor",
            dropdown_button("editor", EDITOR_OPTIONS, &cfg.tools.external_editor, t, cx),
            "Used by 'Open in Editor' actions.",
            t,
        ));
    if cfg.tools.external_editor == "custom" {
        section = section.child(field_row(
            "Command",
            current_value(setting_value(&cfg.tools.custom_editor_command), t),
            "e.g. code, nvim",
            t,
        ));
    }
    section = section.child(field_row(
        "Terminal",
        dropdown_button("terminal", TERMINAL_OPTIONS, &cfg.tools.terminal, t, cx),
        "Used by 'Open in Terminal'.",
        t,
    ));
    if cfg.tools.terminal == "custom" {
        section = section.child(field_row(
            CUSTOM_TERMINAL_LABEL,
            current_value(setting_value(&cfg.tools.custom_terminal_command), t),
            CUSTOM_TERMINAL_HINT,
            t,
        ));
    }
    section
        .child(ai_tools(t))
        .child(cli_tools(t))
        .into_any_element()
}

fn setting_value(value: &str) -> &str {
    if value.is_empty() { "(none)" } else { value }
}

fn ai_tools(t: &Theme) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w_full()
        .gap(px(2.))
        .child(subsection_title("AI Commit Message", t))
        .child(binary_row(
            "Codex CLI",
            glyph::FILE_CODE,
            "codex",
            "Installed",
            "Not found",
            t,
        ))
        .child(binary_row(
            "Claude CLI",
            glyph::SPARKLE,
            "claude",
            "Installed",
            "Not found",
            t,
        ))
}

fn cli_tools(t: &Theme) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .w_full()
        .gap(px(2.))
        .child(subsection_title("CLI", t))
        .child(binary_row(
            "jayjay",
            glyph::INFO,
            "jayjay",
            "Installed",
            "Not installed",
            t,
        ))
        .child(cli_row("jj", glyph::GIT_BRANCH, check_jj_environment(), t))
        .child(cli_row("gh", glyph::GIT_MERGE, check_gh_environment(), t))
        .child(cli_row(
            "glab",
            glyph::GIT_MERGE,
            check_glab_environment(),
            t,
        ))
}

fn binary_row(
    name: &'static str,
    glyph_str: &'static str,
    command: &'static str,
    installed_label: &'static str,
    missing_label: &'static str,
    t: &Theme,
) -> impl IntoElement {
    status_row(
        name,
        glyph_str,
        if command_exists(command) {
            installed_label
        } else {
            missing_label
        },
        command_exists(command),
        t,
    )
}

fn command_exists(command: &str) -> bool {
    Command::new(command)
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
        || Command::new(command)
            .arg("version")
            .output()
            .is_ok_and(|output| output.status.success())
}

fn cli_row(
    name: &'static str,
    glyph_str: &'static str,
    status: CliStatus,
    t: &Theme,
) -> impl IntoElement {
    let detail = if status.is_installed {
        if status.path.is_empty() {
            format!("{name} {}", status.version)
        } else {
            status.path
        }
    } else {
        "Not installed".to_owned()
    };
    status_row(name, glyph_str, detail, status.is_installed, t)
}

fn status_row(
    name: &'static str,
    glyph_str: &'static str,
    detail: impl Into<SharedString>,
    installed: bool,
    t: &Theme,
) -> impl IntoElement {
    row_container(t)
        .debug_selector(move || format!("settings-tool-row-{name}"))
        .py(px(5.))
        .child(icons::icon(glyph_str, 14., t.fg_dim))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .truncate()
                .text_size(px(12.))
                .text_color(rgb(t.fg))
                .child(name),
        )
        .child(
            div()
                .flex_none()
                .max_w(px(360.))
                .min_w_0()
                .truncate()
                .font_family(crate::app::fonts::mono())
                .text_size(px(11.))
                .text_color(rgb(if installed { t.fg_dim } else { t.fg_faint }))
                .child(detail.into()),
        )
        .child(icons::icon(
            if installed {
                glyph::CHECK
            } else {
                glyph::X_CIRCLE
            },
            13.,
            if installed {
                t.tag_added_fg
            } else {
                t.fg_faint
            },
        ))
}

fn dropdown_button(
    field_id: &'static str,
    options: &'static [(&'static str, &'static str)],
    current: &str,
    t: &Theme,
    cx: &mut Context<SettingsView>,
) -> AnyElement {
    let label = options
        .iter()
        .find(|(id, _)| *id == current)
        .map(|(_, l)| *l)
        .unwrap_or(current);
    div()
        .id(SharedString::from(format!("dd-btn-{field_id}")))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(8.))
        .min_w(px(180.))
        .justify_between()
        .px(px(10.))
        .py(px(4.))
        .rounded_sm()
        .border_1()
        .border_color(rgb(t.border))
        .bg(rgb(t.toggle_inactive_bg))
        .text_size(px(11.))
        .text_color(rgb(t.fg))
        .cursor_pointer()
        .hover(|s| s.bg(rgb(t.row_alt_bg)))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |view, ev: &MouseDownEvent, _w, cx| {
                view.open_dropdown(SharedString::from(field_id), ev.position, cx);
            }),
        )
        .child(SharedString::from(label.to_owned()))
        .child(icons::icon(glyph::CARET_DOWN, 10., t.fg_faint))
        .into_any_element()
}
