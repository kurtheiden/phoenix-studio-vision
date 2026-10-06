import SwiftUI

@main
struct PhoenixAppApp: App {
    @StateObject private var model = AppModel()

    var body: some Scene {
        WindowGroup {
            ContentView(model: model)
        }
        .defaultSize(width: 780, height: 740)
    }
}
