import Foundation

struct SequenceViewData: Decodable, Equatable {
    let sequenceID: String
    let displayName: String
    let musicalTrackCount: UInt32?
    let warningCount: UInt32

    enum CodingKeys: String, CodingKey {
        case sequenceID = "sequence_id"
        case displayName = "display_name"
        case musicalTrackCount = "musical_track_count"
        case warningCount = "warning_count"
    }
}

struct ProjectInspection: Decodable, Equatable {
    let sessionID: String
    let displayName: String
    let byteSize: UInt64
    let recognizedStudioVision: Bool
    let profileLabel: String?
    let sequenceCount: UInt32
    let warningCount: UInt32
    let diagnosticsAvailable: Bool
    let sequences: [SequenceViewData]

    enum CodingKeys: String, CodingKey {
        case sessionID = "session_id"
        case project
        case sequences
        case diagnosticsAvailable = "diagnostics_available"
    }

    init(from decoder: Decoder) throws {
        let values = try decoder.container(keyedBy: CodingKeys.self)
        let project = try values.decode(Project.self, forKey: .project)
        sessionID = try values.decode(String.self, forKey: .sessionID)
        displayName = project.displayName
        byteSize = project.byteSize
        recognizedStudioVision = project.recognizedStudioVision
        profileLabel = project.profileLabel
        sequenceCount = project.sequenceCount
        warningCount = project.warningCount
        diagnosticsAvailable = try values.decode(Bool.self, forKey: .diagnosticsAvailable)
        sequences = try values.decode([SequenceViewData].self, forKey: .sequences)
    }

    private struct Project: Decodable {
        let displayName: String
        let byteSize: UInt64
        let recognizedStudioVision: Bool
        let profileLabel: String?
        let sequenceCount: UInt32
        let warningCount: UInt32

        enum CodingKeys: String, CodingKey {
            case displayName = "display_name"
            case byteSize = "byte_size"
            case recognizedStudioVision = "recognized_studio_vision"
            case profileLabel = "profile_label"
            case sequenceCount = "sequence_count"
            case warningCount = "warning_count"
        }
    }
}
