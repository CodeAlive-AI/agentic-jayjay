use std::fs;

use jayjay_core::Repo;
use jj_test::{init_jj_repo, run_jj_in};

#[test]
fn graph_line_stats_separates_source_from_json_data() {
    let temp_dir = init_jj_repo();
    let repo_path = temp_dir.path().join("repo");
    run_jj_in(&repo_path, &["new", "-m", "mixed"]);
    let repo = Repo::open(&repo_path).expect("open repo");

    fs::write(repo_path.join("main.rs"), "fn main() {}\n").expect("write rust");
    fs::write(
        repo_path.join("blob.json"),
        "{\n  \"a\": 1,\n  \"b\": 2\n}\n",
    )
    .expect("write json");
    repo.refresh_working_copy().expect("snapshot");

    let counts = repo.graph_line_stats("@").expect("graph line stats");
    assert_eq!(counts.source.insertions, 1, "{counts:?}");
    assert_eq!(counts.source.deletions, 0, "{counts:?}");
    assert!(
        counts.total.insertions > counts.source.insertions,
        "{counts:?}"
    );
    assert_eq!(
        counts.extra_total().map(|extra| extra.insertions),
        Some(counts.total.insertions)
    );
}
