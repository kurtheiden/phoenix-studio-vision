import Foundation

@MainActor private var checks = 0
@MainActor func check(_ value: Bool, _ message: String) {
    checks += 1
    guard value else { fatalError(message) }
}
@MainActor func until(_ condition: @MainActor () async -> Bool) async {
    for _ in 0..<30_000 {
        if await condition() { return }
        try? await Task.sleep(nanoseconds: 1_000_000)
    }
    fatalError("Timed out waiting for recovery")
}
func project(_ session: String) throws -> ProjectInspection {
    let rows: [[String: Any]] = (0..<15).map { index in
        ["sequence_id": "\(session)-\(index)", "display_name": index == 1 ? "" : "Duplicate",
         "readiness": index < 8 || index == 14 ? "ready" : "partially_supported",
         "readiness_reason": ["severity": "informational", "display_detail": "Bounded fixture"],
         "musical_track_count": 2, "warning_count": 0, "export_capability": NSNull(),
         "bounded_export_capability": index < 8
            ? ["contract_id": "source_layout120_bounded_sequence", "contract_revision": 1, "display_label": "Bounded"] : NSNull(),
         "diagnostics_available": true]
    }
    let object: [String: Any] = ["session_id": session,
        "project": ["display_name": "Project", "byte_size": 42, "recognized_studio_vision": true,
                    "sequence_count": 15, "overall_readiness": "partially_supported", "warning_count": 0],
        "sequences": rows, "warnings": [], "diagnostics_available": true]
    return try JSONDecoder().decode(ProjectInspection.self, from: JSONSerialization.data(withJSONObject: object))
}
struct Call: Equatable {
    let session: String
    let sequence: String
    let destination: String
    let stem: String
}
func receipt(_ call: Call, wrongIdentity: Bool = false) throws -> ExportSequenceResult {
    let counts = ["notes":1,"generated_note_offs":1,"controllers":0,"bank_select_msb":0,"bank_select_lsb":0,
                  "programs":0,"pressure":0,"pitch_bend":0,"tempo":1,"meter":1]
    let object: [String: Any] = ["session_id":call.session,"sequence_id":wrongIdentity ? "wrong" : call.sequence,
        "sequence_display_name":"Duplicate","output_path":"\(call.destination)/\(call.stem).mid",
        "compatibility_profile":NSNull(),"bounded_export_capability":["contract_id":"source_layout120_bounded_sequence","contract_revision":1,"display_label":"Bounded"],
        "musical_track_count":2,"total_smf_track_count":3,"counts":counts,"warnings":[],
        "untranslated_metadata_count":0,"validation_status":"validated"]
    return try JSONDecoder().decode(ExportSequenceResult.self, from: JSONSerialization.data(withJSONObject: object))
}
struct UnknownFailure: LocalizedError { var errorDescription: String? { "Unclassified error" } }
actor ProbeCore: PhoenixCoreServing {
    var opens = 0
    var calls: [Call] = []
    var pending: CheckedContinuation<ExportSequenceResult, Error>?
    var active = 0
    var maximumActive = 0
    func handshake() -> UInt32 { 1 }
    func inspectProject(path: String) throws -> ProjectInspection { opens += 1; return try project("session-\(opens)") }
    func getDiagnostics(sessionID: String) throws -> DiagnosticsSummary {
        let object: [String: Any] = ["core_version":"test","recognized_profile":"Bounded120","structural_status":"bounded","unsupported_families":[]]
        return try JSONDecoder().decode(DiagnosticsSummary.self, from: JSONSerialization.data(withJSONObject: object))
    }
    func exportSequence(sessionID: String, sequenceID: String, destinationFolder: String, filenameStem: String) async throws -> ExportSequenceResult {
        active += 1; maximumActive = max(maximumActive, active)
        defer { active -= 1 }
        calls.append(Call(session:sessionID,sequence:sequenceID,destination:destinationFolder,stem:filenameStem))
        return try await withCheckedThrowingContinuation { pending = $0 }
    }
    func pendingCount() -> Int { pending == nil ? 0 : 1 }
    func callCount() -> Int { calls.count }
    func allCalls() -> [Call] { calls }
    func maxActive() -> Int { maximumActive }
    func openCount() -> Int { opens }
    func finish(error: (any Error)? = nil, wrongIdentity: Bool = false) throws {
        guard let continuation = pending, let call = calls.last else { fatalError("No pending call") }
        pending = nil
        if let error { continuation.resume(throwing:error) }
        else { continuation.resume(returning:try receipt(call,wrongIdentity:wrongIdentity)) }
    }
}
@MainActor final class Pickers {
    var destination: URL? = URL(fileURLWithPath:"/recovered")
    var projectPrompts = 0
    var destinationPrompts = 0
    func source() -> URL? { projectPrompts += 1; return URL(fileURLWithPath:"/source/Project") }
    func folder() -> URL? { destinationPrompts += 1; return destination }
}
@MainActor func setup() async -> (AppModel, ProbeCore, Pickers) {
    let core = ProbeCore(); let pickers = Pickers()
    let model = AppModel(core:core,chooseProject:{pickers.source()},chooseDestination:{pickers.folder()})
    await until { model.state == .ready(contractVersion:1) }
    model.openProject()
    await until { if case .inspected = model.projectState { return true }; return false }
    return (model,core,pickers)
}
@main struct BatchSmoke {
    @MainActor static func main() async throws {
        let (model,core,pickers) = await setup()
        check(model.recoverySelection.isEmpty && !model.canRecoverSelection,"Initial selection empty")
        model.selectedSequenceID = "session-1-8"
        model.setRecoverySelected("session-1-8",selected:true)
        model.setRecoverySelected("unknown",selected:true)
        model.setRecoverySelected("session-1-14",selected:true)
        model.recoverSelectedSequences()
        check(model.recoverySelection.isEmpty && !model.canRecoverSelection && pickers.destinationPrompts == 0,"Blocked/focus cannot recover")
        model.setRecoverySelected("session-1-0",selected:true)
        check(model.canRecoverSelection && model.recoveryActionTitle == "Recover 1 Sequence…","One selected action")
        model.selectedSequenceID = "session-1-9"
        check(model.recoverySelection == ["session-1-0"],"Focus independent of recovery")
        model.setRecoverySelected("session-1-0",selected:false)
        check(model.recoverySelection.isEmpty,"Ready deselect")
        model.selectAllRecoverable()
        check(model.recoverySelection.count == 8 && model.selectedRecoveryCount == 8,"Select exactly eight")
        check(model.recoveryActionTitle == "Recover 8 Sequences…","Multi action count")
        if case .inspected(let inspection) = model.projectState {
            check(inspection.sequences.count == 15 && inspection.recoverableSequenceCount == 8 && inspection.blockedSequenceCount == 7,"15/8/7 retained")
            check(inspection.sequences[0].displayName == inspection.sequences[2].displayName && model.recoverySelection.contains("session-1-0") && model.recoverySelection.contains("session-1-2"),"Duplicate identity independent")
            check(inspection.sequences[1].presentationName(ordinal:2) == "Unnamed sequence 2" && inspection.sequences[1].displayName.isEmpty,"Unnamed fallback source unchanged")
            check(inspection.sequences[8].recoveryStatus == "Not yet recoverable" && inspection.diagnosticsAvailable,"UI2A status/Details preserved")
        }
        model.clearRecoverySelection(); check(model.recoverySelection.isEmpty,"Clear Selection")
        model.setRecoverySelected("session-1-2",selected:true)
        model.setRecoverySelected("session-1-0",selected:true)
        model.setRecoverySelected("session-1-1",selected:true)
        pickers.destination = nil; model.recoverSelectedSequences()
        check(model.batchRecovery == nil && !model.isExporting,"Canceled destination dispatches nothing")
        check(await core.callCount() == 0,"No Core calls on panel cancellation")
        pickers.destination = URL(fileURLWithPath:"/recovered")
        model.recoverSelectedSequences()
        await until { await core.pendingCount() == 1 }
        let snapshot = model.batchRecovery?.snapshot
        check(snapshot?.items.map(\.sequence.sequenceID) == ["session-1-0","session-1-1","session-1-2"],"Frozen source-order selection")
        check(snapshot?.items[0].sequence.boundedExportCapability?.isRecognized == true,"Capability retained in snapshot")
        model.clearRecoverySelection(); model.selectAllRecoverable(); model.setRecoverySelected("session-1-0",selected:false)
        model.selectedSequenceID = "session-1-14"
        check(model.recoverySelection.count == 3 && model.batchRecovery?.snapshot == snapshot,"Active selection/snapshot immutable")
        let prompts = pickers.projectPrompts
        model.openProject()
        check(!model.canOpenProject && pickers.projectPrompts == prompts,"Project switching refused during batch")
        check(await core.openCount() == 1,"No new inspection while running")
        model.recoverSelectedSequences()
        check(await core.callCount() == 1,"No overlapping batch dispatch")
        try await core.finish()
        await until { let count = await core.callCount(); let pending = await core.pendingCount(); return count == 2 && pending == 1 }
        check(model.batchRecovery?.successCount == 1 && model.batchRecovery?.currentIndex == 1,"Individual success/progress")
        let conflict = RecoveryOperationError(category:"destination_exists",diagnosticCode:"destination_exists",displayMessage:"File exists",technicalMessage:"No overwrite")
        try await core.finish(error:conflict)
        await until { let count = await core.callCount(); let pending = await core.pendingCount(); return count == 3 && pending == 1 }
        check(model.batchRecovery?.failureCount == 1,"No-overwrite conflict stays failure")
        try await core.finish()
        await until { model.batchRecovery?.isRunning == false }
        check(model.batchRecovery?.successCount == 2 && model.batchRecovery?.failureCount == 1 && model.batchRecovery?.notAttemptedCount == 0,"Accurate partial result")
        check(model.batchRecovery?.summary == "2 sequences recovered.","No false three-success summary")
        check(model.batchRecovery?.isAllSuccessful == false,"Partial recovery not all-success")
        check(model.batchRecovery?.results.map(\.outcome.status) == ["Recovered","Could not recover","Recovered"],"Concise partial row statuses")
        check(model.batchRecovery?.failureContext?.message == "A MIDI file with this name already exists in the destination folder.","Musician-facing conflict context")
        check(model.batchRecovery?.failureContext?.technicalDetail.contains("destination_exists") == true,"Conflict diagnostics retained secondarily")
        check(await core.maxActive() == 1,"Strictly sequential Core calls")
        let calls = await core.allCalls()
        check(calls.map(\.destination) == ["/recovered","/recovered","/recovered"] && pickers.destinationPrompts == 2,"One chosen destination per started batch")
        check(calls[1].stem == "Sequence 2" && calls[0].sequence != calls[2].sequence,"Unnamed filename and duplicate IDs")
        check(model.canOpenProject,"Project can change after batch")
        model.loadDiagnosticsIfNeeded()
        await until { if case .loaded = model.diagnosticsState { return true }; return false }
        check(model.retainedExportDestination == pickers.destination,"Successful destination retained")

        // Single selected uses the same API, without a folder prompt when retained.
        model.clearRecoverySelection(); model.setRecoverySelected("session-1-1",selected:true)
        let previousPrompts = pickers.destinationPrompts
        model.recoverSelectedSequences(toRetainedDestination:true)
        await until { let count = await core.callCount(); let pending = await core.pendingCount(); return count == 4 && pending == 1 }
        try await core.finish()
        await until { model.batchRecovery?.isRunning == false }
        check(model.batchRecovery?.summary == "1 sequence recovered." && pickers.destinationPrompts == previousPrompts,"Single success existing API/destination")
        check(model.batchRecovery?.isAllSuccessful == true && model.batchRecovery?.results.map(\.outcome.status) == ["Recovered"],"One success presentation")
        check(model.batchRecovery?.snapshot.destination == pickers.destination && model.batchRecovery?.failureContext == nil,"Success destination/no failure context")

        // Fatal, unknown, and receipt mismatch stop all remaining requests.
        for kind in 0..<4 {
            let (fatalModel,fatalCore,_) = await setup()
            fatalModel.selectAllRecoverable(); fatalModel.recoverSelectedSequences()
            await until { await fatalCore.pendingCount() == 1 }
            if kind == 3 {
                try await fatalCore.finish()
                await until { let count = await fatalCore.callCount(); let pending = await fatalCore.pendingCount(); return count == 2 && pending == 1 }
            }
            if kind == 0 || kind == 3 {
                try await fatalCore.finish(error:RecoveryOperationError(category:"export_validation_failed",diagnosticCode:"source_identity_changed",displayMessage:"Source changed",technicalMessage:"Revalidation refused"))
            } else if kind == 1 { try await fatalCore.finish(error:UnknownFailure()) }
            else { try await fatalCore.finish(wrongIdentity:true) }
            await until { fatalModel.batchRecovery?.isRunning == false }
            check(await fatalCore.callCount() == (kind == 3 ? 2 : 1),"Unsafe failure stops subsequent requests")
            check(fatalModel.batchRecovery?.successCount == (kind == 3 ? 1 : 0) && fatalModel.batchRecovery?.failureCount == 1 && fatalModel.batchRecovery?.notAttemptedCount == (kind == 3 ? 6 : 7),"Stopped outcomes complete")
            check(fatalModel.batchRecovery?.summary == (kind == 3 ? "1 sequence recovered." : "No sequences were recovered."),"Honest stopped/partial failure")
            check(fatalModel.batchRecovery?.isAllSuccessful == false,"Stopped recovery never all-success")
            if kind == 0 || kind == 3 {
                check(fatalModel.batchRecovery?.failureContext?.message == "The Studio Vision project changed during recovery, so Phoenix stopped.","Source invalidation copy")
                check(fatalModel.batchRecovery?.failureContext?.nextStep == "Open the project again before recovering more sequences.","Safe source next step")
            }
            let statuses = fatalModel.batchRecovery?.results.map(\.outcome.status) ?? []
            check(statuses.filter { $0 == "Could not recover" }.count == 1 && statuses.filter { $0 == "Not attempted" }.count == (kind == 3 ? 6 : 7),"Failed/not attempted distinct")
            if kind == 3 { check(statuses.first == "Recovered","Earlier success stays recovered") }
            if kind == 1 {
                check(fatalModel.batchRecovery?.failureContext?.message == "Phoenix could not safely complete recovery.","Unknown error not invented")
                check(fatalModel.batchRecovery?.failureContext?.technicalDetail == "Unclassified error","Unknown diagnostics retained")
            }

        }
        check(!RecoveryOperationError(category:"output_io_failed",diagnosticCode:"destination_exists",displayMessage:"",technicalMessage:"").permitsNextSequence,"Category/code pair required for continuation")
        let lookalike = RecoveryFailurePresentation(error:RecoveryOperationError(category:"output_io_failed",diagnosticCode:"source_identity_changed",displayMessage:"opaque",technicalMessage:"detail"))
        check(lookalike.kind == .other,"Do not claim source change from wrong category")
        let exhausted = RecoveryFailurePresentation(error:RecoveryOperationError(category:"destination_exists",diagnosticCode:"destination_name_exhausted",displayMessage:"occupied",technicalMessage:"detail"))
        check(exhausted.message == "Phoenix could not find an unused MIDI filename in the destination folder.","Unique-name exhaustion accurate copy")
        check(RecoveryOperationError(category:"destination_exists",diagnosticCode:"destination_name_exhausted",displayMessage:"",technicalMessage:"").permitsNextSequence,"Existing unique-name exhaustion local")
        check(RecoveryWorkspaceCopy.emptyTitle == "Recover MIDI from an old Studio Vision project","UI2A start preserved")
        model.openProject()
        await until { await core.openCount() == 2 }
        await until { if case .inspected(let p) = model.projectState { return p.sessionID == "session-2" }; return false }
        check(model.recoverySelection.isEmpty,"New project initial selection empty")
        // Actual Swift orchestration uses the established external specimen only.
        let directory = FileManager.default.temporaryDirectory.appendingPathComponent("phoenix-ui2b-\(UUID().uuidString)")
        try FileManager.default.createDirectory(at:directory,withIntermediateDirectories:false)
        defer { try? FileManager.default.removeItem(at:directory) }
        let occupied = directory.appendingPathComponent("DANCING.mid")
        let sentinel = Data("existing MIDI destination must remain unchanged".utf8)
        try sentinel.write(to:occupied)
        let source = URL(fileURLWithPath:"/Users/kurtheiden/Documents/Phoenix Research/Authentic Studio Vision Projects/SCHOOL PROJECTS")
        let sourceBefore = try Data(contentsOf:source)
        let real = AppModel(core:PhoenixCore(),chooseProject:{source},chooseDestination:{directory})
        await until { real.state == .ready(contractVersion:1) }
        real.openProject()
        await until { if case .inspected = real.projectState { return true }; return false }
        guard case .inspected(let actual) = real.projectState else { fatalError("No actual project") }
        check(actual.sequences.count == 15 && actual.recoverableSequenceCount == 8 && actual.blockedSequenceCount == 7,"Actual SCHOOL 15/8/7")
        real.selectAllRecoverable(); check(real.selectedRecoveryCount == 8,"Actual SCHOOL Select All exactly eight")
        real.recoverSelectedSequences()
        await until { real.batchRecovery?.isRunning == false }
        guard let batch = real.batchRecovery else { fatalError("No actual batch receipt") }
        check(batch.successCount == 8 && batch.failureCount == 0 && batch.notAttemptedCount == 0,"Actual eight-Sequence batch success")
        check(batch.isAllSuccessful && batch.summary == "8 sequences recovered." && batch.results.allSatisfy { $0.outcome.status == "Recovered" },"All-success presentation actual eight")
        check(batch.snapshot.destination == directory,"Actual completion destination")
        let names = [1:"DANCING",2:"NOTHING",5:"Nothing FINAL",8:"Sequence H",9:"Unnamed Sequence 09",10:"Wind ensemble",12:"Tuba thing",13:"Sequence M"]
        let artifacts = URL(fileURLWithPath:"/Users/kurtheiden/Documents/Phoenix Research/Experimental MIDI Recovery/SCHOOL PROJECTS")
        for entry in batch.results {
            guard case .succeeded(let result) = entry.outcome, let name = names[entry.item.ordinal] else { fatalError("Unexpected result") }
            let expected = artifacts.appendingPathComponent("SCHOOL - \(name).mid")
            check(try Data(contentsOf:URL(fileURLWithPath:result.outputPath)) == Data(contentsOf:expected),"Actual output parity \(name)")
        }
        check(try Data(contentsOf:occupied) == sentinel,"Actual no-overwrite preserved")
        check(try Data(contentsOf:source) == sourceBefore,"Authentic source unchanged")
        print("UI2B workflow smoke: \(checks) checks passed")
    }
}
