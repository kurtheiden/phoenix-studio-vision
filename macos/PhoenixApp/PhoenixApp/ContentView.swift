import SwiftUI

struct ContentView: View {
    @ObservedObject var model: AppModel

    var body: some View {
        VStack(alignment: .leading, spacing: 20) {
            switch model.state {
            case .starting:
                startContent {
                    ProgressView("Starting Phoenix…")
                }
            case .ready:
                projectContent
                if let batch = model.batchRecovery {
                    BatchRecoveryView(model: model, batch: batch)
                } else {
                    LatestExportView(model: model)
                }
            case .failed(let message):
                startContent {
                    Text("Phoenix could not start").font(.headline)
                    Text(message).foregroundStyle(.secondary)
                }
            }
        }
        .padding(24)
        .frame(minWidth: 640, minHeight: 620, alignment: .topLeading)
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
    @State private var detailsExpanded = false

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            if batch.isRunning, let index = batch.currentIndex,
               batch.snapshot.items.indices.contains(index) {
                HStack {
                    ProgressView()
                    VStack(alignment: .leading, spacing: 4) {
                        Text("Recovering \(index + 1) of \(batch.snapshot.items.count)…").font(.headline)
                        Text(batch.snapshot.items[index].displayName).lineLimit(2)
                    }
                }
            } else {
                HStack {
                    VStack(alignment: .leading, spacing: 4) {
                        Text(batch.summary).font(.headline)
                        if batch.failureCount > 0 { Text("\(batch.failureCount) could not be recovered.") }
                        if batch.notAttemptedCount > 0 { Text("\(batch.notAttemptedCount) not attempted because recovery stopped.") }
                        Text("Destination: \(batch.snapshot.destination.lastPathComponent)")
                            .font(.callout).foregroundStyle(.secondary).help(batch.snapshot.destination.path)
                    }
                    Spacer()
                    if batch.successCount > 0 {
                        Button("Reveal in Finder") { ExportDestinationPanel.revealInFinder(path: batch.snapshot.destination.path) }
                    }
                }
            }
            if case .inspected(let current) = model.projectState, current.sessionID != batch.snapshot.sessionID {
                Text("From previously opened project: \(batch.snapshot.sourceURL.lastPathComponent)")
                    .font(.caption).foregroundStyle(.secondary)
            }
            if !batch.results.isEmpty {
                DisclosureGroup("Recovery results", isExpanded: $detailsExpanded) {
                    ScrollView {
                        VStack(alignment: .leading, spacing: 12) {
                            Text("Source: \(batch.snapshot.sourceURL.path)").font(.caption).textSelection(.enabled)
                            ForEach(batch.results, id: \.item.sequence.sequenceID) { entry in
                                VStack(alignment: .leading, spacing: 3) {
                                    switch entry.outcome {
                                    case .succeeded(let result):
                                        Label("\(entry.item.displayName) — recovered", systemImage: "checkmark.circle")
                                        Text(result.outputPath).font(.caption).textSelection(.enabled)
                                        ForEach(Array(result.warnings.enumerated()), id: \.offset) { _, warning in
                                            Text(warning.message).font(.caption)
                                        }
                                    case .failed(let message, let technicalDetail):
                                        Label("\(entry.item.displayName) — not recovered", systemImage: "exclamationmark.circle")
                                        Text(message).font(.callout)
                                        if let technicalDetail {
                                            DisclosureGroup("Technical details") { Text(technicalDetail).font(.caption).textSelection(.enabled) }
                                        }
                                    case .notAttempted:
                                        Text("\(entry.item.displayName) — not attempted")
                                    }
                                }
                            }
                        }
                        .frame(maxWidth: .infinity, alignment: .leading)
                    }
                    .frame(maxHeight: 140)
                }
            }
        }
    }
}
