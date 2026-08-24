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
            Text(inspection.displayName).font(.headline)
            Text(inspection.recognizedStudioVision ? "Studio Vision project recognized" : "File inspected")
            Text(ByteCountFormatter.string(fromByteCount: Int64(inspection.byteSize), countStyle: .file))
            Text("\(inspection.sequenceCount) sequences · \(inspection.warningCount) warnings")
            if inspection.sequences.isEmpty {
                Text(inspection.recognizedStudioVision ? "No sequences found." : "No Studio Vision sequences found.")
                    .foregroundStyle(.secondary)
            } else {
                List(Array(inspection.sequences.enumerated()), id: \.element.sequenceID) { index, sequence in
                    VStack(alignment: .leading, spacing: 4) {
                        Text("\(index + 1). \(sequence.displayName)")
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
                }
                .frame(minHeight: 160)
            }
            Button("Open Another Project") { model.openProject() }
        case .failed(let message):
            Text("Project inspection failed").font(.headline)
            Text(message).multilineTextAlignment(.center)
            Button("Try Another File") { model.openProject() }
        }
    }
}
