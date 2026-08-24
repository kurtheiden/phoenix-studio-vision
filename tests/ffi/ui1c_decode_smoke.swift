import Foundation

func check(_ condition: @autoclosure () -> Bool, _ message: String) {
    guard condition() else { fputs("UI1C decode failure: \(message)\n", stderr); exit(1) }
}

let zero = #"{"session_id":"session-zero","project":{"display_name":"newest STUFF","byte_size":12,"recognized_studio_vision":true,"sequence_count":0,"warning_count":1},"sequences":[],"warnings":[{}],"diagnostics_available":true,"future":true}"#.data(using: .utf8)!
let unrecognized = #"{"session_id":"session-license","project":{"display_name":"LICENSE","byte_size":7,"recognized_studio_vision":false,"sequence_count":0,"warning_count":1},"sequences":[],"warnings":[{}],"diagnostics_available":true}"#.data(using: .utf8)!
let populated = #"{"session_id":"opaque-session","project":{"display_name":"Synthetic","byte_size":42,"recognized_studio_vision":true,"sequence_count":1,"warning_count":2},"sequences":[{"sequence_id":"opaque-sequence","display_name":"Synthetic Sequence","musical_track_count":3,"warning_count":1,"future_field":"ignored"}],"warnings":[{},{}],"diagnostics_available":true}"#.data(using: .utf8)!

let decoder = JSONDecoder()
let zeroResult = try decoder.decode(ProjectInspection.self, from: zero)
check(zeroResult.sessionID == "session-zero", "opaque zero session")
check(zeroResult.sequenceCount == 0 && zeroResult.sequences.isEmpty, "zero sequence response")
let unrecognizedResult = try decoder.decode(ProjectInspection.self, from: unrecognized)
check(!unrecognizedResult.recognizedStudioVision && unrecognizedResult.warningCount == 1, "unrecognized response")
let populatedResult = try decoder.decode(ProjectInspection.self, from: populated)
check(populatedResult.sequences.count == 1, "populated sequence count")
check(populatedResult.sequences[0].sequenceID == "opaque-sequence" && populatedResult.sequences[0].warningCount == 1, "opaque sequence fields")
check(populatedResult.sequenceCount == 1 && populatedResult.warningCount == 2, "explicit project counts")
print("UI1C Swift decoder smoke passed")
