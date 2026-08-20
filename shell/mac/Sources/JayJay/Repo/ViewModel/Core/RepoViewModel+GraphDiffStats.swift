import JayJayCore

extension RepoViewModel {
    func requestGraphDiffStats(for change: ChangeInfo) {
        let commitId = change.commitId.id
        guard !change.isEmpty else { return }
        if graphDiffStats[commitId] != nil { return }
        if graphDiffStatsInFlight == commitId { return }
        if graphDiffStatsQueued.contains(where: { $0.commitId == commitId }) { return }
        graphDiffStatsQueued.append((commitId: commitId, rev: commitId))
        pumpGraphDiffStats()
    }

    func storeGraphDiffStats(commitId: String, stats: ChangeLineCounts) {
        guard graphEntries.contains(where: { $0.change.commitId.id == commitId }) else { return }
        graphDiffStats[commitId] = stats
    }

    func retainGraphDiffStats() {
        let live = Set(graphEntries.map(\.change.commitId.id))
        graphDiffStats = graphDiffStats.filter { live.contains($0.key) }
        graphDiffStatsQueued.removeAll { !live.contains($0.commitId) }
    }

    func seedWorkingCopyGraphDiffStats() {
        guard let workingCopy = graphEntries.first(where: { $0.change.isWorkingCopy }),
              !workingCopy.change.isEmpty
        else { return }
        requestGraphDiffStats(for: workingCopy.change)
    }

    func pumpGraphDiffStats() {
        guard graphDiffStatsTask == nil else { return }
        guard !graphDiffStatsQueued.isEmpty else { return }
        let next = graphDiffStatsQueued.removeFirst()
        graphDiffStatsInFlight = next.commitId
        graphDiffStatsTask = Task.detached { [repo] in
            let counts = (try? repo.diffFileStats(rev: next.rev, ignoreWhitespace: false))
                .map(ChangeLineCounts.from(fileStats:))
            await MainActor.run { [weak self] in
                guard let self else { return }
                if let counts {
                    self.storeGraphDiffStats(commitId: next.commitId, stats: counts)
                }
                self.graphDiffStatsInFlight = nil
                self.graphDiffStatsTask = nil
                self.pumpGraphDiffStats()
            }
        }
    }
}
