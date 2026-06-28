use crate::app::config::{self, AppConfig};
use crate::app::theme::Theme;
use crate::ui::icons::{self, LOGO_SVG, glyph};
use gpui::{
    ClickEvent, InteractiveElement, IntoElement, ParentElement, SharedString,
    StatefulInteractiveElement, Styled, div, img, px, rgb,
};

const APP_NAME: &str = "JayJay";
const TAGLINE: &str = "A native GUI for Jujutsu";
const SPONSOR_URL: &str = "https://github.com/sponsors/hewigovens";
const GITHUB_URL: &str = "https://github.com/hewigovens/jayjay";

pub(super) fn about_section(cfg: &AppConfig, t: &Theme) -> impl IntoElement {
    let version = format!("Version {} (GPUI Alpha)", env!("CARGO_PKG_VERSION"));

    div()
        .flex()
        .flex_col()
        .items_center()
        .gap(px(12.))
        .pt(px(8.))
        .child(app_icon())
        .child(
            div()
                .text_size(px(20.))
                .text_color(rgb(t.fg))
                .child(APP_NAME),
        )
        .child(
            div()
                .text_size(px(12.))
                .text_color(rgb(t.fg_dim))
                .child(TAGLINE),
        )
        .child(
            div()
                .text_size(px(11.))
                .text_color(rgb(t.fg_faint))
                .child(SharedString::from(version)),
        )
        .child(about_toggle(
            "Send anonymous usage stats",
            cfg.telemetry.enabled,
            t,
        ))
        .child(
            div()
                .flex()
                .flex_row()
                .gap(px(8.))
                .pt(px(8.))
                .child(link_button(
                    "tb-sponsor",
                    glyph::SPARKLE,
                    "Sponsor",
                    SPONSOR_URL,
                    t,
                ))
                .child(link_button(
                    "tb-github",
                    glyph::ARROW_CIRCLE_RIGHT,
                    "Star on GitHub",
                    GITHUB_URL,
                    t,
                )),
        )
}

fn app_icon() -> impl IntoElement {
    img(LOGO_SVG).w(px(72.)).h(px(72.)).rounded_lg()
}

fn link_button(
    id: &'static str,
    glyph_str: &'static str,
    label: &'static str,
    url: &'static str,
    t: &Theme,
) -> impl IntoElement {
    div()
        .id(SharedString::from(id))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(6.))
        .px(px(14.))
        .py(px(6.))
        .rounded_md()
        .bg(rgb(t.toggle_inactive_bg))
        .text_size(px(12.))
        .text_color(rgb(t.toggle_inactive_fg))
        .cursor_pointer()
        .hover(|s| s.bg(rgb(t.row_alt_bg)))
        .on_click(move |_: &ClickEvent, _, cx| {
            cx.open_url(url);
        })
        .child(icons::icon(glyph_str, 12., t.toggle_inactive_fg))
        .child(label)
}

fn about_toggle(label: &'static str, active: bool, t: &Theme) -> impl IntoElement {
    let (bg, fg, icon) = if active {
        (t.toggle_active_bg, t.toggle_active_fg, glyph::CHECK)
    } else {
        (t.toggle_inactive_bg, t.toggle_inactive_fg, glyph::DOT)
    };
    div()
        .id(SharedString::from("about-telemetry"))
        .debug_selector(|| "about-telemetry".to_owned())
        .flex()
        .flex_row()
        .items_center()
        .gap(px(8.))
        .pt(px(6.))
        .text_size(px(12.))
        .text_color(rgb(t.fg_dim))
        .child(label)
        .child(
            div()
                .id(SharedString::from("about-telemetry-toggle"))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(6.))
                .px(px(10.))
                .py(px(3.))
                .rounded_sm()
                .bg(rgb(bg))
                .text_color(rgb(fg))
                .cursor_pointer()
                .on_click(|_: &ClickEvent, _w, cx| {
                    config::update(cx, |c| c.telemetry.enabled ^= true);
                })
                .child(icons::icon(icon, 12., fg))
                .child(if active { "On" } else { "Off" }),
        )
}
