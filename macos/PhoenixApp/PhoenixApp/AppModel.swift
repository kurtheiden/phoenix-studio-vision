import Foundation
import Combine
import AppKit

protocol PhoenixCoreServing: Sendable {
    func partialRecovery(sessionID: String, sequenceID: String, inclusion: RecoveryInclusionPreference, destination: String, consent: String?) async throws -> String
    func handshake() async throws -> UInt32
    func inspectProject(path: String) async throws -> ProjectInspection
    func getDiagnostics(sessionID: String) async throws -> DiagnosticsSummary
    func exportSequence(sessionID: String, sequenceID: String,
                        destinationFolder: String, filenameStem: String,
                        recoveryInclusion: RecoveryInclusionPreference?) async throws -> ExportSequenceResult
}

// Existing callers omit preferences; only the scoped recovery route accepts them.
extension PhoenixCoreServing {
    func partialRecovery(sessionID: String, sequenceID: String, inclusion: RecoveryInclusionPreference, destination: String, consent: String?) async throws -> String { throw NSError(domain: "PhoenixPartialRecovery", code: 1) }
    func exportSequence(sessionID: String, sequenceID: String, destinationFolder: String, filenameStem: String) async throws -> ExportSequenceResult {
        try await exportSequence(sessionID: sessionID, sequenceID: sequenceID, destinationFolder: destinationFolder,
                                 filenameStem: filenameStem, recoveryInclusion: nil)
    }
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
        let recoveryInclusion: RecoveryInclusionPreference?
    }

    enum ExportState: Equatable {
        case idle
        case exporting(ExportContext)
        case succeeded(ExportContext, ExportSequenceResult)
        case failed(ExportContext, message: String)
    }

    struct RecoveryItem: Equatable {
        let sequence: SequenceViewData
        let ordinal: Int
        var displayName: String { sequence.presentationName(ordinal: ordinal) }
    }
    struct RecoverySnapshot: Equatable {
        let id: UUID
        let sessionID: String
        let sourceURL: URL
        let destination: URL
        let items: [RecoveryItem]
        let recoveryInclusion: RecoveryInclusionPreference
    }
    enum RecoveryOutcome: Equatable {
        case succeeded(ExportSequenceResult)
        case failed(RecoveryFailurePresentation)
        case notAttempted
        var status: String {
            switch self {
            case .succeeded: return "Recovered"
            case .failed: return "Could not recover"
            case .notAttempted: return "Not attempted"
            }
        }
        var statusSymbol: String {
            switch self {
            case .succeeded: return "checkmark.circle"
            case .failed: return "exclamationmark.circle"
            case .notAttempted: return "minus.circle"
            }
        }
    }
    struct RecoveryResult: Equatable {
        let item: RecoveryItem
        let outcome: RecoveryOutcome
    }
    struct BatchRecovery: Equatable {
        let snapshot: RecoverySnapshot
        var currentIndex: Int?
        var results: [RecoveryResult]
        var isRunning: Bool
        var successCount: Int { results.filter { if case .succeeded = $0.outcome { return true }; return false }.count }
        var failureCount: Int { results.filter { if case .failed = $0.outcome { return true }; return false }.count }
        var notAttemptedCount: Int { results.filter { if case .notAttempted = $0.outcome { return true }; return false }.count }
        var isAllSuccessful: Bool {
            !isRunning && results.count == snapshot.items.count && !results.isEmpty
                && failureCount == 0 && notAttemptedCount == 0
        }
        var failureContext: RecoveryFailurePresentation? {
            let failures = results.compactMap { result -> RecoveryFailurePresentation? in
                if case .failed(let failure) = result.outcome { return failure }
                return nil
            }
            // The last failure is the stopping request when later items were not attempted.
            return notAttemptedCount > 0 ? failures.last : failures.first
        }
        var summary: String {
            if successCount == 0 { return "No sequences were recovered." }
            return "\(successCount) \(successCount == 1 ? "sequence" : "sequences") recovered."
        }
    }
    @Published private(set) var excludeMutedTracks = false
    var canChangeRecoveryInclusion: Bool {
        guard !isExporting, case .inspected = projectState else { return false }
        return true
    }
    func setExcludeMutedTracks(_ excluded: Bool) {
        guard canChangeRecoveryInclusion else { return }
        excludeMutedTracks = excluded
    }
    private var recoveryInclusion: RecoveryInclusionPreference { excludeMutedTracks ? .excludeMuted : .includeMuted }

    @Published private(set) var recoverySelection: Set<String> = []
    @Published var workspaceTab: RecoveryWorkspaceTab = .sequences
    @Published private(set) var batchRecovery: BatchRecovery?

    @Published private(set) var state: State = .starting
    @Published private(set) var projectState: ProjectState = .idle
    /// Row focus for inspection only; it never changes recoverySelection.
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
        if batchRecovery?.isRunning == true { return true }
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
        if case .ready = state { return projectState != .inspecting && batchRecovery?.isRunning != true }
        return false
    }

    var currentProjectRecovery: BatchRecovery? {
        guard case .inspected(let inspection) = projectState,
              let batch = batchRecovery, batch.snapshot.sessionID == inspection.sessionID else { return nil }
        return batch
    }

    func openProject() {
        guard canOpenProject, let url = chooseProject() else { return }
        selectedSequenceID = nil
        recoverySelection = []
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
                workspaceTab = .sequences
                excludeMutedTracks = false
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

    var selectedRecoveryCount: Int {
        guard case .inspected(let inspection) = projectState else { return 0 }
        return inspection.sequences.filter { $0.isExportEligible && recoverySelection.contains($0.sequenceID) }.count
    }
    var canRecoverSelection: Bool { selectedRecoveryCount > 0 && !isExporting }
    var recoveryActionTitle: String {
        let count = selectedRecoveryCount
        return count == 0 ? "Recover Sequences…" : "Recover \(count) \(count == 1 ? "Sequence" : "Sequences")…"
    }
    func setRecoverySelected(_ id: String, selected: Bool) {
        guard !isExporting, case .inspected(let inspection) = projectState,
              inspection.sequences.contains(where: { $0.sequenceID == id && $0.isExportEligible }) else { return }
        if selected { recoverySelection.insert(id) } else { recoverySelection.remove(id) }
    }
    func selectAllRecoverable() {
        guard !isExporting, case .inspected(let inspection) = projectState else { return }
        recoverySelection = Set(inspection.sequences.filter(\.isExportEligible).map(\.sequenceID))
    }
    func clearRecoverySelection() {
        guard !isExporting else { return }
        recoverySelection = []
    }
    func recoverSelectedSequences(toRetainedDestination: Bool = false) {
        guard canRecoverSelection, case .inspected(let inspection) = projectState,
              let sourceURL = inspectedSourceURL else { return }
        let items = inspection.sequences.enumerated().compactMap { index, sequence -> RecoveryItem? in
            guard sequence.isExportEligible && recoverySelection.contains(sequence.sequenceID) else { return nil }
            return RecoveryItem(sequence: sequence, ordinal: index + 1)
        }
        guard !items.isEmpty else { return }
        let inclusion = recoveryInclusion
        let destination: URL
        if toRetainedDestination {
            guard let retained = retainedExportDestination else { return }
            destination = retained
        } else {
            guard let chosen = chooseDestination() else { return }
            destination = chosen
        }
        guard !isExporting, case .inspected(let current) = projectState,
              current.sessionID == inspection.sessionID else { return }
        let snapshot = RecoverySnapshot(id: UUID(), sessionID: inspection.sessionID,
                                        sourceURL: sourceURL, destination: destination, items: items, recoveryInclusion: inclusion)
        batchRecovery = BatchRecovery(snapshot: snapshot, currentIndex: 0, results: [], isRunning: true)
        // The actor-backed one-Sequence API retains all Core authorization and
        // publication gates. No Swift file writer or cached MIDI is involved.
        Task {
            var progress = BatchRecovery(snapshot: snapshot, currentIndex: 0, results: [], isRunning: true)
            for (index, item) in snapshot.items.enumerated() {
                progress.currentIndex = index
                batchRecovery = progress
                do {
                    let result = try await core.exportSequence(sessionID: snapshot.sessionID,
                        sequenceID: item.sequence.sequenceID, destinationFolder: snapshot.destination.path,
                        filenameStem: ExportFilename.stem(for: item.sequence.displayName, ordinal: item.ordinal),
                        recoveryInclusion: item.sequence.supportsRecoveryInclusion ? snapshot.recoveryInclusion : nil)
                    guard result.sessionID == snapshot.sessionID && result.sequenceID == item.sequence.sequenceID else {
                        throw RecoveryOperationError(category: "invalid_response", diagnosticCode: "receipt_identity_mismatch",
                            displayMessage: "Phoenix could not confirm this recovery result.", technicalMessage: "Receipt identity differs from the requested Sequence.")
                    }
                    progress.results.append(RecoveryResult(item: item, outcome: .succeeded(result)))
                    retainedExportDestination = snapshot.destination
                } catch {
                    let typed = error as? RecoveryOperationError
                    progress.results.append(RecoveryResult(item: item, outcome: .failed(
                        RecoveryFailurePresentation(error: error))))
                    if typed?.permitsNextSequence != true {
                        progress.results += snapshot.items.dropFirst(index + 1).map { RecoveryResult(item: $0, outcome: .notAttempted) }
                        break
                    }
                }
            }
            progress.currentIndex = nil
            progress.isRunning = false
            batchRecovery = progress
            workspaceTab = .results
        }
    }

    func recoverPartialSequence() {
        guard !isExporting, case .inspected(let inspection) = projectState,
              let sequence = selectedSequenceID else { return }
        let inclusion = recoveryInclusion
        Task {
            do {
                let report = try await core.partialRecovery(sessionID: inspection.sessionID,
                    sequenceID: sequence, inclusion: inclusion, destination: "", consent: nil)
                let alert = NSAlert()
                alert.messageText = "Export incomplete MIDI recovery?"
                alert.informativeText = "Only independently qualified tracks will be exported. Original mute inclusion, full conductor map and completeness are not guaranteed. Review the recovery report below."
                let scroll = NSScrollView(frame: NSRect(x: 0, y: 0, width: 620, height: 360))
                let text = NSTextView(frame: scroll.bounds)
                text.string = report
                text.isEditable = false
                scroll.documentView = text
                scroll.hasVerticalScroller = true
                alert.accessoryView = scroll
                alert.addButton(withTitle: "Export Incomplete MIDI…")
                alert.addButton(withTitle: "Cancel")
                guard alert.runModal() == .alertFirstButtonReturn,
                      let destination = chooseDestination() else { return }
                let receipt = try await core.partialRecovery(sessionID: inspection.sessionID,
                    sequenceID: sequence, inclusion: inclusion, destination: destination.path, consent: report)
                let done = NSAlert()
                done.messageText = "Incomplete recovery exported"
                done.informativeText = receipt
                done.runModal()
            } catch {
                let alert = NSAlert()
                alert.messageText = "Partial recovery unavailable"
                alert.informativeText = String(describing: error)
                alert.runModal()
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
        let inclusion = sequence.supportsRecoveryInclusion ? recoveryInclusion : nil

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
            filenameStem: ExportFilename.stem(for: sequence.displayName, ordinal: index + 1),
            recoveryInclusion: inclusion
        )
        currentExportAttemptID = context.attemptID
        exportState = .exporting(context)
        Task {
            do {
                let result = try await core.exportSequence(
                    sessionID: context.sessionID,
                    sequenceID: context.sequenceID,
                    destinationFolder: context.destination.path,
                    filenameStem: context.filenameStem,
                    recoveryInclusion: context.recoveryInclusion
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
