import Foundation
import Phoenix

private struct APIInfoResult: Decodable {
    let contractVersion: UInt32

    enum CodingKeys: String, CodingKey {
        case contractVersion = "contract_version"
    }
}

private struct APIInfoEnvelope: Decodable {
    let ok: Bool
    let result: APIInfoResult?
    let error: ResponseError?
}

private struct ResponseError: Decodable {
    let kind: String
    let code: String?
    let message: String?
    let appError: AppErrorPayload?

    enum CodingKeys: String, CodingKey {
        case kind
        case code
        case message
        case appError = "app_error"
    }
}

private struct AppErrorPayload: Decodable {
    let category: String
    let displayMessage: String
    let technicalMessage: String
    let diagnosticCode: String

    enum CodingKeys: String, CodingKey {
        case category
        case displayMessage = "display_message"
        case technicalMessage = "technical_message"
        case diagnosticCode = "diagnostic_code"
    }
}

private struct InspectEnvelope: Decodable {
    let ok: Bool
    let result: ProjectInspection?
    let error: ResponseError?
}

private struct InspectRequest: Encodable {
    let operation = "inspect_project"
    let contractVersion: UInt32
    let payload: Payload

    struct Payload: Encodable {
        let sourcePath: String
        let diagnosticsLevel = "summary"

        enum CodingKeys: String, CodingKey {
            case sourcePath = "source_path"
            case diagnosticsLevel = "diagnostics_level"
        }
    }

    enum CodingKeys: String, CodingKey {
        case operation
        case contractVersion = "contract_version"
        case payload
    }
}

private enum PhoenixCoreError: LocalizedError {
    case abiMismatch(UInt32)
    case status(Int32)
    case invalidResponse
    case transport(String)
    case application(String)

    var errorDescription: String? {
        switch self {
        case .abiMismatch(let version): return "Unsupported Phoenix ABI version \(version)."
        case .status(let status): return "Phoenix call failed (status \(status))."
        case .invalidResponse: return "Phoenix returned an invalid handshake response."
        case .transport(let message): return "Phoenix transport error: \(message)"
        case .application(let message): return "Phoenix Core error: \(message)"
        }
    }
}

actor PhoenixCore {
    private var handle: phoenix_service_handle_t = 0
    private var destroyed = false

    deinit {
        if handle != 0 {
            _ = phoenix_service_destroy(handle)
        }
    }

    func handshake() throws -> UInt32 {
        guard phoenix_abi_version() == 1 else {
            throw PhoenixCoreError.abiMismatch(phoenix_abi_version())
        }

        if handle == 0 {
            var created: phoenix_service_handle_t = 0
            let status = phoenix_service_create(&created)
            guard status == 0, created != 0 else {
                throw PhoenixCoreError.status(status)
            }
            handle = created
        }

        let request = Data("{\"operation\":\"get_api_info\",\"payload\":{}}".utf8)
        let data = try Self.call(handle: handle, request: request)

        let envelope: APIInfoEnvelope
        do {
            envelope = try JSONDecoder().decode(APIInfoEnvelope.self, from: data)
        } catch {
            throw PhoenixCoreError.invalidResponse
        }
        guard envelope.ok else {
            guard let error = envelope.error else { throw PhoenixCoreError.invalidResponse }
            if error.kind == "transport" {
                throw PhoenixCoreError.transport(error.message ?? error.code ?? "unknown error")
            }
            throw PhoenixCoreError.application(error.appError?.displayMessage ?? "unknown error")
        }
        guard let result = envelope.result else { throw PhoenixCoreError.invalidResponse }
        return result.contractVersion
    }

    func inspectProject(path: String) throws -> ProjectInspection {
        let request = InspectRequest(contractVersion: 1, payload: .init(sourcePath: path))
        let requestData = try JSONEncoder().encode(request)
        let data = try Self.call(handle: ensureHandle(), request: requestData)
        let envelope: InspectEnvelope
        do {
            envelope = try JSONDecoder().decode(InspectEnvelope.self, from: data)
        } catch {
            throw PhoenixCoreError.invalidResponse
        }
        guard envelope.ok else {
            guard let error = envelope.error else { throw PhoenixCoreError.invalidResponse }
            if error.kind == "transport" {
                throw PhoenixCoreError.transport(error.message ?? error.code ?? "unknown error")
            }
            throw PhoenixCoreError.application(error.appError?.displayMessage ?? "unknown error")
        }
        guard let result = envelope.result else { throw PhoenixCoreError.invalidResponse }
        return result
    }

    private func ensureHandle() throws -> phoenix_service_handle_t {
        if handle != 0 { return handle }
        var created: phoenix_service_handle_t = 0
        let status = phoenix_service_create(&created)
        guard status == 0, created != 0 else { throw PhoenixCoreError.status(status) }
        handle = created
        return created
    }

    private nonisolated static func call(handle: phoenix_service_handle_t,
                                         request: Data) throws -> Data {
        var response = phoenix_buffer_t(ptr: nil, len: 0)
        let status = request.withUnsafeBytes { bytes in
            phoenix_call(handle, bytes.bindMemory(to: UInt8.self).baseAddress,
                         request.count, &response)
        }
        guard status == 0 else { throw PhoenixCoreError.status(status) }
        guard let pointer = response.ptr, response.len > 0 else {
            throw PhoenixCoreError.invalidResponse
        }

        let data = Data(bytes: pointer, count: response.len)
        let freeStatus = phoenix_free_buffer(response)
        guard freeStatus == 0 else { throw PhoenixCoreError.status(freeStatus) }
        return data
    }

    private func destroy() {
        guard !destroyed, handle != 0 else { return }
        _ = phoenix_service_destroy(handle)
        destroyed = true
        handle = 0
    }
}
