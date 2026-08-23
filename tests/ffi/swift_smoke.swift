import Foundation
import Phoenix

// Swift's Clang importer does not import the header's INT32_C(...) macros.
// These typed values mirror the public numeric status contract without adding
// a Swift wrapper or changing the C ABI.
let statusOK: Int32 = 0
let statusInvalidHandle: Int32 = 2

@inline(__always)
func fail(_ message: String) -> Never {
    let output = "phoenix UI0G2 Swift smoke failure: \(message)\n"
    FileHandle.standardError.write(output.data(using: .utf8)!)
    exit(1)
}

func callJSON(handle: phoenix_service_handle_t, request: Data) -> Data {
    var response = phoenix_buffer_t(ptr: nil, len: 0)
    let status = request.withUnsafeBytes { rawBuffer -> Int32 in
        let bytes = rawBuffer.bindMemory(to: UInt8.self)
        return phoenix_call(handle, bytes.baseAddress, bytes.count, &response)
    }
    guard status == statusOK else {
        fail("phoenix_call returned status \(status)")
    }
    guard let pointer = response.ptr, response.len > 0 else {
        fail("phoenix_call returned an empty response")
    }
    // Phoenix owns response.ptr until this exact free. Copy first so only the
    // Swift-owned Data is inspected after the Phoenix allocation is released.
    let copied = Data(bytes: pointer, count: response.len)
    guard phoenix_free_buffer(response) == statusOK else {
        fail("phoenix_free_buffer failed")
    }
    return copied
}

func decode(_ data: Data) -> [String: Any] {
    do {
        guard let object = try JSONSerialization.jsonObject(with: data) as? [String: Any] else {
            fail("response was not a JSON object")
        }
        return object
    } catch {
        fail("response JSON decoding failed: \(error)")
    }
}

let request = Data(#"{"operation":"get_api_info","payload":{}}"#.utf8)
guard phoenix_abi_version() == 1 else {
    fail("unexpected ABI version")
}

var handle: phoenix_service_handle_t = 0
guard phoenix_service_create(&handle) == statusOK, handle != 0 else {
    fail("service creation failed")
}
var liveHandle: phoenix_service_handle_t? = handle
defer {
    if let liveHandle {
        _ = phoenix_service_destroy(liveHandle)
    }
}

let success = decode(callJSON(handle: handle, request: request))
guard let successOK = success["ok"] as? NSNumber,
      successOK.boolValue,
      let result = success["result"] as? [String: Any],
      let contractVersion = result["contract_version"] as? NSNumber,
      contractVersion.intValue == 1 else {
    fail("get_api_info response fields were unexpected")
}

guard phoenix_service_destroy(handle) == statusOK else {
    fail("service destruction failed")
}
liveHandle = nil

let stale = decode(callJSON(handle: handle, request: request))
guard let staleOK = stale["ok"] as? NSNumber,
      !staleOK.boolValue,
      let error = stale["error"] as? [String: Any],
      error["kind"] as? String == "transport",
      error["code"] as? String == "invalid_handle" else {
    fail("stale-handle response was unexpected")
}

guard phoenix_service_destroy(handle) == statusInvalidHandle else {
    fail("second destroy did not return invalid-handle status")
}

print("UI0G2 Swift interoperability smoke passed")
