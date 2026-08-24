import Foundation

enum Readiness: String, Decodable, Equatable {
    case ready
    case partiallySupported = "partially_supported"
    case unsupported
    case unknown

    var displayName: String {
        switch self {
        case .ready: return "Ready"
        case .partiallySupported: return "Partially supported"
        case .unsupported: return "Not currently exportable"
        case .unknown: return "Readiness unknown"
        }
    }
}

enum ReasonSeverity: String, Decodable, Equatable {
    case informational
    case caution
    case dataLossRisk = "data_loss_risk"
}

struct ReadinessReason: Decodable, Equatable {
    let severity: ReasonSeverity
    let displayDetail: String

    enum CodingKeys: String, CodingKey {
        case severity
        case displayDetail = "display_detail"
    }
}

enum WarningSeverity: String, Decodable, Equatable {
    case informational
    case caution
    case dataLossRisk = "data_loss_risk"

    var displayName: String {
        switch self {
        case .informational: return "Information"
        case .caution: return "Caution"
        case .dataLossRisk: return "Data loss risk"
        }
    }
}

enum WarningScope: String, Decodable, Equatable {
    case project
    case sequence
    case genericTrack = "generic_track"
}

struct ProjectWarning: Decodable, Equatable {
    let message: String
    let scope: WarningScope
    let severity: WarningSeverity
}

struct ProfileCapability: Decodable, Equatable {
    let displayLabel: String

    enum CodingKeys: String, CodingKey {
        case displayLabel = "display_label"
    }
}

struct SequenceViewData: Decodable, Equatable {
    let sequenceID: String
    let displayName: String
    let readiness: Readiness
    let readinessReason: ReadinessReason
    let musicalTrackCount: UInt32?
    let warningCount: UInt32
    let exportCapability: ProfileCapability?
    let diagnosticsAvailable: Bool

    enum CodingKeys: String, CodingKey {
        case sequenceID = "sequence_id"
        case displayName = "display_name"
        case readiness
        case readinessReason = "readiness_reason"
        case musicalTrackCount = "musical_track_count"
        case warningCount = "warning_count"
        case exportCapability = "export_capability"
        case diagnosticsAvailable = "diagnostics_available"
    }
}

struct ProjectInspection: Decodable, Equatable {
    let sessionID: String
    let displayName: String
    let byteSize: UInt64
    let recognizedStudioVision: Bool
    let profileLabel: String?
    let sequenceCount: UInt32
    let overallReadiness: Readiness
    let warningCount: UInt32
    let diagnosticsAvailable: Bool
    let sequences: [SequenceViewData]
    let warnings: [ProjectWarning]

    enum CodingKeys: String, CodingKey {
        case sessionID = "session_id"
        case project
        case sequences
        case warnings
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
        overallReadiness = project.overallReadiness
        warningCount = project.warningCount
        diagnosticsAvailable = try values.decode(Bool.self, forKey: .diagnosticsAvailable)
        sequences = try values.decode([SequenceViewData].self, forKey: .sequences)
        warnings = try values.decode([ProjectWarning].self, forKey: .warnings)
    }

    private struct Project: Decodable {
        let displayName: String
        let byteSize: UInt64
        let recognizedStudioVision: Bool
        let profileLabel: String?
        let sequenceCount: UInt32
        let overallReadiness: Readiness
        let warningCount: UInt32

        enum CodingKeys: String, CodingKey {
            case displayName = "display_name"
            case byteSize = "byte_size"
            case recognizedStudioVision = "recognized_studio_vision"
            case profileLabel = "profile_label"
            case sequenceCount = "sequence_count"
            case overallReadiness = "overall_readiness"
            case warningCount = "warning_count"
        }
    }
}

struct DiagnosticsSummary: Decodable, Equatable {
    let coreVersion: String
    let recognizedProfile: String?
    let structuralStatus: String?
    let unsupportedFamilies: [String]
    let compatibilityProfile: ProfileCapability?

    enum CodingKeys: String, CodingKey {
        case coreVersion = "core_version"
        case recognizedProfile = "recognized_profile"
        case structuralStatus = "structural_status"
        case unsupportedFamilies = "unsupported_families"
        case compatibilityProfile = "compatibility_profile"
    }
}
