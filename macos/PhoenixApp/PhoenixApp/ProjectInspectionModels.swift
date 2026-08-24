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

    var isExportEligible: Bool {
        readiness == .ready && exportCapability != nil
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

enum CollisionPolicy: String, Codable, Equatable {
    case failIfExists = "fail_if_exists"
    case generateUniqueName = "generate_unique_name"
}

enum ValidationStatus: String, Decodable, Equatable {
    case validated
}

struct ExportCounts: Decodable, Equatable {
    let notes: UInt64
    let generatedNoteOffs: UInt64
    let controllers: UInt64
    let bankSelectMSB: UInt64
    let bankSelectLSB: UInt64
    let programs: UInt64
    let pressure: UInt64
    let pitchBend: UInt64
    let tempo: UInt64
    let meter: UInt64

    enum CodingKeys: String, CodingKey {
        case notes
        case generatedNoteOffs = "generated_note_offs"
        case controllers
        case bankSelectMSB = "bank_select_msb"
        case bankSelectLSB = "bank_select_lsb"
        case programs
        case pressure
        case pitchBend = "pitch_bend"
        case tempo
        case meter
    }
}

struct ExportSequenceResult: Decodable, Equatable {
    let sessionID: String
    let sequenceID: String
    let sequenceDisplayName: String
    let outputPath: String
    let compatibilityProfile: ProfileCapability?
    let musicalTrackCount: UInt32
    let totalSMFTrackCount: UInt32
    let counts: ExportCounts
    let warnings: [ProjectWarning]
    let untranslatedMetadataCount: UInt64
    let validationStatus: ValidationStatus

    enum CodingKeys: String, CodingKey {
        case sessionID = "session_id"
        case sequenceID = "sequence_id"
        case sequenceDisplayName = "sequence_display_name"
        case outputPath = "output_path"
        case compatibilityProfile = "compatibility_profile"
        case musicalTrackCount = "musical_track_count"
        case totalSMFTrackCount = "total_smf_track_count"
        case counts
        case warnings
        case untranslatedMetadataCount = "untranslated_metadata_count"
        case validationStatus = "validation_status"
    }
}

struct ExportSequenceRequest: Encodable, Equatable {
    let operation = "export_sequence"
    let contractVersion: UInt32 = 1
    let payload: Payload

    struct Payload: Encodable, Equatable {
        let sessionID: String
        let sequenceID: String
        let destinationFolder: String
        let filenameStem: String
        let collisionPolicy: CollisionPolicy
        let operationID: String?

        enum CodingKeys: String, CodingKey {
            case sessionID = "session_id"
            case sequenceID = "sequence_id"
            case destinationFolder = "destination_folder"
            case filenameStem = "filename_stem"
            case collisionPolicy = "collision_policy"
            case operationID = "operation_id"
        }

        func encode(to encoder: Encoder) throws {
            var values = encoder.container(keyedBy: CodingKeys.self)
            try values.encode(sessionID, forKey: .sessionID)
            try values.encode(sequenceID, forKey: .sequenceID)
            try values.encode(destinationFolder, forKey: .destinationFolder)
            try values.encode(filenameStem, forKey: .filenameStem)
            try values.encode(collisionPolicy, forKey: .collisionPolicy)
            if let operationID {
                try values.encode(operationID, forKey: .operationID)
            } else {
                try values.encodeNil(forKey: .operationID)
            }
        }
    }

    enum CodingKeys: String, CodingKey {
        case operation
        case contractVersion = "contract_version"
        case payload
    }
}
