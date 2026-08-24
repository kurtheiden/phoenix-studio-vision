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
        .frame(minWidth: 420, minHeight: 240)
    }

    @ViewBuilder
    private var projectContent: some View {
        switch model.projectState {
        case .idle:
            Button("Open Project") { model.openProject() }
        case .inspecting:
            ProgressView()
            Text("Inspecting project…")
        case .inspected(let summary):
            Text(summary.displayName).font(.headline)
            Text(summary.recognizedStudioVision ? "Studio Vision project recognized" : "File inspected")
            Text("\(summary.sequenceCount) sequences · \(summary.warningCount) warnings")
            Button("Open Another Project") { model.openProject() }
        case .failed(let message):
            Text("Project inspection failed").font(.headline)
            Text(message).multilineTextAlignment(.center)
            Button("Try Another File") { model.openProject() }
        }
    }
}
