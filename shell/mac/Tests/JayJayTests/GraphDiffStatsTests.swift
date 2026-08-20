@testable import JayJay
import JayJayCore
import XCTest

@MainActor
final class GraphDiffStatsTests: RepoViewModelTestCase {
    func testEmptyWorkingCopyDoesNotSeedGraphDiffStats() async throws {
        let viewModel = try XCTUnwrap(viewModel)
        try await refresh(viewModel)

        let workingCopy = try XCTUnwrap(viewModel.graphEntries.first(where: { $0.change.isWorkingCopy }))
        XCTAssertTrue(workingCopy.change.isEmpty)
        viewModel.requestGraphDiffStats(for: workingCopy.change)
        XCTAssertNil(viewModel.graphDiffStats[workingCopy.change.commitId.id])
    }

    func testRefreshSeedsWorkingCopyLineCounts() async throws {
        let viewModel = try XCTUnwrap(viewModel)
        let file = URL(fileURLWithPath: viewModel.repoPath).appending(path: "added.txt")
        try "one\n".write(to: file, atomically: true, encoding: .utf8)
        try await refresh(viewModel)

        let workingCopy = try XCTUnwrap(viewModel.graphEntries.first(where: { $0.change.isWorkingCopy }))
        let stats = try await waitForStats(viewModel, commitId: workingCopy.change.commitId.id)
        XCTAssertEqual(stats.sourceInsertions, 1)
        XCTAssertEqual(stats.sourceDeletions, 0)
    }

    func testRequestLoadsStatsForAHistoricalChange() async throws {
        let viewModel = try XCTUnwrap(viewModel)
        let file = URL(fileURLWithPath: viewModel.repoPath).appending(path: "added.txt")
        try "one\n".write(to: file, atomically: true, encoding: .utf8)
        try await refresh(viewModel)

        let previousSignal = viewModel.successActionSignal
        viewModel.newChange(parent: "@")
        for _ in 0 ..< 100 where viewModel.successActionSignal == previousSignal {
            try await Task.sleep(for: .milliseconds(20))
        }
        try await refresh(viewModel)

        let described = try XCTUnwrap(
            viewModel.graphEntries.first(where: { !$0.change.isWorkingCopy && !$0.change.isEmpty })
        )
        viewModel.requestGraphDiffStats(for: described.change)
        let stats = try await waitForStats(viewModel, commitId: described.change.commitId.id)
        XCTAssertEqual(stats.sourceInsertions, 1)
        XCTAssertEqual(stats.sourceDeletions, 0)
    }

    func testRefreshDropsStatsForCommitsThatLeftTheGraph() async throws {
        let viewModel = try XCTUnwrap(viewModel)
        viewModel.graphDiffStats["deadbeef"] = ChangeLineCounts(
            sourceInsertions: 9,
            sourceDeletions: 3,
            totalInsertions: 9,
            totalDeletions: 3
        )
        try await refresh(viewModel)
        XCTAssertNil(viewModel.graphDiffStats["deadbeef"])
    }

    private func refresh(_ viewModel: RepoViewModel) async throws {
        viewModel.refresh()
        for _ in 0 ..< 100 where viewModel.isRefreshingInFlight {
            try await Task.sleep(for: .milliseconds(20))
        }
        XCTAssertFalse(viewModel.isRefreshingInFlight)
    }

    private func waitForStats(_ viewModel: RepoViewModel, commitId: String) async throws -> ChangeLineCounts {
        for _ in 0 ..< 100 {
            if let stats = viewModel.graphDiffStats[commitId] {
                return stats
            }
            try await Task.sleep(for: .milliseconds(20))
        }
        XCTFail("timed out waiting for graph diff stats")
        return ChangeLineCounts()
    }
}
