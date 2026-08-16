use gpui::Context;
use jayjay_core::{ChangeInfo, DiffStats};

use crate::repo::view_model::RepoViewModel;

impl RepoViewModel {
    pub(in crate::repo) fn ensure_graph_diff_stats(
        &mut self,
        change: &ChangeInfo,
        cx: &mut Context<Self>,
    ) {
        if change.is_empty {
            return;
        }
        let commit_id = change.commit_id.id.clone();
        if let Some(stats) = self.graph_diff_stats.get(&commit_id).cloned() {
            self.apply_graph_diff_stats_to_selection(&commit_id, &stats);
            return;
        }
        if self.graph_diff_stats_in_flight.as_deref() == Some(commit_id.as_str()) {
            return;
        }
        if self
            .graph_diff_stats_queue
            .iter()
            .any(|(id, _)| id == &commit_id)
        {
            return;
        }
        self.graph_diff_stats_queue
            .push_back((commit_id, change.commit_id.id.clone()));
        self.pump_graph_diff_stats(cx);
    }

    pub(in crate::repo) fn retain_graph_diff_stats(&mut self) {
        let live: std::collections::HashSet<&str> = self
            .graph
            .changes
            .iter()
            .map(|change| change.commit_id.id.as_str())
            .collect();
        self.graph_diff_stats
            .retain(|commit_id, _| live.contains(commit_id.as_str()));
        self.graph_diff_stats_queue
            .retain(|(commit_id, _)| live.contains(commit_id.as_str()));
    }

    pub(in crate::repo) fn seed_working_copy_graph_diff_stats(&mut self) {
        let Some(stats) = self.working_copy_stats.clone() else {
            return;
        };
        if stats.insertions == 0 && stats.deletions == 0 && stats.files_changed == 0 {
            return;
        }
        let Some(working_copy) = self.working_copy_change() else {
            return;
        };
        self.store_graph_diff_stats(working_copy.commit_id.id.clone(), stats);
    }

    fn store_graph_diff_stats(&mut self, commit_id: String, stats: DiffStats) {
        if !self
            .graph
            .changes
            .iter()
            .any(|change| change.commit_id.id == commit_id)
        {
            return;
        }
        self.apply_graph_diff_stats_to_selection(&commit_id, &stats);
        self.graph_diff_stats.insert(commit_id, stats);
    }

    fn apply_graph_diff_stats_to_selection(&mut self, commit_id: &str, stats: &DiffStats) {
        if self
            .selected_change()
            .is_some_and(|change| change.commit_id.id == commit_id)
        {
            self.change_stats = Some(stats.clone());
        }
    }

    fn pump_graph_diff_stats(&mut self, cx: &mut Context<Self>) {
        if self.graph_diff_stats_in_flight.is_some() {
            return;
        }
        let Some((commit_id, rev)) = self.graph_diff_stats_queue.pop_front() else {
            return;
        };
        let Some(repo) = self.repo.clone() else {
            return;
        };
        self.graph_diff_stats_in_flight = Some(commit_id.clone());
        Self::background_update(
            cx,
            async move { (commit_id, repo.diff_stats(&rev).ok()) },
            move |vm, (commit_id, stats), cx| {
                vm.graph_diff_stats_in_flight = None;
                if let Some(stats) = stats {
                    vm.store_graph_diff_stats(commit_id, stats);
                }
                vm.pump_graph_diff_stats(cx);
                cx.notify();
            },
        );
    }
}
