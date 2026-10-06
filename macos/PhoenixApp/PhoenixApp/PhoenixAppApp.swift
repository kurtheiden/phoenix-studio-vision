import SwiftUI

@main
struct PhoenixAppApp: App {
    @StateObject private var model = AppModel()

    var body: some Scene {
        WindowGroup("Phoenix") {
            ContentView(model: model)
        }
        .windowStyle(.titleBar)
        .defaultSize(width: 780, height: 740)
    }
}
