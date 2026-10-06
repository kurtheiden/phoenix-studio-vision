import SwiftUI

struct ContentView: View {
    @ObservedObject var model: AppModel
    @State private var resultsExpanded = false

    var body: some View {
        VStack(alignment: .leading, spacing: 20) {
            switch model.state {
            case .starting:
                startContent {
                    ProgressView("Starting Phoenix…")
                }
            case .ready:
                if RecoveryResultsLayout.usesSplit(hasResults: model.batchRecovery?.results.isEmpty == false,
                                                  expanded: resultsExpanded) {
                    VSplitView {
                        ScrollView {
                            projectContent
                                .frame(maxWidth: .infinity, alignment: .leading)
                        }
                        .frame(minHeight: 160)
                        recoveryWorkspace
                            .frame(minHeight: 140)
                    }
                } else {
                    projectContent
                    recoveryWorkspace
                }
            case .failed(let message):
                startContent {
                    Text("Phoenix could not start").font(.headline)
                    Text(message).foregroundStyle(.secondary)
                }
            }
        }
        .onChange(of: model.batchRecovery?.snapshot.id) { _ in resultsExpanded = false }
        .padding(24)
        .frame(minWidth: 640, minHeight: 620, alignment: .topLeading)
    }

    @ViewBuilder
    private var recoveryWorkspace: some View {
        if let batch = model.batchRecovery {
            if resultsExpanded {
                ScrollView {
                    BatchRecoveryView(model: model, batch: batch, detailsExpanded: $resultsExpanded)
                        .frame(maxWidth: .infinity, alignment: .leading)
                        .padding(.top, 8)
                }
            } else {
                BatchRecoveryView(model: model, batch: batch, detailsExpanded: $resultsExpanded)
            }
        } else {
            LatestExportView(model: model)
        }
    }

    private func startContent<Content: View>(@ViewBuilder content: () -> Content) -> some View {
        VStack(spacing: 16) {
            Text("Phoenix").font(.largeTitle.weight(.semibold))
            content()
        }
        .multilineTextAlignment(.center)
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }

    @ViewBuilder
    private var projectContent: some View {
        switch model.projectState {
        case .idle:
            startContent {
                Text(RecoveryWorkspaceCopy.emptyTitle).font(.title3)
                Text("Open your project to see which sequences Phoenix can safely recover.")
                    .foregroundStyle(.secondary)
                Button(RecoveryWorkspaceCopy.openProject) { model.openProject() }
                    .buttonStyle(.borderedProminent)
            }
        case .inspecting:
            startContent {
                ProgressView("Checking which sequences can be recovered…")
            }
        case .inspected(let inspection):
            ProjectInspectionView(model: model, inspection: inspection)
                .id(inspection.sessionID)
        case .failed(let message):
            startContent {
                Text("Could not open this project").font(.headline)
                Text(message).foregroundStyle(.secondary)
                Button(RecoveryWorkspaceCopy.openProject) { model.openProject() }
            }
        }
    }
}

private struct ProjectInspectionView: View {
    @ObservedObject var model: AppModel
    let inspection: ProjectInspection
    @State private var detailsExpanded = false

    private var selectedSequence: SequenceViewData? {
        inspection.sequences.first { $0.sequenceID == model.selectedSequenceID }
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            HStack(alignment: .top, spacing: 20) {
                VStack(alignment: .leading, spacing: 6) {
                    Text("Phoenix").font(.headline).foregroundStyle(.secondary)
                    Text(inspection.displayName).font(.title.weight(.semibold))
                        .lineLimit(2).textSelection(.enabled)
                    Text(inspection.recognizedStudioVision
                         ? "Studio Vision project" : "File not recognized as a supported Studio Vision project")
                        .foregroundStyle(.secondary)
                    Text(inspection.recoverySummary).font(.callout)
                        .accessibilityLabel(inspection.recoverySummary)
                }
                Spacer(minLength: 0)
                Button("Open Another Project…") { model.openProject() }
                    .disabled(!model.canOpenProject)
                    .controlSize(.small)
            }

            HStack {
                Text("Sequences").font(.headline)
                Spacer()
                Text("\(model.selectedRecoveryCount) selected").foregroundStyle(.secondary)
                Button("Select All Recoverable") { model.selectAllRecoverable() }
                    .disabled(model.isExporting || inspection.recoverableSequenceCount == 0)
                Button("Clear Selection") { model.clearRecoverySelection() }
                    .disabled(model.isExporting || model.selectedRecoveryCount == 0)
            }
            .controlSize(.small)
            if inspection.sequences.isEmpty {
                VStack(spacing: 12) {
                    Label("No sequences found", systemImage: "music.note.list").font(.headline)
                    Text("Phoenix could not identify sequences using its currently supported project structures.")
                        .foregroundStyle(.secondary)
                }
                .frame(maxWidth: .infinity, maxHeight: .infinity)
            } else {
                List(Array(inspection.sequences.enumerated()), id: \.element.sequenceID,
                     selection: $model.selectedSequenceID) { index, sequence in
                    HStack(spacing: 12) {
                        Toggle("Select for recovery", isOn: Binding(
                            get: { model.recoverySelection.contains(sequence.sequenceID) },
                            set: { model.setRecoverySelected(sequence.sequenceID, selected: $0) }
                        ))
                        .toggleStyle(.checkbox).labelsHidden()
                        .disabled(!sequence.isExportEligible || model.isExporting)
                        .accessibilityLabel("Select \(sequence.presentationName(ordinal: index + 1)) for recovery")
                        Text("\(index + 1)")
                            .monospacedDigit().foregroundStyle(.secondary)
                            .frame(width: 28, alignment: .trailing)
                        Text(sequence.presentationName(ordinal: index + 1)).font(.body.weight(.medium))
                            .lineLimit(2)
                        Spacer(minLength: 12)
                        Label(sequence.recoveryStatus,
                              systemImage: sequence.isExportEligible ? "checkmark.circle" : "minus.circle")
                            .font(.callout)
                            .foregroundStyle(sequence.isExportEligible ? Color.primary : Color.secondary)
                    }
                    .padding(.vertical, 7)
                    .tag(sequence.sequenceID)
                    .help(sequence.recoveryExplanation)
                    .accessibilityElement(children: .contain)
                    .accessibilityLabel("Sequence \(index + 1), \(sequence.presentationName(ordinal: index + 1)), \(sequence.recoveryStatus)")
                }
                .disabled(model.batchRecovery?.isRunning == true)
                .listStyle(.inset)
                .frame(minHeight: 260)
            }

            Divider()
            HStack(alignment: .center, spacing: 20) {
                VStack(alignment: .leading, spacing: 5) {
                    Text(model.selectedRecoveryCount == 0 ? "Select sequences to recover" : "\(model.selectedRecoveryCount) selected")
                        .font(.headline)
                    if let sequence = selectedSequence {
                        Text(sequence.isExportEligible ? "Ready to recover" : "Not yet recoverable")
                            .font(.callout)
                        Text(sequence.recoveryExplanation).font(.callout).foregroundStyle(.secondary)
                        if let tracks = sequence.musicalTrackCount {
                            Text("\(tracks) tracks").font(.caption).foregroundStyle(.secondary)
                        }
                    } else {
                        Text("Sequences not yet recoverable remain visible.")
                            .font(.callout).foregroundStyle(.secondary)
                    }
                }
                Spacer(minLength: 0)
                VStack(alignment: .trailing, spacing: 8) {
                    Button(model.recoveryActionTitle) { model.recoverSelectedSequences() }
                        .buttonStyle(.borderedProminent)
                        .disabled(!model.canRecoverSelection)
                        .accessibilityHint("Choose one folder and recover the checked sequences as MIDI")
                    if let destination = model.retainedExportDestination {
                        Button("Recover to Last Folder") { model.recoverSelectedSequences(toRetainedDestination: true) }
                            .disabled(!model.canRecoverSelection)
                            .controlSize(.small)
                        Text(destination.lastPathComponent).font(.caption).foregroundStyle(.secondary)
                            .lineLimit(1).truncationMode(.middle).help(destination.path)
                    }
                }
            }

            if inspection.warnings.contains(where: { $0.severity == .dataLossRisk }) {
                Label("Review the recovery notes in Details.", systemImage: "exclamationmark.triangle")
                    .font(.callout)
            }
            DisclosureGroup("Details", isExpanded: Binding(
                get: { detailsExpanded },
                set: { expanded in
                    detailsExpanded = expanded
                    if expanded { model.loadDiagnosticsIfNeeded() }
                }
            )) {
                ScrollView {
                    VStack(alignment: .leading, spacing: 12) {
                        if !inspection.warnings.isEmpty {
                            Text("Recovery notes").font(.headline)
                            ForEach(Array(inspection.warnings.enumerated()), id: \.offset) { _, warning in
                                VStack(alignment: .leading, spacing: 3) {
                                    Text(warning.message)
                                    Text(warning.severity.displayName).font(.caption).foregroundStyle(.secondary)
                                }
                            }
                        }
                        if let sequence = selectedSequence {
                            Text("Selected sequence details").font(.headline)
                            Text(sequence.readinessReason.displayDetail)
                        }
                        Text("Project information").font(.headline)
                        if let source = model.inspectedSourceURL {
                            Text(source.path).textSelection(.enabled)
                        }
                        Text(ByteCountFormatter.string(fromByteCount: Int64(clamping: inspection.byteSize), countStyle: .file))
                        Text("Project support: \(inspection.overallReadiness.displayName)")
                        if case .ready(let version) = model.state {
                            Text("Application contract version \(version)")
                        }
                        if inspection.diagnosticsAvailable { diagnosticsContent }
                    }
                    .font(.callout)
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding(.top, 8)
                }
                .frame(maxHeight: 160)
            }
        }
    }

    @ViewBuilder
    private var diagnosticsContent: some View {
        switch model.diagnosticsState {
        case .notLoaded, .loading:
            ProgressView("Loading technical details…")
        case .failed(let message):
            Text("Technical details unavailable: \(message)").foregroundStyle(.secondary)
        case .loaded(let summary):
            Text("Phoenix Core \(summary.coreVersion)")
            if let profile = summary.recognizedProfile { Text("Recognized profile: \(profile)") }
            if let status = summary.structuralStatus { Text("Structural status: \(status)") }
            if let capability = summary.compatibilityProfile { Text("Compatibility: \(capability.displayLabel)") }
            if !summary.unsupportedFamilies.isEmpty {
                Text("Unsupported event families: \(summary.unsupportedFamilies.joined(separator: ", "))")
            }
        }
    }
}

private struct LatestExportView: View {
    @ObservedObject var model: AppModel
    @State private var detailsExpanded = false

    @ViewBuilder
    var body: some View {
        switch model.exportState {
        case .idle:
            EmptyView()
        case .exporting(let context):
            HStack {
                ProgressView()
                VStack(alignment: .leading, spacing: 4) {
                    Text("Recovering MIDI…").font(.headline)
                    Text(context.sequenceDisplayName).lineLimit(2)
                    previousProject(context)
                }
            }
            .accessibilityElement(children: .combine)
        case .failed(let context, let message):
            VStack(alignment: .leading, spacing: 5) {
                Text("MIDI could not be recovered").font(.headline)
                Text(context.sequenceDisplayName).lineLimit(2)
                previousProject(context)
                Text(message).foregroundStyle(.secondary)
                DisclosureGroup("Recovery attempt details") {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("Source: \(context.sourceURL.path)")
                        Text("Destination: \(context.destination.path)")
                    }
                    .font(.caption).textSelection(.enabled)
                }
            }
        case .succeeded(let context, let result):
            VStack(alignment: .leading, spacing: 8) {
                HStack {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("MIDI recovered").font(.headline)
                        Text(result.sequenceDisplayName).lineLimit(2)
                        Text("Saved to \(context.destination.lastPathComponent)")
                            .font(.callout).foregroundStyle(.secondary)
                            .help(result.outputPath)
                        previousProject(context)
                    }
                    Spacer()
                    Button("Reveal in Finder") { ExportDestinationPanel.revealInFinder(path: result.outputPath) }
                }
                DisclosureGroup("Recovery result details", isExpanded: $detailsExpanded) {
                    ScrollView {
                        VStack(alignment: .leading, spacing: 5) {
                            Text("Source: \(context.sourceURL.path)")
                            Text(result.outputPath).textSelection(.enabled)
                            Text("\(result.musicalTrackCount) musical tracks · \(result.totalSMFTrackCount) SMF tracks")
                            if result.untranslatedMetadataCount > 0 {
                                Text("\(result.untranslatedMetadataCount) untranslated metadata items")
                            }
                            ForEach(Array(result.warnings.enumerated()), id: \.offset) { _, warning in
                                Text(warning.message).accessibilityLabel("Recovery note: \(warning.message)")
                            }
                        }
                        .frame(maxWidth: .infinity, alignment: .leading)
                    }
                    .font(.caption).frame(maxHeight: 100)
                }
                if result.warnings.contains(where: { $0.severity == .dataLossRisk }) {
                    Label("Review the recovery result notes.", systemImage: "exclamationmark.triangle")
                        .font(.callout)
                }
            }
            .padding(.top, 8)
        }
    }

    @ViewBuilder
    private func previousProject(_ context: AppModel.ExportContext) -> some View {
        if case .inspected(let current) = model.projectState, current.sessionID != context.sessionID {
            Text("From previously opened project: \(context.sourceURL.lastPathComponent)")
                .font(.caption).foregroundStyle(.secondary)
        }
    }
}

private struct BatchRecoveryView: View {
    @ObservedObject var model: AppModel
    let batch: AppModel.BatchRecovery
    @Binding var detailsExpanded: Bool

    var body: some View {
        VStack(alignment: .leading, spacing: 10) {
            Text("Most recent recovery · \(batch.snapshot.sourceURL.lastPathComponent)")
                .font(.caption).foregroundStyle(.secondary)
                .lineLimit(1).truncationMode(.middle).help(batch.snapshot.sourceURL.path)
            if batch.isRunning, let index = batch.currentIndex,
               batch.snapshot.items.indices.contains(index) {
                HStack {
                    ProgressView()
                    VStack(alignment: .leading, spacing: 4) {
                        Text("Recovering \(index + 1) of \(batch.snapshot.items.count)…").font(.headline)
                        Text(batch.snapshot.items[index].displayName).lineLimit(2)
                    }
                }
                .accessibilityElement(children: .combine)
            } else {
                HStack(alignment: .top, spacing: 16) {
                    VStack(alignment: .leading, spacing: 4) {
                        Label(batch.summary, systemImage: batch.isAllSuccessful ? "checkmark.circle" : "exclamationmark.circle")
                            .font(.headline)
                        if batch.failureCount > 0 { Text("\(batch.failureCount) could not be recovered.") }
                        if batch.notAttemptedCount > 0 { Text("\(batch.notAttemptedCount) not attempted because recovery stopped.") }
                        Text("Destination: \(batch.snapshot.destination.lastPathComponent)")
                            .font(.callout).foregroundStyle(.secondary)
                            .lineLimit(1).truncationMode(.middle).help(batch.snapshot.destination.path)
                            .accessibilityLabel("Recovery destination: \(batch.snapshot.destination.path)")
                    }
                    Spacer(minLength: 0)
                    if batch.successCount > 0 {
                        Button("Reveal in Finder") { ExportDestinationPanel.revealInFinder(path: batch.snapshot.destination.path) }
                    }
                }
                if let failure = batch.failureContext {
                    VStack(alignment: .leading, spacing: 4) {
                        Text(failure.message).font(.callout)
                        Text(failure.nextStep).font(.callout).foregroundStyle(.secondary)
                    }
                }
            }
            if case .inspected(let current) = model.projectState, current.sessionID != batch.snapshot.sessionID {
                Text("From a previously opened project")
                    .font(.caption).foregroundStyle(.secondary)
            }
            if !batch.results.isEmpty {
                DisclosureGroup("Recovery results", isExpanded: $detailsExpanded) {
                    VStack(alignment: .leading, spacing: 12) {
                        ForEach(batch.results, id: \.item.sequence.sequenceID) { entry in
                            resultRow(entry)
                        }
                        DisclosureGroup("Operation details") {
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Source: \(batch.snapshot.sourceURL.path)")
                                Text("Destination: \(batch.snapshot.destination.path)")
                            }
                            .font(.caption).textSelection(.enabled)
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding(.top, 4)
                }
            }
        }
    }

    private func resultRow(_ entry: AppModel.RecoveryResult) -> some View {
        VStack(alignment: .leading, spacing: 5) {
            HStack(alignment: .firstTextBaseline, spacing: 12) {
                Text(entry.item.displayName).font(.callout.weight(.medium)).lineLimit(2)
                Spacer(minLength: 0)
                Label(entry.outcome.status, systemImage: entry.outcome.statusSymbol).font(.callout)
            }
            .accessibilityElement(children: .combine)
            .accessibilityLabel("Sequence \(entry.item.ordinal), \(entry.item.displayName), \(entry.outcome.status)")
            switch entry.outcome {
            case .succeeded(let result):
                Text(URL(fileURLWithPath: result.outputPath).lastPathComponent)
                    .font(.caption).foregroundStyle(.secondary)
                    .lineLimit(1).truncationMode(.middle).help(result.outputPath)
                ForEach(Array(result.warnings.enumerated()), id: \.offset) { _, warning in
                    Text(warning.message).font(.caption)
                }
                DisclosureGroup("File details") {
                    Text(result.outputPath).font(.caption).textSelection(.enabled)
                }
            case .failed(let failure):
                Text(failure.message).font(.callout).foregroundStyle(.secondary)
                DisclosureGroup("Technical details") {
                    Text(failure.technicalDetail).font(.caption).textSelection(.enabled)
                }
            case .notAttempted:
                Text("Recovery stopped before this sequence was attempted.")
                    .font(.caption).foregroundStyle(.secondary)
            }
        }
    }
}
