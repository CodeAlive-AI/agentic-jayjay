import SwiftUI

/// Compact source `+N` / `-M`, with gray raw totals in parentheses when extras exist.
struct DiffLineCounts: View {
    let insertions: UInt32
    let deletions: UInt32
    var totalInsertions: UInt32? = nil
    var totalDeletions: UInt32? = nil
    var size: CGFloat = 10

    init(counts: ChangeLineCounts, size: CGFloat = 10) {
        insertions = counts.sourceInsertions
        deletions = counts.sourceDeletions
        if let extra = counts.extraTotal {
            totalInsertions = extra.insertions
            totalDeletions = extra.deletions
        }
        self.size = size
    }

    init(insertions: UInt32, deletions: UInt32, size: CGFloat = 10) {
        self.insertions = insertions
        self.deletions = deletions
        self.size = size
    }

    var body: some View {
        if insertions > 0 || deletions > 0 || showsExtra {
            HStack(spacing: 4) {
                if insertions > 0 {
                    Text("+\(insertions)")
                        .jayjayFont(size, weight: .semibold, design: .monospaced)
                        .foregroundStyle(.green)
                }
                if deletions > 0 {
                    Text("-\(deletions)")
                        .jayjayFont(size, weight: .semibold, design: .monospaced)
                        .foregroundStyle(.red)
                }
                if showsExtra {
                    Text("(\(extraLabel))")
                        .jayjayFont(size, weight: .medium, design: .monospaced)
                        .foregroundStyle(.secondary)
                }
            }
        }
    }

    private var showsExtra: Bool {
        guard let totalInsertions, let totalDeletions else { return false }
        return totalInsertions != insertions || totalDeletions != deletions
    }

    private var extraLabel: String {
        let plus = totalInsertions ?? 0
        let minus = totalDeletions ?? 0
        switch (plus > 0, minus > 0) {
            case (true, true): return "+\(plus) -\(minus)"
            case (true, false): return "+\(plus)"
            case (false, true): return "-\(minus)"
            case (false, false): return "+0"
        }
    }
}
