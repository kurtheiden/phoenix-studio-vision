import Foundation
import Combine

protocol PhoenixCoreServing: Sendable {
    func handshake() async throws -> UInt32
    func inspectProject(path: String) async throws -> ProjectInspection
    func getDiagnostics(sessionID: String) async throws -> DiagnosticsSummary
    func exportSequence(sessionID: String, sequenceID: String,
                        destinationFolder: String, filenameStem: String) async throws -> ExportSequenceResult
}

/// Destination naming only. Core still validates and publishes without overwrite.
enum ExportFilename {
    static func stem(for name: String, ordinal: Int) -> String {
        var candidate = ""
        for scalar in name.unicodeScalars {
            let replacement = CharacterSet.controlCharacters.contains(scalar)
                || "/\\:".unicodeScalars.contains(scalar) ? "-" : String(scalar)
            // Leave room for .mid and Core's unique-name suffix on macOS.
            if candidate.utf8.count + replacement.utf8.count > 180 { break }
            candidate += replacement
        }
        candidate = candidate.trimmingCharacters(in: .whitespacesAndNewlines)
        let validatedPart = candidate.lowercased().hasSuffix(".mid")
            ? String(candidate.dropLast(4)) : candidate
        if validatedPart.isEmpty || validatedPart.unicodeScalars.allSatisfy({
            CharacterSet.whitespacesAndNewlines.contains($0) || ".-".unicodeScalars.contains($0)
        }) {
            return "Sequence \(max(1, ordinal))"
        }
        return candidate
    }
}

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

    struct ExportContext: Equatable {
        let attemptID: UUID
        let sessionID: String
        let sequenceID: String
        let sequenceDisplayName: String
        let sourceURL: URL
        let destination: URL
        let filenameStem: String
    }

    enum ExportState: Equatable {
        case idle
        case exporting(ExportContext)
        case succeeded(ExportContext, ExportSequenceResult)
        case failed(ExportContext, message: String)
    }

    @Published private(set) var state: State = .starting
    @Published private(set) var projectState: ProjectState = .idle
    @Published var selectedSequenceID: String?
    @Published private(set) var inspectedSourceURL: URL?
    @Published private(set) var diagnosticsState: DiagnosticsState = .notLoaded
    @Published private(set) var exportState: ExportState = .idle
    @Published private(set) var retainedExportDestination: URL?
    private let core: any PhoenixCoreServing
    private let chooseProject: @MainActor () -> URL?
    private let chooseDestination: @MainActor () -> URL?
    private var started = false
    private var currentExportAttemptID: UUID?

    init(core: any PhoenixCoreServing = PhoenixCore(),
         chooseProject: @escaping @MainActor () -> URL? = ProjectOpenPanel.chooseProject,
         chooseDestination: @escaping @MainActor () -> URL? = ExportDestinationPanel.chooseFolder) {
        self.core = core
        self.chooseProject = chooseProject
        self.chooseDestination = chooseDestination
        startHandshake()
    }

    var isExporting: Bool {
        if case .exporting = exportState { return true }
        return false
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
        guard canOpenProject, let url = chooseProject() else { return }
        selectedSequenceID = nil
        inspectedSourceURL = nil
        retainedExportDestination = nil
        diagnosticsState = .notLoaded
        projectState = .inspecting
        let path = url.path
        Task {
            do {
                let summary = try await core.inspectProject(path: path)
                selectedSequenceID = nil
                diagnosticsState = .notLoaded
                inspectedSourceURL = url
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
        guard !isExporting else { return }
        beginExportIfPossible(destination: nil)
    }

    func exportSelectedSequenceToRetainedDestination() {
        guard !isExporting,
              let destination = retainedExportDestination else { return }
        beginExportIfPossible(destination: destination)
    }

    private func beginExportIfPossible(destination retainedDestination: URL?) {
        guard case .inspected(let inspection) = projectState,
              let sourceURL = inspectedSourceURL,
              let sequenceID = selectedSequenceID,
              let index = inspection.sequences.firstIndex(where: { $0.sequenceID == sequenceID }),
              inspection.sequences[index].isExportEligible else { return }
        let sequence = inspection.sequences[index]

        let destination: URL
        if let retainedDestination {
            destination = retainedDestination
        } else if let chosenDestination = chooseDestination() {
            destination = chosenDestination
        } else {
            return
        }

        let context = ExportContext(
            attemptID: UUID(), sessionID: inspection.sessionID, sequenceID: sequenceID,
            sequenceDisplayName: sequence.displayName, sourceURL: sourceURL,
            destination: destination,
            filenameStem: ExportFilename.stem(for: sequence.displayName, ordinal: index + 1)
        )
        currentExportAttemptID = context.attemptID
        exportState = .exporting(context)
        Task {
            do {
                let result = try await core.exportSequence(
                    sessionID: context.sessionID,
                    sequenceID: context.sequenceID,
                    destinationFolder: context.destination.path,
                    filenameStem: context.filenameStem
                )
                guard currentExportAttemptID == context.attemptID else { return }
                if case .inspected(let current) = projectState,
                   current.sessionID == context.sessionID {
                    retainedExportDestination = context.destination
                }
                exportState = .succeeded(context, result)
            } catch {
                guard currentExportAttemptID == context.attemptID else { return }
                exportState = .failed(context, message: error.localizedDescription)
            }
            currentExportAttemptID = nil
        }
    }
}
