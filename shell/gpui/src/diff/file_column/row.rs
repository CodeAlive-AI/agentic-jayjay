use gpui::{
    AnyElement, App, ClickEvent, InteractiveElement, IntoElement, ParentElement,
    StatefulInteractiveElement, Styled, Window, div, px, rgb,
};
use jayjay_core::DiffHunk;

use crate::app::theme::Theme;
use crate::diff::file_status;
use crate::ui::icons::{self, glyph};

pub(super) fn row_bg(is_selected: bool, _ix: usize, t: &Theme) -> u32 {
    if is_selected {
        t.selected_bg
    } else {
        t.sidebar_bg
    }
}

pub(super) fn file_name_opacity(show_review: bool, reviewed: bool) -> f32 {
    if show_review && reviewed { 0.5 } else { 1.0 }
}

pub(super) fn review_checkbox<FRev>(
    id: (&'static str, usize),
    reviewed: bool,
    t: &Theme,
    on_click: FRev,
) -> AnyElement
where
    FRev: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    let border = if reviewed {
        t.file_added_color
    } else {
        t.fg_faint
    };
    let mut checkbox = div()
        .id(id)
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .w(px(14.))
        .h(px(14.))
        .rounded_full()
        .border_1()
        .border_color(rgb(border))
        .cursor_pointer()
        .on_click(move |ev, w, cx| {
            cx.stop_propagation();
            on_click(ev, w, cx);
        });
    if reviewed {
        checkbox =
            checkbox
                .bg(rgb(t.file_added_color))
                .child(icons::icon(glyph::CHECK, 9., 0xffffff));
    }
    checkbox.into_any_element()
}

pub(super) fn status_dot(hunk: &DiffHunk, t: &Theme) -> impl IntoElement {
    div()
        .flex_none()
        .w(px(8.))
        .h(px(8.))
        .rounded_full()
        .bg(rgb(file_status::color(hunk, t)))
}

#[cfg(test)]
mod tests {
    use super::file_name_opacity;

    #[test]
    fn reviewed_style_only_dims_working_copy_file_names() {
        assert_eq!(file_name_opacity(true, true), 0.5);
        assert_eq!(file_name_opacity(false, true), 1.0);
        assert_eq!(file_name_opacity(true, false), 1.0);
    }
}
