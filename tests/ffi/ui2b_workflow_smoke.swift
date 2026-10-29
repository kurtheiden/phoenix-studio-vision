import Foundation
import CryptoKit

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
func project(_ session: String, recovery: Bool = false) throws -> ProjectInspection {
    let rows: [[String: Any]] = (0..<15).map { index in
        ["sequence_id": "\(session)-\(index)", "display_name": index == 1 ? "" : "Duplicate",
         "readiness": index < 8 || index == 14 ? "ready" : "partially_supported",
         "readiness_reason": ["severity": "informational", "display_detail": "Bounded fixture"],
         "musical_track_count": 2, "warning_count": 0, "export_capability": NSNull(),
         "bounded_export_capability": index < 8
            ? ["contract_id": recovery && index < 2 ? (index == 0 ? "authenticated_xform_recovery" : "authenticated_jurrasic_recovery") : "source_layout120_bounded_sequence", "contract_revision": 1, "display_label": "Bounded"] : NSNull(),
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
    var inclusion: RecoveryInclusionPreference? = nil
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
    var recovery = false
    func enableRecovery() { recovery = true }
    var calls: [Call] = []
    var pending: CheckedContinuation<ExportSequenceResult, Error>?
    var active = 0
    var maximumActive = 0
    func handshake() -> UInt32 { 1 }
    func inspectProject(path: String) throws -> ProjectInspection { opens += 1; return try project("session-\(opens)", recovery: recovery) }
    func getDiagnostics(sessionID: String) throws -> DiagnosticsSummary {
        let object: [String: Any] = ["core_version":"test","recognized_profile":"Bounded120","structural_status":"bounded","unsupported_families":[]]
        return try JSONDecoder().decode(DiagnosticsSummary.self, from: JSONSerialization.data(withJSONObject: object))
    }
    func exportSequence(sessionID: String, sequenceID: String, destinationFolder: String, filenameStem: String, recoveryInclusion: RecoveryInclusionPreference?) async throws -> ExportSequenceResult {
        active += 1; maximumActive = max(maximumActive, active)
        defer { active -= 1 }
        calls.append(Call(session:sessionID,sequence:sequenceID,destination:destinationFolder,stem:filenameStem,inclusion:recoveryInclusion))
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
    var sourceURL: URL? = URL(fileURLWithPath:"/source/Project")
    var projectPrompts = 0
    var destinationPrompts = 0
    func source() -> URL? { projectPrompts += 1; return sourceURL }
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
        // The actor must supply the async protocol witness, not its test fallback.
        let nativePartialCore = PhoenixCore()
        do {
            _ = try await nativePartialCore.partialRecovery(sessionID: "missing", sequenceID: "missing",
                inclusion: .includeMuted, destination: "", consent: nil)
            fatalError("Unknown native partial session accepted")
        } catch {
            check(String(describing: error).contains("No trustworthy supported partial recovery"), "Concrete partial actor reaches Rust transport")
        }
        let nativePartialProtocol: any PhoenixCoreServing = nativePartialCore
        do {
            _ = try await nativePartialProtocol.partialRecovery(sessionID: "missing", sequenceID: "missing",
                inclusion: .includeMuted, destination: "", consent: nil)
            fatalError("Unknown partial protocol session accepted")
        } catch {
            check(String(describing: error).contains("No trustworthy supported partial recovery"), "Partial protocol witness reaches Rust transport")
        }

        // CCA-101: preference lifecycle and captured batch/single contexts.
        do {
            let core = ProbeCore(); await core.enableRecovery(); let pickers = Pickers()
            let model = AppModel(core: core, chooseProject: {pickers.source()}, chooseDestination: {pickers.folder()})
            await until { model.state == .ready(contractVersion: 1) }
            check(!model.excludeMutedTracks, "Default unchecked")
            model.openProject(); await until { if case .inspected = model.projectState { return true }; return false }
            check(model.canChangeRecoveryInclusion, "Qualified recovery preference enabled")
            model.setExcludeMutedTracks(true)
            pickers.sourceURL = nil; model.openProject()
            check(model.excludeMutedTracks, "Canceled source picker preserves preference")
            model.setRecoverySelected("session-1-0", selected: true); model.setRecoverySelected("session-1-1", selected: true)
            pickers.destination = nil; model.recoverSelectedSequences()
            check(model.excludeMutedTracks && model.batchRecovery == nil, "Canceled destination preserves preference")
            check(await core.callCount() == 0, "Canceled picker dispatches nothing")
            pickers.destination = URL(fileURLWithPath: "/recovered")
            model.recoverSelectedSequences(); await until { await core.pendingCount() == 1 }
            check(!model.canChangeRecoveryInclusion, "Preference disabled while busy")
            model.setExcludeMutedTracks(false)
            check(model.excludeMutedTracks && model.batchRecovery?.snapshot.recoveryInclusion == .excludeMuted, "Busy changes rejected; batch snapshot frozen")
            try await core.finish(); await until { let count = await core.callCount(); let pending = await core.pendingCount(); return count == 2 && pending == 1 }
            try await core.finish(); await until { model.batchRecovery?.isRunning == false }
            check(await core.allCalls().map(\.inclusion) == [.excludeMuted,.excludeMuted], "Every batch item uses captured preference")
            model.selectedSequenceID = "session-1-0"; model.setExcludeMutedTracks(false); model.exportSelectedSequence()
            await until { await core.pendingCount() == 1 }
            model.setExcludeMutedTracks(true)
            if case .exporting(let context) = model.exportState { check(context.recoveryInclusion == .includeMuted, "Single context frozen") }
            else { fatalError("Expected single export") }
            check(await core.allCalls().last?.inclusion == .includeMuted, "Single request uses captured include mode")
            try await core.finish(); await until { !model.isExporting }
            model.setExcludeMutedTracks(true); model.exportSelectedSequence()
            await until { await core.pendingCount() == 1 }
            pickers.sourceURL = URL(fileURLWithPath: "/source/Other")
            model.openProject(); await until { await core.openCount() == 2 && model.projectState != .inspecting }
            check(!model.excludeMutedTracks, "Successful new project resets unchecked")
            check(await core.allCalls().last?.inclusion == .excludeMuted, "In-flight single retains exclusion after new-project UI reset")
            if case .exporting(let context) = model.exportState { check(context.recoveryInclusion == .excludeMuted, "Context survives actual preference reset") }
            else { fatalError("Expected in-flight single") }
            try await core.finish(); await until { !model.isExporting }
        }
        // Real Swift actor -> JSON -> C ABI -> AppService -> transactional output.
        do {
            let directory = FileManager.default.temporaryDirectory.appendingPathComponent("phoenix-cca101-" + UUID().uuidString)
            try FileManager.default.createDirectory(at: directory, withIntermediateDirectories: true)
            defer { try? FileManager.default.removeItem(at: directory) }
            let original = URL(fileURLWithPath: "/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/CCA073 CONTROL")
            let bytes = try Data(contentsOf: original); let copy = directory.appendingPathComponent("renamed.svp")
            try bytes.write(to: copy)
            let real = PhoenixCore(); check(try await real.handshake() == 1, "ABI/contract unchanged")
            let inspection = try await real.inspectProject(path: copy.path)
            let sequence = inspection.sequences[0]
            check(sequence.supportsRecoveryInclusion, "Real CONTROL capability decoded")
            for mode in [RecoveryInclusionPreference.includeMuted, .excludeMuted] {
                let result = try await real.exportSequence(sessionID: inspection.sessionID, sequenceID: sequence.sequenceID,
                    destinationFolder: directory.path, filenameStem: mode.rawValue, recoveryInclusion: mode)
                let midi = try Data(contentsOf: URL(fileURLWithPath: result.outputPath)); let count = mode == .includeMuted ? 14 : 13
                check(result.totalSMFTrackCount == count && Int(midi[11]) == count, "Real bridge publishes requested mode")
            }
            let jurrasic = inspection.sequences[13]
            check(jurrasic.supportsRecoveryInclusion, "Real Jurrasic capability decoded")
            for mode in [RecoveryInclusionPreference.includeMuted, .excludeMuted] {
                let result = try await real.exportSequence(sessionID: inspection.sessionID, sequenceID: jurrasic.sequenceID,
                    destinationFolder: directory.path, filenameStem: "Jurrasic-" + mode.rawValue, recoveryInclusion: mode)
                let midi = try Data(contentsOf: URL(fileURLWithPath: result.outputPath))
                let count = mode == .includeMuted ? 6 : 5
                check(result.totalSMFTrackCount == count && Int(midi[11]) == count, "Real Jurrasic bridge mode")
            }
            check(try Data(contentsOf: copy) == bytes && Data(contentsOf: original) == bytes, "Original and copy unchanged")
        }
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
        check(model.workspaceTab == .sequences && model.currentProjectRecovery == nil,"New project Sequences / empty results")
        let focusBeforeTabs = model.selectedSequenceID
        let selectionBeforeTabs = model.recoverySelection
        model.workspaceTab = .details
        model.workspaceTab = .results
        check(model.selectedSequenceID == focusBeforeTabs && model.recoverySelection == selectionBeforeTabs,"Tabs independent of focus and recovery selection")
        model.workspaceTab = .sequences
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
        let activeSelection = model.recoverySelection
        let activeFocus = model.selectedSequenceID
        model.workspaceTab = .details
        model.workspaceTab = .results
        check(model.batchRecovery?.snapshot == snapshot && model.recoverySelection == activeSelection && model.selectedSequenceID == activeFocus,"Active batch/focus/selection unchanged by tabs")
        model.workspaceTab = .sequences
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
        check(model.workspaceTab == .results && model.currentProjectRecovery != nil,"Partial completion opens current results")
        let selectionAfterRecovery = model.recoverySelection
        let resultAfterRecovery = model.batchRecovery
        model.workspaceTab = .sequences
        check(model.recoverySelection == selectionAfterRecovery && model.batchRecovery == resultAfterRecovery && model.canRecoverSelection,"Return preserves selection/results/actions")
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
        check(model.workspaceTab == .results && model.retainedExportDestination != nil,"Success opens results / last destination retained")
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
            check(fatalModel.workspaceTab == .results,"Failure/stopped partial completion opens results")
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
        check(PhoenixStartupPresentation.logoResource == "PhoenixStartupLogo","Bundled startup logo identity")
        check(RecoveryDetailsPresentation.helpInvitation == "Want help recovering a blocked Sequence?","Friendly blocked help invitation")
        check(RecoveryDetailsPresentation.helpContact == "Send your original Studio Vision project to soundzilla@yahoo.com. It may help us add support for the data Phoenix found.","Friendly help / bounded support possibility")
        let cautions = [ProjectWarning(message:"Same caution",scope:.project,severity:.caution),
                        ProjectWarning(message:"Same caution",scope:.sequence,severity:.informational),
                        ProjectWarning(message:"Different caution",scope:.project,severity:.caution)]
        let originalCautions = cautions
        check(RecoveryDetailsPresentation.cautionMessages(cautions) == ["Same caution","Different caution"],"Exact caution dedup / distinct retained")
        check(cautions == originalCautions && cautions.count == 3,"Underlying diagnostic records untouched")
        check(RecoveryDetailsPresentation.technicalTitle == "Technical Details" && RecoveryDetailsPresentation.technicalExplanation == "Most users will never need this information.","Secondary technical disclosure copy")
        check(RecoveryDetailsPresentation.cautionsAnswer.contains("can be opened in your DAW") && RecoveryDetailsPresentation.cautionsAnswer.contains("not translated"),"Bounded caution explanation")
        check(RecoveryDetailsPresentation.fidelityNote.contains("does not establish") && RecoveryDetailsPresentation.fidelityNote.contains("historical playback fidelity"),"No historical fidelity guarantee")
        check(RecoveryDetailsPresentation.blockedQuestion.contains("Not yet recoverable") && RecoveryDetailsPresentation.blockedAnswer.contains("rather than guessing or discarding musical information"),"Blocked recovery explanation")
        check(RecoveryDetailsPresentation.contact == "soundzilla@yahoo.com","Copyable developer contact")
        check(RecoveryWorkspaceTab.allCases.map(\.rawValue) == ["Sequences","Details","Recovery Results"],"All three native workspaces always available")
        check(RecoveryWorkspaceTab.emptyResults == "No MIDI has been recovered yet.","No-results workspace empty copy")
        check(RecoveryWorkspaceCopy.emptyTitle == "Resurrect MIDI from Studio Vision projects.","UI2A start preserved")
        model.openProject()
        await until { await core.openCount() == 2 }
        await until { model.canOpenProject }
        check(model.workspaceTab == .sequences && model.currentProjectRecovery == nil,"New project resets tab and excludes previous result")

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
        // Exact output hashes from an isolated build of parent 2389e5d.
        // Historical SCHOOL exports predate existing Controller routing fixes;
        // Rust recovery120 tests retain that detailed historical reconciliation.
        let baselineHashes = [
            "DANCING":"b4f7c2881e091654edcfedf1dab3967a7d919c6d5149bc113eb83bb2e780cff0",
            "NOTHING":"661d5fb6e9f4e7876c12606c28c84e9a034aeaa882b0da7fc66ec9cbb674c970",
            "Nothing FINAL":"92e5d3eca95fc5ad26cd0dca18a62ca47d201eb60f68f9c9be0af25652c4bc41",
            "Sequence H":"b7b0d39da264919f273e36886eae4913a5ff86dc8d6122a9576ca96c31935cc5",
            "Unnamed Sequence 09":"a4421055e4f6d6915e7b04e196b87590f3689735a768a0d25ca9b62feca00fc2",
            "Wind ensemble":"eadab495c4e8e8befae4c9ccef05522c43d573199a9e2d6d4d013ee372a78b28",
            "Tuba thing":"4085abaa837942e426a6c95895b414ce77345488ba2e2fcb4d73d4470f97bcc5",
            "Sequence M":"e8f988e8a7b983588ef12dd1c0f26a5fb9ac102f6fcace85529f59f8a39ec21c"
        ]
        check(SHA256.hash(data: sourceBefore).map {String(format:"%02x",$0)}.joined() == "bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331", "Authenticated SCHOOL identity")
        for entry in batch.results {
            guard case .succeeded(let result) = entry.outcome, let name = names[entry.item.ordinal] else { fatalError("Unexpected result") }
            let bytes = try Data(contentsOf:URL(fileURLWithPath:result.outputPath))
            check(SHA256.hash(data:bytes).map {String(format:"%02x",$0)}.joined() == baselineHashes[name],"Actual baseline output parity \(name)")
        }
        check(try Data(contentsOf:occupied) == sentinel,"Actual no-overwrite preserved")
        check(try Data(contentsOf:source) == sourceBefore,"Authentic source unchanged")
        print("UI2B workflow smoke: \(checks) checks passed")
    }
}
