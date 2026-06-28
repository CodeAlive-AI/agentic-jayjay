use gpui::{
    AnyElement, App, ClickEvent, Context, InteractiveElement, IntoElement, ParentElement,
    SharedString, StatefulInteractiveElement, Styled, Window, div, px, rgb,
};

use crate::app::config;
use crate::app::theme::{FONT_META, Theme};
use crate::repo::window::RepoWindow;
use crate::ui::icons::{self, glyph};
use crate::ui::primitives::toggle_button;

pub(super) fn file_column_header(
    reviewed: usize,
    count: usize,
    loading: bool,
    show_review: bool,
    hide_reviewed: bool,
    tree_mode: bool,
    cx: &mut Context<RepoWindow>,
    t: &Theme,
) -> impl IntoElement {
    let label = if loading {
        String::from("Loading…")
    } else if count == 0 {
        String::from("0 files")
    } else if show_review {
        format!("{reviewed} / {count} reviewed")
    } else {
        format!("{count} files")
    };
    let (tree_glyph, tree_label) = if tree_mode {
        (glyph::FOLDER, "Tree")
    } else {
        (glyph::ROWS, "Flat")
    };
    let mut row = div()
        .flex()
        .flex_row()
        .items_center()
        .px(px(12.))
        .py(px(6.))
        .bg(rgb(t.header_bg))
        .border_b_1()
        .border_color(rgb(t.border))
        .child(
            div()
                .text_size(px(FONT_META))
                .text_color(rgb(t.fg_dim))
                .child(SharedString::from(label)),
        )
        .child(div().flex_1());

    if show_review && reviewed > 0 {
        row = row.child(icon_toggle_button(
            "file-hide-reviewed",
            if hide_reviewed {
                glyph::EYE_OFF
            } else {
                glyph::EYE
            },
            hide_reviewed,
            t,
            cx.listener(|view, _event: &ClickEvent, _window, cx| {
                view.toggle_hide_reviewed_files(cx);
            }),
        ));
    }

    row.child(toggle_button(
        tree_glyph,
        tree_label,
        "file-tree",
        tree_mode,
        t,
        |_, _, cx| config::update(cx, |c| c.diff.tree_file_list ^= true),
    ))
}

fn icon_toggle_button<F>(
    id: &'static str,
    glyph_str: &'static str,
    active: bool,
    t: &Theme,
    on_click: F,
) -> AnyElement
where
    F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    let (bg, fg) = if active {
        (t.toggle_active_bg, t.toggle_active_fg)
    } else {
        (t.toggle_inactive_bg, t.toggle_inactive_fg)
    };
    div()
        .id(SharedString::from(id))
        .debug_selector(move || id.to_owned())
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .w(px(24.))
        .h(px(22.))
        .mr(px(6.))
        .rounded_sm()
        .bg(rgb(bg))
        .cursor_pointer()
        .hover(|s| s.bg(rgb(t.row_alt_bg)))
        .on_click(on_click)
        .child(icons::icon(glyph_str, 13., fg))
        .into_any_element()
}
