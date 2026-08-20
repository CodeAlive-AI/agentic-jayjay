import JayJayCore

struct ChangeLineCounts: Equatable {
    var sourceInsertions: UInt32 = 0
    var sourceDeletions: UInt32 = 0
    var totalInsertions: UInt32 = 0
    var totalDeletions: UInt32 = 0

    var extraTotal: (insertions: UInt32, deletions: UInt32)? {
        if sourceInsertions == totalInsertions, sourceDeletions == totalDeletions {
            return nil
        }
        return (totalInsertions, totalDeletions)
    }

    var hasVisibleCounts: Bool {
        sourceInsertions > 0 || sourceDeletions > 0 || extraTotal != nil
    }

    static func from(fileStats: [FileDiffStats]) -> ChangeLineCounts {
        var counts = ChangeLineCounts()
        for file in fileStats {
            counts.totalInsertions += file.insertions
            counts.totalDeletions += file.deletions
            if SourcePath.isSource(file.path) {
                counts.sourceInsertions += file.insertions
                counts.sourceDeletions += file.deletions
            }
        }
        return counts
    }
}
