import AppKit

enum ProjectOpenPanel {
    @MainActor
    static func chooseProject() -> URL? {
        let panel = NSOpenPanel()
        panel.canChooseFiles = true
        panel.canChooseDirectories = false
        panel.allowsMultipleSelection = false
        panel.allowsOtherFileTypes = true
        return panel.runModal() == .OK ? panel.url : nil
    }
}
