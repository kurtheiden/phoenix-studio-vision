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
            case .failed(let message):
                Text("Phoenix Core connection failed").font(.headline)
                Text(message).multilineTextAlignment(.center)
            }
        }
        .padding(40)
        .frame(minWidth: 420, minHeight: 240)
    }
}
