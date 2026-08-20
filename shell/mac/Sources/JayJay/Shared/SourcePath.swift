import Foundation

enum SourcePath {
    static func isSource(_ path: String) -> Bool {
        let name = (path as NSString).lastPathComponent.lowercased()
        if SourcePathData.filenames.contains(name) {
            return true
        }
        var rest = name[...]
        while let dot = rest.firstIndex(of: ".") {
            if SourcePathData.extensions.contains(String(rest[dot...])) {
                return true
            }
            rest = rest[rest.index(after: dot)...]
        }
        return false
    }
}
