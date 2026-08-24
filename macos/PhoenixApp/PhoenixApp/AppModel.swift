import Foundation

@MainActor
final class AppModel: ObservableObject {
    enum State: Equatable {
        case starting
        case ready(contractVersion: UInt32)
        case failed(message: String)
    }

    enum ProjectState: Equatable {
        case idle
        case inspecting
        case inspected(InspectionSummary)
        case failed(message: String)
    }

    @Published private(set) var state: State = .starting
    @Published private(set) var projectState: ProjectState = .idle
    private let core = PhoenixCore()
    private var started = false

    init() {
        startHandshake()
    }

    private func startHandshake() {
        guard !started else { return }
        started = true
        Task {
            do {
                let version = try await core.handshake()
                state = .ready(contractVersion: version)
                FileHandle.standardError.write(Data("UI1A_READY contract_version=\(version)\n".utf8))
            } catch {
                state = .failed(message: error.localizedDescription)
                FileHandle.standardError.write(Data("UI1A_FAILED \(error.localizedDescription)\n".utf8))
            }
        }
    }

    var canOpenProject: Bool {
        if case .ready = state { return projectState != .inspecting }
        return false
    }

    func openProject() {
        guard canOpenProject, let url = ProjectOpenPanel.chooseProject() else { return }
        projectState = .inspecting
        let path = url.path
        Task {
            do {
                let summary = try await core.inspectProject(path: path)
                projectState = .inspected(summary)
                FileHandle.standardError.write(Data("UI1B_INSPECTED sequences=\(summary.sequenceCount)\n".utf8))
            } catch {
                projectState = .failed(message: error.localizedDescription)
                FileHandle.standardError.write(Data("UI1B_INSPECTION_FAILED\n".utf8))
            }
        }
    }
}
