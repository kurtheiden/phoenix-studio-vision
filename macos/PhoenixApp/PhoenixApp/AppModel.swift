import Foundation

@MainActor
final class AppModel: ObservableObject {
    enum State: Equatable {
        case starting
        case ready(contractVersion: UInt32)
        case failed(message: String)
    }

    @Published private(set) var state: State = .starting
    private let core = PhoenixCore()
    private var started = false

    init() {
        startHandshake()
    }

    private func startHandshake() {
        guard !started else { return }
        started = true
        Task {
            do {
                let version = try await core.handshake()
                state = .ready(contractVersion: version)
                FileHandle.standardError.write(Data("UI1A_READY contract_version=\(version)\n".utf8))
            } catch {
                state = .failed(message: error.localizedDescription)
                FileHandle.standardError.write(Data("UI1A_FAILED \(error.localizedDescription)\n".utf8))
            }
        }
    }
}
