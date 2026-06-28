use std::sync::Arc;

use gpui::{App, BorrowAppContext, Global};

use super::AppConfig;
use crate::app::theme;

/// Wrapper makes `AppConfig` registrable as a GPUI global. Fields are
/// accessed via `cx.global::<AppConfigStore>().config` and mutated via the
/// `update` helper below (which also persists to disk).
pub struct AppConfigStore {
    pub config: Arc<AppConfig>,
    persist: bool,
}

impl Global for AppConfigStore {}

impl AppConfigStore {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config: Arc::new(config),
            persist: true,
        }
    }

    /// Use for tests and previews that need global config state without writing the user's real config file.
    pub fn new_ephemeral(config: AppConfig) -> Self {
        Self {
            config: Arc::new(config),
            persist: false,
        }
    }
}

/// Read the current config from a `cx`. Pair with
/// `cx.observe_global::<AppConfigStore>(|_, cx| cx.notify())` in any view
/// that needs to re-render on changes.
pub fn current(cx: &App) -> Arc<AppConfig> {
    cx.global::<AppConfigStore>().config.clone()
}

/// Apply a mutation to the config; persist to disk and notify global
/// observers.
pub fn update<F>(cx: &mut App, mutate: F)
where
    F: FnOnce(&mut AppConfig),
{
    cx.update_global::<AppConfigStore, _>(|store, _| {
        let mut next = (*store.config).clone();
        mutate(&mut next);
        if store.persist
            && let Err(err) = next.save()
        {
            eprintln!("[jayjay-gpui] failed to save config: {err}");
        }
        store.config = Arc::new(next);
    });
    theme::refresh_for_current_appearance(cx);
    crate::app::menus::refresh(cx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::theme::Theme;

    #[gpui::test]
    fn ephemeral_updates_still_change_current_config(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            cx.set_global(AppConfigStore::new_ephemeral(AppConfig::default()));
            cx.set_global(Theme::light());

            update(cx, |c| c.diff.tree_file_list = true);

            assert!(current(cx).diff.tree_file_list);
        });
    }
}
