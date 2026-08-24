import SwiftUI

struct ContentView: View {
    @ObservedObject var model: AppModel

    var body: some View {
        VStack(spacing: 16) {
            Text("Phoenix").font(.largeTitle)
            switch model.state {
            case .starting:
                ProgressView()
                Text("Connecting to Phoenix Core…")
            case .ready(let version):
                Text("Phoenix Core connected")
                Text("Application contract version \(version)")
                projectContent
            case .failed(let message):
                Text("Phoenix Core connection failed").font(.headline)
                Text(message).multilineTextAlignment(.center)
            }
        }
        .padding(40)
        .frame(minWidth: 640, minHeight: 440)
    }

    @ViewBuilder
    private var projectContent: some View {
        switch model.projectState {
        case .idle:
            Button("Open Project") { model.openProject() }
        case .inspecting:
            ProgressView()
            Text("Inspecting project…")
        case .inspected(let inspection):
            ProjectInspectionView(model: model, inspection: inspection)
                .id(inspection.sessionID)
        case .failed(let message):
            Text("Project inspection failed").font(.headline)
            Text(message).multilineTextAlignment(.center)
            Button("Try Another File") { model.openProject() }
        }
    }

}

private struct ProjectInspectionView: View {
    @ObservedObject var model: AppModel
    let inspection: ProjectInspection

    @State private var warningsExpanded = true
    @State private var detailsExpanded = false

    var body: some View {
        Text(inspection.displayName).font(.headline)
        Text(inspection.recognizedStudioVision ? "Studio Vision project recognized" : "File inspected")
        Label(inspection.overallReadiness.displayName, systemImage: "gauge.with.dots.needle.50percent")
            .accessibilityLabel("Project readiness: \(inspection.overallReadiness.displayName)")
        Text(ByteCountFormatter.string(fromByteCount: Int64(inspection.byteSize), countStyle: .file))
        Text("\(inspection.sequenceCount) sequences · \(inspection.warningCount) warnings")
        if !inspection.warnings.isEmpty {
            DisclosureGroup("Warnings (\(inspection.warnings.count))", isExpanded: $warningsExpanded) {
                VStack(alignment: .leading, spacing: 8) {
                    ForEach(Array(inspection.warnings.enumerated()), id: \.offset) { _, warning in
                        VStack(alignment: .leading, spacing: 2) {
                            Text(warning.message)
                            Text(warning.severity.displayName)
                                .font(.caption)
                                .foregroundStyle(.secondary)
                        }
                        .accessibilityElement(children: .combine)
                    }
                }
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding(.top, 4)
            }
        }
        if inspection.sequences.isEmpty {
            Text(inspection.recognizedStudioVision ? "No sequences found." : "No Studio Vision sequences found.")
                .foregroundStyle(.secondary)
        } else {
            List(Array(inspection.sequences.enumerated()), id: \.element.sequenceID,
                 selection: $model.selectedSequenceID) { index, sequence in
                VStack(alignment: .leading, spacing: 4) {
                    Text("\(index + 1). \(sequence.displayName)")
                    Text(sequence.readiness.displayName)
                        .font(.caption)
                    HStack(spacing: 12) {
                        if let tracks = sequence.musicalTrackCount {
                            Text("\(tracks) tracks")
                        }
                        if sequence.warningCount > 0 {
                            Text("\(sequence.warningCount) warnings")
                        }
                    }
                    .font(.caption)
                    .foregroundStyle(.secondary)
                }
                .padding(.vertical, 3)
                .tag(sequence.sequenceID)
                .accessibilityLabel("\(sequence.displayName), \(sequence.readiness.displayName)")
            }
            .frame(minHeight: 160)
        }
        if let sequence = inspection.sequences.first(where: { $0.sequenceID == model.selectedSequenceID }) {
            GroupBox("Selected sequence") {
                VStack(alignment: .leading, spacing: 5) {
                    Text(sequence.displayName).font(.headline)
                    Label(sequence.readiness.displayName, systemImage: "gauge.with.dots.needle.50percent")
                        .accessibilityLabel("Sequence readiness: \(sequence.readiness.displayName)")
                    Text(sequence.readinessReason.displayDetail)
                    HStack(spacing: 12) {
                        if let tracks = sequence.musicalTrackCount { Text("\(tracks) tracks") }
                        Text("\(sequence.warningCount) warnings")
                    }
                    .font(.caption)
                    .foregroundStyle(.secondary)
                }
                .frame(maxWidth: .infinity, alignment: .leading)
            }
        }
        if inspection.diagnosticsAvailable {
            DisclosureGroup("Details", isExpanded: Binding(
                get: { detailsExpanded },
                set: { expanded in
                    detailsExpanded = expanded
                    if expanded { model.loadDiagnosticsIfNeeded() }
                }
            )) {
                diagnosticsContent
                    .padding(.top, 4)
            }
        }
        Button("Open Another Project") { model.openProject() }
    }

    @ViewBuilder
    private var diagnosticsContent: some View {
        switch model.diagnosticsState {
        case .notLoaded, .loading:
            HStack {
                ProgressView()
                Text("Loading details")
            }
            .accessibilityElement(children: .combine)
        case .failed(let message):
            VStack(alignment: .leading, spacing: 4) {
                Text("Details unavailable").font(.headline)
                Text(message).foregroundStyle(.secondary)
            }
            .accessibilityElement(children: .combine)
        case .loaded(let summary):
            VStack(alignment: .leading, spacing: 5) {
                Text("Phoenix Core \(summary.coreVersion)")
                if let profile = summary.recognizedProfile {
                    Text("Recognized profile: \(profile)")
                }
                if let status = summary.structuralStatus {
                    Text("Structural status: \(status)")
                }
                if let capability = summary.compatibilityProfile {
                    Text("Compatibility: \(capability.displayLabel)")
                }
                if !summary.unsupportedFamilies.isEmpty {
                    Text("Unsupported event families: \(summary.unsupportedFamilies.joined(separator: ", "))")
                }
            }
            .frame(maxWidth: .infinity, alignment: .leading)
        }
    }
}
