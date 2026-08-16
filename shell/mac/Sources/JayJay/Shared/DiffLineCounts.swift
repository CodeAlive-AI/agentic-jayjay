import SwiftUI

/// Compact `+N` / `-M` counts in the same green/red treatment as the detail header.
struct DiffLineCounts: View {
    let insertions: UInt32
    let deletions: UInt32
    var size: CGFloat = 10

    var body: some View {
        if insertions > 0 || deletions > 0 {
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
            }
        }
    }
}
