import Foundation

@MainActor
private var assertions = 0
@MainActor
func check(_ value: Bool, _ message: String) {
    assertions += 1
    guard value else { fatalError(message) }
}
@MainActor
func until(_ predicate: @MainActor () async -> Bool) async {
    for _ in 0..<100_000 {
        if await predicate() { return }
        await Task.yield()
    }
    fatalError("Timed out waiting for model transition")
}
func inspection(_ session: String) throws -> ProjectInspection {
    let rows: [[String: Any]] = [
        ["sequence_id": "\(session)-a", "display_name": "../Legacy", "readiness": "ready",
         "readiness_reason": ["severity": "informational", "display_detail": "Ready"],
         "warning_count": 0, "export_capability": NSNull(),
         "bounded_export_capability": ["contract_id": "descriptor166_bounded_sequence", "contract_revision": 1, "display_label": "Bounded"], "diagnostics_available": false],
        ["sequence_id": "\(session)-b", "display_name": "Other", "readiness": "partially_supported",
         "readiness_reason": ["severity": "caution", "display_detail": "Refused"],
         "warning_count": 1, "export_capability": NSNull(), "diagnostics_available": false]
    ]
    let payload: [String: Any] = ["session_id": session,
        "project": ["display_name": "archive", "byte_size": 123, "recognized_studio_vision": true,
                    "sequence_count": 2, "overall_readiness": "partially_supported", "warning_count": 1],
        "sequences": rows, "warnings": [], "diagnostics_available": false]
    return try JSONDecoder().decode(ProjectInspection.self, from: JSONSerialization.data(withJSONObject: payload))
}
struct Failure: LocalizedError {
    var errorDescription: String? { "Controlled export failure" }
}
struct ExportCall: Sendable {
    let session: String
    let sequence: String
    let destination: String
    let stem: String
}
func receipt(_ call: ExportCall) throws -> ExportSequenceResult {
    let counts = ["notes": 1, "generated_note_offs": 1, "controllers": 0, "bank_select_msb": 0,
                  "bank_select_lsb": 0, "programs": 0, "pressure": 0, "pitch_bend": 0, "tempo": 1, "meter": 1]
    let payload: [String: Any] = ["session_id": call.session, "sequence_id": call.sequence,
        "sequence_display_name": "../Legacy", "output_path": "\(call.destination)/\(call.stem).mid",
        "compatibility_profile": NSNull(), "musical_track_count": 1, "total_smf_track_count": 2,
        "counts": counts, "warnings": [["message": "Receipt warning", "scope": "sequence", "severity": "caution"]],
        "untranslated_metadata_count": 0, "validation_status": "validated"]
    return try JSONDecoder().decode(ExportSequenceResult.self, from: JSONSerialization.data(withJSONObject: payload))
}
actor ControlledCore: PhoenixCoreServing {
    var opens = 0
    var failInspection = false
    var calls: [ExportCall] = []
    var pending: CheckedContinuation<ExportSequenceResult, Error>?
    func handshake() -> UInt32 { 1 }
    func inspectProject(path: String) throws -> ProjectInspection {
        opens += 1
        if failInspection { throw Failure() }
        return try inspection("session-\(opens)")
    }
    func getDiagnostics(sessionID: String) throws -> DiagnosticsSummary { throw Failure() }
    func exportSequence(sessionID: String, sequenceID: String, destinationFolder: String,
                        filenameStem: String) async throws -> ExportSequenceResult {
        calls.append(.init(session: sessionID, sequence: sequenceID, destination: destinationFolder, stem: filenameStem))
        return try await withCheckedThrowingContinuation { pending = $0 }
    }
    func isPending() -> Bool { pending != nil }
    func callCount() -> Int { calls.count }
    func lastCall() -> ExportCall { calls.last! }
    func setInspectionFailure() { failInspection = true }
    func finish(success: Bool) throws {
        let continuation = pending!
        pending = nil
        if success { continuation.resume(returning: try receipt(calls.last!)) }
        else { continuation.resume(throwing: Failure()) }
    }
}
@MainActor
final class Pickers {
    var source: URL? = URL(fileURLWithPath: "/archive/original/Project")
    var destination: URL? = URL(fileURLWithPath: "/recovered")
}

@main
struct WorkflowSmoke {
    @MainActor
    static func main() async throws {
        for (name, expected) in [("Ordinary Name", "Ordinary Name"), ("Song.mid", "Song.mid"),
            ("Song.mid.mid", "Song.mid.mid"), ("../escape", "..-escape"), ("a/b\\c:d", "a-b-c-d"),
            ("a\u{0}b", "a-b"), ("", "Sequence 3"), (" \n\t ", "Sequence 3"), (".", "Sequence 3"),
            ("..", "Sequence 3"), (".mid", "Sequence 3"), ("/\\:", "Sequence 3")] {
            check(ExportFilename.stem(for: name, ordinal: 3) == expected, "Filename \(name.debugDescription)")
        }
        let long = ExportFilename.stem(for: String(repeating: "🎵", count: 200), ordinal: 1)
        check(long.utf8.count <= 180 && !long.isEmpty, "Bounded Unicode filename")
        check(ExportFilename.stem(for: "...", ordinal: 0) == "Sequence 1", "Fallback ordinal")

        // A/B: selection changes while the real model awaits a controllable service.
        for success in [true, false] {
            let core = ControlledCore(); let pickers = Pickers()
            let model = AppModel(core: core, chooseProject: { pickers.source }, chooseDestination: { pickers.destination })
            await until { model.state == .ready(contractVersion: 1) }
            model.openProject()
            await until { if case .inspected = model.projectState { return true }; return false }
            check(model.inspectedSourceURL == pickers.source, "Selected source path retained")
            check(model.inspectedSourceURL?.lastPathComponent == "Project", "Source filename")
            model.selectedSequenceID = "session-1-a"
            model.exportSelectedSequence()
            await until { await core.isPending() }
            let original = await core.lastCall()
            check(original.stem == "..-Legacy", "Legacy source name sanitized in request only")
            model.selectedSequenceID = "session-1-b"
            check(model.isExporting, "Selection keeps in-flight state")
            model.exportSelectedSequence()
            check(await core.callCount() == 1, "Single in-flight operation")
            try await core.finish(success: success)
            await until { !model.isExporting }
            switch model.exportState {
            case .succeeded(let context, let result):
                check(success && context.sequenceID == original.sequence, "A: original successful sequence")
                check(context.sequenceDisplayName == "../Legacy", "Original musical name unchanged")
                check(result.outputPath == "/recovered/..-Legacy.mid", "Actual output path")
                check(result.warnings.first?.message == "Receipt warning", "Warnings preserved")
                check(model.retainedExportDestination == pickers.destination, "Same project remembers destination")
            case .failed(let context, let message):
                check(!success && context.sequenceID == original.sequence, "B: original failed sequence")
                check(message == "Controlled export failure", "Original failure reason")
                check(context.destination == pickers.destination, "Failure destination")
            default: fatalError("Missing receipt")
            }
            check(model.selectedSequenceID == "session-1-b", "Completion does not change selection")
            model.selectedSequenceID = nil
            check(model.exportState != .idle, "Receipt visible with no selected row")
        }

        // C: a new same-named project cannot acquire the old receipt's folder/state.
        for success in [true, false] {
            let core = ControlledCore(); let pickers = Pickers()
            let model = AppModel(core: core, chooseProject: { pickers.source }, chooseDestination: { pickers.destination })
            await until { model.state == .ready(contractVersion: 1) }
            model.openProject()
            await until { model.inspectedSourceURL != nil }
            let oldSource = model.inspectedSourceURL
            model.selectedSequenceID = "session-1-a"; model.exportSelectedSequence()
            await until { await core.isPending() }
            pickers.source = URL(fileURLWithPath: "/archive/repaired/Project")
            model.openProject()
            check(model.inspectedSourceURL == nil, "Old source cleared during inspection")
            await until { model.inspectedSourceURL == pickers.source }
            model.selectedSequenceID = "session-2-b"
            try await core.finish(success: success)
            await until { !model.isExporting }
            let context: AppModel.ExportContext
            switch model.exportState {
            case .succeeded(let c, _): context = c
            case .failed(let c, _): context = c
            default: fatalError("Missing old completion")
            }
            check(context.sessionID == "session-1" && context.sourceURL == oldSource, "C: receipt labelled as old project")
            check(model.selectedSequenceID == "session-2-b", "New project selection isolated")
            check(model.inspectedSourceURL == pickers.source, "New project source isolated")
            check(model.retainedExportDestination == nil, "Old completion cannot set new project's folder")
            if case .inspected(let current) = model.projectState { check(current.sessionID == "session-2", "New session unchanged") }
            else { fatalError("New project state lost") }
        }

        // D: normal completion, subsequent navigation, picker cancellation and failure.
        let core = ControlledCore(); let pickers = Pickers()
        let model = AppModel(core: core, chooseProject: { pickers.source }, chooseDestination: { pickers.destination })
        await until { model.state == .ready(contractVersion: 1) }
        model.openProject(); await until { model.inspectedSourceURL != nil }
        model.selectedSequenceID = "session-1-b"; model.exportSelectedSequence()
        check(!model.isExporting, "Partial is not exportable")
        model.selectedSequenceID = "session-1-a"; model.exportSelectedSequence()
        await until { await core.isPending() }; try await core.finish(success: true)
        await until { !model.isExporting }
        if case .succeeded(let context, _) = model.exportState { check(context.sequenceID == model.selectedSequenceID, "D: normal result") }
        else { fatalError("Normal result missing") }
        let source = model.inspectedSourceURL; let previous = model.exportState
        pickers.source = nil; model.openProject()
        check(model.inspectedSourceURL == source && model.exportState == previous, "Canceled picker preserves state")
        await core.setInspectionFailure()
        pickers.source = URL(fileURLWithPath: "/archive/broken/Other")
        model.openProject(); check(model.inspectedSourceURL == nil, "No stale source while opening")
        await until { if case .failed = model.projectState { return true }; return false }
        check(model.inspectedSourceURL == nil, "Failed inspection has no stale source")
        check(model.exportState == previous, "Previous receipt remains explicitly source-labelled")

        // Real Core publication: policy feeds the unchanged validation/publisher.
        let directory = FileManager.default.temporaryDirectory.appendingPathComponent("phoenix-v1a-\(UUID().uuidString)")
        try FileManager.default.createDirectory(at: directory, withIntermediateDirectories: false)
        defer { try? FileManager.default.removeItem(at: directory) }
        let real = PhoenixCore(); _ = try await real.handshake()
        let project = try await real.inspectProject(path: "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline")
        let sequence = project.sequences.first { $0.displayName == "Sequence R" }!
        check(sequence.isExportEligible && sequence.exportCapability == nil && sequence.boundedExportCapability != nil, "Real bounded capability")
        let occupied = directory.appendingPathComponent("Ordinary Name.mid")
        try Data("preserve".utf8).write(to: occupied)
        for (index, name) in ["Ordinary Name", "Ordinary Name", "../escape", "", "a/b\\c"].enumerated() {
            let result = try await real.exportSequence(sessionID: project.sessionID, sequenceID: sequence.sequenceID,
                destinationFolder: directory.path, filenameStem: ExportFilename.stem(for: name, ordinal: 3))
            check(URL(fileURLWithPath: result.outputPath).deletingLastPathComponent().path == directory.path, "Safe destination containment")
            check(try Data(contentsOf: URL(fileURLWithPath: result.outputPath)).prefix(4) == Data("MThd".utf8), "Usable SMF")
            if index < 2 { check(URL(fileURLWithPath: result.outputPath).lastPathComponent == "Ordinary Name \(index + 2).mid", "Collision retry") }
        }
        check(try Data(contentsOf: occupied) == Data("preserve".utf8), "No overwrite")
        let entries = try FileManager.default.contentsOfDirectory(atPath: directory.path)
        do {
            _ = try await real.exportSequence(sessionID: project.sessionID, sequenceID: sequence.sequenceID,
                destinationFolder: directory.path, filenameStem: "../escape")
            fatalError("Core accepted traversal")
        } catch { check(try FileManager.default.contentsOfDirectory(atPath: directory.path) == entries, "Traversal refused without output") }
        print("V1A workflow smoke: \(assertions) checks passed; 0 failed; 0 ignored")
    }
}
