mod harness;

use gpui::TestAppContext;
use harness::*;
use jayjay_core::DiffStats;
use jj_test::{LinearFixture, run_jj_in};

#[gpui::test]
fn working_copy_refresh_seeds_graph_line_counts(cx: &mut TestAppContext) {
    let fixture = LinearFixture::build();
    let (view, cx) = open_fixture(&fixture, cx);

    let stats = view.read_with(cx, |view, cx| {
        let vm = view.view_model().read(cx);
        let working_copy = vm
            .graph
            .changes
            .iter()
            .find(|change| change.is_working_copy)
            .expect("working copy row");
        vm.graph_diff_stats
            .get(&working_copy.commit_id.id)
            .cloned()
            .expect("working copy stats seeded from refresh")
    });
    assert_eq!(stats.insertions, 2);
    assert_eq!(stats.deletions, 0);
}

#[gpui::test]
fn selecting_a_parent_fills_graph_line_counts(cx: &mut TestAppContext) {
    let fixture = LinearFixture::build();
    let (view, cx) = open_fixture(&fixture, cx);

    let parent_ix = view.read_with(cx, |view, cx| {
        view.view_model()
            .read(cx)
            .graph
            .changes
            .iter()
            .position(|change| change.description.starts_with("add feature"))
            .expect("add feature change")
    });
    view.update_in(cx, |view, _, cx| {
        view.view_model()
            .update(cx, |vm, cx| vm.select_change(parent_ix, cx));
    });
    settle_visual(cx);

    let stats = view.read_with(cx, |view, cx| {
        let vm = view.view_model().read(cx);
        let change = &vm.graph.changes[parent_ix];
        vm.graph_diff_stats
            .get(&change.commit_id.id)
            .cloned()
            .expect("selected parent stats")
    });
    assert_eq!(stats.insertions, 1);
    assert_eq!(stats.deletions, 0);
}

#[gpui::test]
fn empty_working_copy_is_not_cached(cx: &mut TestAppContext) {
    let fixture = LinearFixture::build();
    run_jj_in(&fixture.path, &["new", "-m", "empty next"]);
    let (view, cx) = open_fixture(&fixture, cx);

    view.read_with(cx, |view, cx| {
        let vm = view.view_model().read(cx);
        let working_copy = vm
            .graph
            .changes
            .iter()
            .find(|change| change.is_working_copy)
            .expect("working copy row");
        assert!(
            working_copy.is_empty,
            "new empty change should be marked empty"
        );
        assert!(
            !vm.graph_diff_stats.contains_key(&working_copy.commit_id.id),
            "empty changes must not occupy the stats cache"
        );
    });
}

#[gpui::test]
fn refresh_drops_stats_for_commits_that_left_the_graph(cx: &mut TestAppContext) {
    let fixture = LinearFixture::build();
    let (view, cx) = open_fixture(&fixture, cx);

    view.update_in(cx, |view, _, _cx| {
        view.view_model().update(_cx, |vm, _| {
            vm.graph_diff_stats.insert(
                "deadbeef".into(),
                DiffStats {
                    files_changed: 1,
                    insertions: 9,
                    deletions: 3,
                },
            );
        });
    });
    view.update_in(cx, |view, _, cx| {
        view.view_model().update(cx, |vm, cx| vm.refresh(false, cx));
    });
    settle_visual(cx);

    view.read_with(cx, |view, cx| {
        let vm = view.view_model().read(cx);
        assert!(
            !vm.graph_diff_stats.contains_key("deadbeef"),
            "stale commit ids must not survive a graph refresh"
        );
    });
}
