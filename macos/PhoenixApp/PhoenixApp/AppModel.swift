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
        case inspected(ProjectInspection)
        case failed(message: String)
    }

    enum DiagnosticsState: Equatable {
        case notLoaded
        case loading
        case loaded(DiagnosticsSummary)
        case failed(message: String)
    }

    enum ExportState: Equatable {
        case idle
        case exporting
        case succeeded(ExportSequenceResult)
        case failed(message: String)
    }

    @Published private(set) var state: State = .starting
    @Published private(set) var projectState: ProjectState = .idle
    @Published var selectedSequenceID: String? {
        didSet {
            if selectedSequenceID != oldValue {
                resetExportState()
            }
        }
    }
    @Published private(set) var diagnosticsState: DiagnosticsState = .notLoaded
    @Published private(set) var exportState: ExportState = .idle
    private let core = PhoenixCore()
    private var started = false
    private var currentExportAttemptID: UUID?

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
        selectedSequenceID = nil
        projectState = .inspecting
        let path = url.path
        Task {
            do {
                let summary = try await core.inspectProject(path: path)
                selectedSequenceID = nil
                diagnosticsState = .notLoaded
                resetExportState()
                projectState = .inspected(summary)
                FileHandle.standardError.write(Data("UI1B_INSPECTED sequences=\(summary.sequenceCount)\n".utf8))
            } catch {
                projectState = .failed(message: error.localizedDescription)
                FileHandle.standardError.write(Data("UI1B_INSPECTION_FAILED\n".utf8))
            }
        }
    }

    func loadDiagnosticsIfNeeded() {
        guard case .notLoaded = diagnosticsState,
              case .inspected(let inspection) = projectState,
              inspection.diagnosticsAvailable else { return }
        diagnosticsState = .loading
        let sessionID = inspection.sessionID
        Task {
            do {
                let summary = try await core.getDiagnostics(sessionID: sessionID)
                guard case .inspected(let current) = projectState,
                      current.sessionID == sessionID else { return }
                diagnosticsState = .loaded(summary)
            } catch {
                guard case .inspected(let current) = projectState,
                      current.sessionID == sessionID else { return }
                diagnosticsState = .failed(message: error.localizedDescription)
            }
        }
    }

    func exportSelectedSequence() {
        guard exportState != .exporting else { return }
        beginExportIfPossible()
    }

    private func beginExportIfPossible() {
        guard case .inspected(let inspection) = projectState,
              let sequenceID = selectedSequenceID,
              let sequence = inspection.sequences.first(where: { $0.sequenceID == sequenceID }),
              sequence.isExportEligible,
              let destination = ExportDestinationPanel.chooseFolder() else { return }

        let sessionID = inspection.sessionID
        let attemptID = UUID()
        currentExportAttemptID = attemptID
        exportState = .exporting
        Task {
            do {
                let result = try await core.exportSequence(
                    sessionID: sessionID,
                    sequenceID: sequenceID,
                    destinationFolder: destination.path,
                    filenameStem: sequence.displayName
                )
                guard case .inspected(let current) = projectState,
                      current.sessionID == sessionID,
                      selectedSequenceID == sequenceID,
                      currentExportAttemptID == attemptID else { return }
                exportState = .succeeded(result)
            } catch {
                guard case .inspected(let current) = projectState,
                      current.sessionID == sessionID,
                      selectedSequenceID == sequenceID,
                      currentExportAttemptID == attemptID else { return }
                exportState = .failed(message: error.localizedDescription)
            }
        }
    }

    private func resetExportState() {
        currentExportAttemptID = nil
        exportState = .idle
    }
}
