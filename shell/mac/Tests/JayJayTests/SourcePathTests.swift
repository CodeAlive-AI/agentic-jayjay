@testable import JayJay
import JayJayCore
import XCTest

final class SourcePathTests: XCTestCase {
    func testProgrammingPathsAreSource() {
        XCTAssertTrue(SourcePath.isSource("src/main.rs"))
        XCTAssertTrue(SourcePath.isSource("App.swift"))
        XCTAssertTrue(SourcePath.isSource("README.md"))
        XCTAssertTrue(SourcePath.isSource("Dockerfile"))
    }

    func testMediaAndJsonAreNotSource() {
        XCTAssertFalse(SourcePath.isSource("song.mp3"))
        XCTAssertFalse(SourcePath.isSource("clip.wav"))
        XCTAssertFalse(SourcePath.isSource("package-lock.json"))
        XCTAssertFalse(SourcePath.isSource("unknown.bin"))
    }

    func testLineCountsKeepJsonAsGrayExtra() {
        let counts = ChangeLineCounts.from(fileStats: [
            FileDiffStats(path: "lib.rs", insertions: 2, deletions: 0),
            FileDiffStats(path: "data.json", insertions: 40, deletions: 3),
        ])
        XCTAssertEqual(counts.sourceInsertions, 2)
        XCTAssertEqual(counts.extraTotal?.insertions, 42)
        XCTAssertEqual(counts.extraTotal?.deletions, 3)
    }
}
