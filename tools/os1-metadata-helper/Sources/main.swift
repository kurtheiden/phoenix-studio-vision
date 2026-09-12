import CryptoKit
import Darwin
import Foundation

private let schemaVersion = "OS1_METADATA_HELPER_MAP_V1"
private let recordSchemaVersion = "OS1_METADATA_HELPER_RECORD_V1"
private let errorEnumVersion = "OS1_METADATA_HELPER_ERRORS_V1"
private let helperVersion = "1.0.0-review"
private let maximumEntries = 999_999
private let readBufferSize = 65_536
private let coverage = "COMPLETE_REGULAR_FILE_DATA_FORK_ONLY; XATTRS_AND_RESOURCE_FORK_EXCLUDED"

private enum ExitCode: Int32 {
    case invalidArguments = 64
    case failure = 70
}

private enum HelperError: String, Error {
    case invalidArguments = "INVALID_ARGUMENTS"
    case nonabsolutePath = "NONABSOLUTE_PATH"
    case outputExists = "OUTPUT_EXISTS"
    case outputOutsidePrivateRootRequired = "OUTPUT_OUTSIDE_PRIVATE_ROOT_REQUIRED"
    case rootNotDirectory = "ROOT_NOT_DIRECTORY"
    case rootSymlink = "ROOT_SYMLINK"
    case enumerationFailed = "ENUMERATION_FAILED"
    case entryLimitExceeded = "ENTRY_LIMIT_EXCEEDED"
    case mapSchemaInvalid = "MAP_SCHEMA_INVALID"
    case mapIntegrityFailed = "MAP_INTEGRITY_FAILED"
    case mapRootIdentityChanged = "MAP_ROOT_IDENTITY_CHANGED"
    case openFailed = "OPEN_FAILED"
    case notRegularAfterOpen = "NOT_REGULAR_AFTER_OPEN"
    case objectChangedOrReplaced = "OBJECT_CHANGED_OR_REPLACED"
    case readFailed = "READ_FAILED"
    case seekFailed = "SEEK_FAILED"
    case finderInfoMalformed = "FINDERINFO_MALFORMED"
    case finderInfoError = "FINDERINFO_ERROR"
    case resourceForkError = "RESOURCE_FORK_ERROR"
    case hashLengthMismatch = "HASH_LENGTH_MISMATCH"
    case hashPassesDiffer = "HASH_PASSES_DIFFER"
    case integrityIncident = "INTEGRITY_INCIDENT"
    case privateOutputCreateFailed = "PRIVATE_OUTPUT_CREATE_FAILED"
    case privateOutputWriteFailed = "PRIVATE_OUTPUT_WRITE_FAILED"
    case privateOutputSyncFailed = "PRIVATE_OUTPUT_SYNC_FAILED"
    case privateOutputPublishFailed = "PRIVATE_OUTPUT_PUBLISH_FAILED"
}

private struct FileTime: Equatable {
    let seconds: Int64
    let nanoseconds: UInt32

    var json: [String: Any] { ["seconds": seconds, "nanoseconds": nanoseconds] }
}

private struct Snapshot: Equatable {
    let size: UInt64
    let birth: FileTime
    let modification: FileTime
    let change: FileTime
    let access: FileTime
    let device: UInt64
    let inode: UInt64
    let mode: mode_t

    init(_ value: stat) {
        size = UInt64(max(0, value.st_size))
        birth = FileTime(seconds: Int64(value.st_birthtimespec.tv_sec), nanoseconds: UInt32(value.st_birthtimespec.tv_nsec))
        modification = FileTime(seconds: Int64(value.st_mtimespec.tv_sec), nanoseconds: UInt32(value.st_mtimespec.tv_nsec))
        change = FileTime(seconds: Int64(value.st_ctimespec.tv_sec), nanoseconds: UInt32(value.st_ctimespec.tv_nsec))
        access = FileTime(seconds: Int64(value.st_atimespec.tv_sec), nanoseconds: UInt32(value.st_atimespec.tv_nsec))
        device = UInt64(value.st_dev)
        inode = UInt64(value.st_ino)
        mode = value.st_mode
    }

    var timesJSON: [String: Any] {
        ["birth": birth.json, "modification": modification.json, "metadata_change": change.json, "access": access.json]
    }

    func protectedMatches(_ other: Snapshot) -> Bool {
        size == other.size && birth == other.birth && modification == other.modification &&
            change == other.change && device == other.device && inode == other.inode &&
            (mode & S_IFMT) == (other.mode & S_IFMT)
    }
}

private enum FinderInfo: Equatable {
    case absent
    case present(type: [UInt8], creator: [UInt8])
    case failure(HelperError)

    var json: [String: Any] {
        switch self {
        case .absent: return ["state": "ABSENT"]
        case let .present(type, creator):
            return ["state": "PRESENT", "type_hex": hex(type), "creator_hex": hex(creator)]
        case .failure: return ["state": "ERROR"]
        }
    }

    var isAlias: Bool {
        if case let .present(type, _) = self { return type == Array("alis".utf8) }
        return false
    }
}

private enum Presence: String { case present = "PRESENT", absent = "ABSENT", error = "ERROR" }

private func hex<S: Sequence>(_ bytes: S) -> String where S.Element == UInt8 {
    bytes.map { String(format: "%02x", $0) }.joined()
}

private func digest(_ data: Data) -> String { hex(SHA256.hash(data: data)) }

private func jsonData(_ object: Any) throws -> Data {
    guard JSONSerialization.isValidJSONObject(object) else { throw HelperError.privateOutputWriteFailed }
    return try JSONSerialization.data(withJSONObject: object, options: [.sortedKeys, .withoutEscapingSlashes])
}

private func isAbsolute(_ path: String) -> Bool { path.utf8.first == 0x2f && !path.utf8.contains(0) }

private func pathIsWithin(_ path: String, _ root: String) -> Bool {
    path == root || path.hasPrefix(root.hasSuffix("/") ? root : root + "/")
}

private func lstatSnapshot(_ path: String) throws -> Snapshot {
    var info = stat()
    guard lstat(path, &info) == 0 else { throw HelperError.openFailed }
    return Snapshot(info)
}

private func validatePrivateOutput(_ output: String, outside root: String?) throws {
    guard isAbsolute(output) else { throw HelperError.nonabsolutePath }
    if let root, pathIsWithin(output, root) { throw HelperError.outputOutsidePrivateRootRequired }
    var existing = stat()
    if lstat(output, &existing) == 0 { throw HelperError.outputExists }
    if errno != ENOENT { throw HelperError.privateOutputCreateFailed }
    let parent = (output as NSString).deletingLastPathComponent
    var parentInfo = stat()
    guard lstat(parent, &parentInfo) == 0, (parentInfo.st_mode & S_IFMT) == S_IFDIR else {
        throw HelperError.outputOutsidePrivateRootRequired
    }
    guard (parentInfo.st_mode & S_IFMT) != S_IFLNK, (parentInfo.st_mode & 0o077) == 0 else {
        throw HelperError.outputOutsidePrivateRootRequired
    }
    var ancestor = parent
    while ancestor != "/" {
        var marker = stat()
        if lstat(ancestor + "/.git", &marker) == 0 { throw HelperError.outputOutsidePrivateRootRequired }
        ancestor = (ancestor as NSString).deletingLastPathComponent
    }
}

private func publish(_ data: Data, to output: String) throws {
    let parent = (output as NSString).deletingLastPathComponent
    let temporary = parent + "/.os1-metadata-" + UUID().uuidString.lowercased()
    let fd = open(temporary, O_WRONLY | O_CREAT | O_EXCL | O_CLOEXEC, 0o600)
    guard fd >= 0 else { throw HelperError.privateOutputCreateFailed }
    var succeeded = false
    defer {
        Darwin.close(fd)
        if !succeeded { unlink(temporary) }
    }
    let writeResult = data.withUnsafeBytes { raw -> Bool in
        guard let base = raw.baseAddress else { return data.isEmpty }
        var offset = 0
        while offset < raw.count {
            let count = Darwin.write(fd, base.advanced(by: offset), raw.count - offset)
            if count <= 0 { return false }
            offset += count
        }
        return true
    }
    guard writeResult else { throw HelperError.privateOutputWriteFailed }
    guard fsync(fd) == 0 else { throw HelperError.privateOutputSyncFailed }
    guard renameatx_np(AT_FDCWD, temporary, AT_FDCWD, output, UInt32(RENAME_EXCL)) == 0 else {
        if errno == EEXIST { throw HelperError.outputExists }
        throw HelperError.privateOutputPublishFailed
    }
    succeeded = true
}

private func rawName(_ entry: UnsafeMutablePointer<dirent>) -> [UInt8] {
    withUnsafePointer(to: &entry.pointee.d_name) { pointer in
        pointer.withMemoryRebound(to: UInt8.self, capacity: Int(MAXNAMLEN) + 1) { bytes in
            var result: [UInt8] = []
            var index = 0
            while index <= Int(MAXNAMLEN), bytes[index] != 0 { result.append(bytes[index]); index += 1 }
            return result
        }
    }
}

private func withCStringBytes<T>(_ bytes: [UInt8], _ body: (UnsafePointer<CChar>) throws -> T) rethrows -> T {
    var terminated = bytes.map { CChar(bitPattern: $0) }
    terminated.append(0)
    return try terminated.withUnsafeBufferPointer { try body($0.baseAddress!) }
}

private func kind(_ mode: mode_t) -> String {
    switch mode & S_IFMT {
    case S_IFREG: return "REGULAR_FILE"
    case S_IFDIR: return "DIRECTORY"
    case S_IFLNK: return "SYMLINK"
    case S_IFIFO: return "FIFO"
    case S_IFSOCK: return "SOCKET"
    case S_IFCHR: return "CHARACTER_DEVICE"
    case S_IFBLK: return "BLOCK_DEVICE"
    default: return "OTHER"
    }
}

private func descriptorXattrLength(_ fd: Int32, _ name: StaticString) -> ssize_t {
    name.withUTF8Buffer { buffer in
        fgetxattr(fd, UnsafePointer<CChar>(OpaquePointer(buffer.baseAddress!)), nil, 0, 0, 0)
    }
}

private func finderInfo(_ fd: Int32) -> FinderInfo {
    let attribute: StaticString = "com.apple.FinderInfo"
    let length = descriptorXattrLength(fd, attribute)
    if length < 0 { return errno == ENOATTR ? .absent : .failure(.finderInfoError) }
    guard length >= 8, length <= 4096 else { return .failure(.finderInfoMalformed) }
    var bytes = [UInt8](repeating: 0, count: Int(length))
    let readCount: ssize_t = attribute.withUTF8Buffer { name in
        bytes.withUnsafeMutableBytes { value in
            fgetxattr(fd, UnsafePointer<CChar>(OpaquePointer(name.baseAddress!)), value.baseAddress, value.count, 0, 0)
        }
    }
    guard readCount == length else { _ = bytes.withUnsafeMutableBytes { $0.initializeMemory(as: UInt8.self, repeating: 0) }; return .failure(.finderInfoError) }
    let type = Array(bytes[0..<4])
    let creator = Array(bytes[4..<8])
    _ = bytes.withUnsafeMutableBytes { $0.initializeMemory(as: UInt8.self, repeating: 0) }
    return .present(type: type, creator: creator)
}

private func resourceFork(_ fd: Int32) -> Presence {
    let length = descriptorXattrLength(fd, "com.apple.ResourceFork")
    if length >= 0 { return .present }
    return errno == ENOATTR ? .absent : .error
}

private func descriptorSnapshot(_ fd: Int32) throws -> Snapshot {
    var value = stat()
    guard fstat(fd, &value) == 0 else { throw HelperError.integrityIncident }
    return Snapshot(value)
}

private func sha256Pass(_ fd: Int32, expected: UInt64) throws -> String {
    guard lseek(fd, 0, SEEK_SET) == 0 else { throw HelperError.seekFailed }
    var hasher = SHA256()
    var buffer = [UInt8](repeating: 0, count: readBufferSize)
    var total: UInt64 = 0
    while true {
        let count = buffer.withUnsafeMutableBytes { Darwin.read(fd, $0.baseAddress, $0.count) }
        if count < 0 { _ = buffer.withUnsafeMutableBytes { $0.initializeMemory(as: UInt8.self, repeating: 0) }; throw HelperError.readFailed }
        if count == 0 { break }
        total += UInt64(count)
        buffer.withUnsafeBytes {
            hasher.update(bufferPointer: UnsafeRawBufferPointer(start: $0.baseAddress, count: count))
        }
    }
    _ = buffer.withUnsafeMutableBytes { $0.initializeMemory(as: UInt8.self, repeating: 0) }
    guard total == expected else { throw HelperError.hashLengthMismatch }
    return hex(hasher.finalize())
}

private func executableSHA256() throws -> String {
    var size: UInt32 = 0
    _NSGetExecutablePath(nil, &size)
    var bytes = [CChar](repeating: 0, count: Int(size))
    guard _NSGetExecutablePath(&bytes, &size) == 0 else { throw HelperError.integrityIncident }
    let fd = open(bytes, O_RDONLY | O_CLOEXEC)
    guard fd >= 0 else { throw HelperError.integrityIncident }
    defer { Darwin.close(fd) }
    let snapshot = try descriptorSnapshot(fd)
    return try sha256Pass(fd, expected: snapshot.size)
}

private func mapDigest(_ mapWithoutDigest: [String: Any]) throws -> String { digest(try jsonData(mapWithoutDigest)) }

private func parseArguments() throws -> (mode: String, first: String, second: String) {
    let arguments = Array(CommandLine.arguments.dropFirst())
    guard arguments.count == 5 else { throw HelperError.invalidArguments }
    let mode = arguments[0]
    let expected = mode == "enumerate" ? ("--root", "--map-output") : mode == "inspect" ? ("--map", "--record-output") : ("", "")
    guard !expected.0.isEmpty else { throw HelperError.invalidArguments }
    var values: [String: String] = [:]
    var index = 1
    while index < arguments.count {
        let key = arguments[index]
        guard key == expected.0 || key == expected.1, values[key] == nil else { throw HelperError.invalidArguments }
        values[key] = arguments[index + 1]
        index += 2
    }
    guard let first = values[expected.0], let second = values[expected.1], isAbsolute(first), isAbsolute(second) else {
        throw HelperError.nonabsolutePath
    }
    return (mode, first, second)
}

private func enumerate(root: String, output: String) throws {
    try validatePrivateOutput(output, outside: root)
    let initial = try lstatSnapshot(root)
    guard (initial.mode & S_IFMT) != S_IFLNK else { throw HelperError.rootSymlink }
    guard (initial.mode & S_IFMT) == S_IFDIR else { throw HelperError.rootNotDirectory }
    guard let directory = opendir(root) else { throw HelperError.enumerationFailed }
    defer { closedir(directory) }
    let directoryFD = dirfd(directory)
    let opened = try descriptorSnapshot(directoryFD)
    guard opened.device == initial.device, opened.inode == initial.inode, (opened.mode & S_IFMT) == S_IFDIR else { throw HelperError.rootSymlink }
    var names: [[UInt8]] = []
    errno = 0
    while let entry = readdir(directory) {
        let name = rawName(entry)
        if name == [46] || name == [46, 46] { continue }
        names.append(name)
        if names.count > maximumEntries { throw HelperError.entryLimitExceeded }
    }
    guard errno == 0 else { throw HelperError.enumerationFailed }
    names.sort { $0.lexicographicallyPrecedes($1) }
    var entries: [[String: Any]] = []
    for (offset, name) in names.enumerated() {
        var value = stat()
        let result = withCStringBytes(name) { fstatat(directoryFD, $0, &value, AT_SYMLINK_NOFOLLOW) }
        guard result == 0 else { throw HelperError.enumerationFailed }
        var item: [String: Any] = [
            "artifact_id": String(format: "R-%06d", offset + 1),
            "basename_encoding": Data(name).base64EncodedString().replacingOccurrences(of: "+", with: "-").replacingOccurrences(of: "/", with: "_").replacingOccurrences(of: "=", with: ""),
            "kind": kind(value.st_mode)
        ]
        if let utf8 = String(bytes: name, encoding: .utf8) { item["basename_utf8"] = utf8 }
        entries.append(item)
    }
    var map: [String: Any] = [
        "schema_version": schemaVersion,
        "error_enum_version": errorEnumVersion,
        "helper_version": helperVersion,
        "helper_source_sha256": BuildIdentity.sourceSHA256,
        "helper_binary_sha256": try executableSHA256(),
        "intake_nonce": UUID().uuidString.lowercased(),
        "root_path": root,
        "root_device": String(initial.device),
        "root_inode": String(initial.inode),
        "entry_count": entries.count,
        "entries": entries
    ]
    map["canonical_map_sha256"] = try mapDigest(map)
    try publish(try jsonData(map), to: output)
}

private func decodedName(_ encoded: String) throws -> [UInt8] {
    var value = encoded.replacingOccurrences(of: "-", with: "+").replacingOccurrences(of: "_", with: "/")
    value += String(repeating: "=", count: (4 - value.count % 4) % 4)
    guard let data = Data(base64Encoded: value), !data.isEmpty, !data.contains(0), data != Data([46]), data != Data([46, 46]), !data.contains(47) else {
        throw HelperError.mapSchemaInvalid
    }
    return Array(data)
}

private func rejectedRecord(id: String, kind value: String, binaryHash: String) -> [String: Any] {
    let status: String
    let code: String
    switch value {
    case "SYMLINK": status = "REJECTED_NO_FOLLOW"; code = "REJECTED_NO_FOLLOW"
    case "DIRECTORY": status = "REJECTED_DIRECTORY"; code = "REJECTED_DIRECTORY"
    default: status = "REJECTED_UNSUPPORTED_KIND"; code = "REJECTED_UNSUPPORTED_KIND"
    }
    return ["schema_version": recordSchemaVersion, "helper_source_sha256": BuildIdentity.sourceSHA256, "helper_binary_sha256": binaryHash,
            "artifact_id": id, "status": status, "error_code": code, "kind": value,
            "integrity_status": "NOT_INSPECTED", "coverage_statement": coverage]
}

private func failureRecord(id: String, code: HelperError, binaryHash: String) -> [String: Any] {
    ["schema_version": recordSchemaVersion, "helper_source_sha256": BuildIdentity.sourceSHA256,
     "helper_binary_sha256": binaryHash, "artifact_id": id, "status": "ERROR", "error_code": code.rawValue,
     "kind": "REGULAR_FILE", "integrity_status": "FAILED", "coverage_statement": coverage]
}

private func inspectRegular(directoryFD: Int32, name: [UInt8], id: String, binaryHash: String) throws -> [String: Any] {
    var beforeOpen = stat()
    guard withCStringBytes(name, { fstatat(directoryFD, $0, &beforeOpen, AT_SYMLINK_NOFOLLOW) }) == 0 else { throw HelperError.objectChangedOrReplaced }
    let preOpen = Snapshot(beforeOpen)
    guard (preOpen.mode & S_IFMT) == S_IFREG else { throw HelperError.objectChangedOrReplaced }
    let fd = withCStringBytes(name) { openat(directoryFD, $0, O_RDONLY | O_NOFOLLOW | O_CLOEXEC) }
    guard fd >= 0 else { throw HelperError.openFailed }
    defer { Darwin.close(fd) }
    let pre = try descriptorSnapshot(fd)
    guard (pre.mode & S_IFMT) == S_IFREG else { throw HelperError.notRegularAfterOpen }
    guard pre.device == preOpen.device, pre.inode == preOpen.inode else { throw HelperError.objectChangedOrReplaced }
    let finderPre = finderInfo(fd)
    let resourcePre = resourceFork(fd)
    if case let .failure(error) = finderPre { throw error }
    if resourcePre == .error { throw HelperError.resourceForkError }
    if finderPre.isAlias {
        var aliasPath = stat()
        guard withCStringBytes(name, { fstatat(directoryFD, $0, &aliasPath, AT_SYMLINK_NOFOLLOW) }) == 0 else {
            throw HelperError.objectChangedOrReplaced
        }
        let aliasFinal = Snapshot(aliasPath)
        guard aliasFinal.device == pre.device, aliasFinal.inode == pre.inode, (aliasFinal.mode & S_IFMT) == S_IFREG else {
            throw HelperError.objectChangedOrReplaced
        }
        return ["schema_version": recordSchemaVersion, "helper_source_sha256": BuildIdentity.sourceSHA256,
                "helper_binary_sha256": binaryHash, "artifact_id": id, "status": "REJECTED_ALIAS_TYPE",
                "error_code": "REJECTED_ALIAS_TYPE", "kind": "REGULAR_FILE", "integrity_status": "NOT_INSPECTED",
                "coverage_statement": coverage]
    }
    let first = try sha256Pass(fd, expected: pre.size)
    let intermediate = try descriptorSnapshot(fd)
    guard pre.protectedMatches(intermediate) else { throw HelperError.integrityIncident }
    let second = try sha256Pass(fd, expected: pre.size)
    guard first == second else { throw HelperError.hashPassesDiffer }
    let post = try descriptorSnapshot(fd)
    let finderPost = finderInfo(fd)
    let resourcePost = resourceFork(fd)
    if case let .failure(error) = finderPost { throw error }
    if resourcePost == .error { throw HelperError.resourceForkError }
    guard pre.protectedMatches(post), finderPre == finderPost, resourcePre == resourcePost else { throw HelperError.integrityIncident }
    var finalPath = stat()
    guard withCStringBytes(name, { fstatat(directoryFD, $0, &finalPath, AT_SYMLINK_NOFOLLOW) }) == 0 else { throw HelperError.objectChangedOrReplaced }
    let final = Snapshot(finalPath)
    guard final.device == pre.device, final.inode == pre.inode, (final.mode & S_IFMT) == S_IFREG else { throw HelperError.objectChangedOrReplaced }
    let atime = pre.access == post.access ? "NO" : "YES"
    return [
        "schema_version": recordSchemaVersion, "helper_source_sha256": BuildIdentity.sourceSHA256,
        "helper_binary_sha256": binaryHash, "artifact_id": id, "status": "INSPECTED", "kind": "REGULAR_FILE",
        "eligibility_hint": pre.size == 0 ? "EMPTY_REGULAR_FILE" : "NONE", "size_bytes": pre.size,
        "pre_times": pre.timesJSON, "post_times": post.timesJSON, "atime_changed": atime,
        "finder_info": finderPre.json,
        "resource_fork": ["pre": resourcePre.rawValue, "post": resourcePost.rawValue, "unchanged": resourcePre == resourcePost],
        "sha256": first, "integrity_status": "VERIFIED", "coverage_statement": coverage
    ]
}

private func inspect(mapPath: String, output: String) throws {
    try validatePrivateOutput(output, outside: nil)
    let mapInfo = try lstatSnapshot(mapPath)
    guard (mapInfo.mode & S_IFMT) == S_IFREG else { throw HelperError.mapSchemaInvalid }
    let mapFD = open(mapPath, O_RDONLY | O_NOFOLLOW | O_CLOEXEC)
    guard mapFD >= 0 else { throw HelperError.mapSchemaInvalid }
    defer { Darwin.close(mapFD) }
    let openedMap = try descriptorSnapshot(mapFD)
    guard openedMap.device == mapInfo.device, openedMap.inode == mapInfo.inode, (openedMap.mode & S_IFMT) == S_IFREG else {
        throw HelperError.mapSchemaInvalid
    }
    let mapData = try readAllBounded(mapFD, maximum: 64 * 1024 * 1024)
    guard let parsed = try JSONSerialization.jsonObject(with: mapData) as? [String: Any],
          parsed["schema_version"] as? String == schemaVersion,
          parsed["helper_source_sha256"] as? String == BuildIdentity.sourceSHA256,
          let storedDigest = parsed["canonical_map_sha256"] as? String,
          let root = parsed["root_path"] as? String,
          let rootDevice = parsed["root_device"] as? String,
          let rootInode = parsed["root_inode"] as? String,
          let entries = parsed["entries"] as? [[String: Any]],
          parsed["entry_count"] as? Int == entries.count else { throw HelperError.mapSchemaInvalid }
    if pathIsWithin(output, root) { throw HelperError.outputOutsidePrivateRootRequired }
    var withoutDigest = parsed
    withoutDigest.removeValue(forKey: "canonical_map_sha256")
    guard try mapDigest(withoutDigest) == storedDigest else { throw HelperError.mapIntegrityFailed }
    let rootSnapshot = try lstatSnapshot(root)
    guard (rootSnapshot.mode & S_IFMT) == S_IFDIR, String(rootSnapshot.device) == rootDevice, String(rootSnapshot.inode) == rootInode else {
        throw HelperError.mapRootIdentityChanged
    }
    guard let directory = opendir(root) else { throw HelperError.mapRootIdentityChanged }
    defer { closedir(directory) }
    let directoryFD = dirfd(directory)
    let openedRoot = try descriptorSnapshot(directoryFD)
    guard openedRoot.device == rootSnapshot.device, openedRoot.inode == rootSnapshot.inode else { throw HelperError.mapRootIdentityChanged }
    let binaryHash = try executableSHA256()
    var lines = Data()
    var terminalArtifactError: HelperError?
    for (offset, entry) in entries.enumerated() {
        guard let id = entry["artifact_id"] as? String, id == String(format: "R-%06d", offset + 1),
              let encoded = entry["basename_encoding"] as? String, let mappedKind = entry["kind"] as? String else {
            throw HelperError.mapSchemaInvalid
        }
        let name = try decodedName(encoded)
        var current = stat()
        guard withCStringBytes(name, { fstatat(directoryFD, $0, &current, AT_SYMLINK_NOFOLLOW) }) == 0,
              kind(current.st_mode) == mappedKind else { throw HelperError.objectChangedOrReplaced }
        let record: [String: Any]
        if mappedKind == "REGULAR_FILE" {
            do {
                record = try inspectRegular(directoryFD: directoryFD, name: name, id: id, binaryHash: binaryHash)
            } catch let error as HelperError {
                record = failureRecord(id: id, code: error, binaryHash: binaryHash)
                terminalArtifactError = error
            }
        } else {
            record = rejectedRecord(id: id, kind: mappedKind, binaryHash: binaryHash)
        }
        lines.append(try jsonData(record)); lines.append(0x0a)
        if terminalArtifactError != nil { break }
    }
    try publish(lines, to: output)
    if let terminalArtifactError { throw terminalArtifactError }
}

private func readAllBounded(_ fd: Int32, maximum: Int) throws -> Data {
    var result = Data()
    var buffer = [UInt8](repeating: 0, count: readBufferSize)
    while true {
        let count = buffer.withUnsafeMutableBytes { Darwin.read(fd, $0.baseAddress, $0.count) }
        if count < 0 { throw HelperError.readFailed }
        if count == 0 { break }
        guard result.count <= maximum - count else { throw HelperError.mapSchemaInvalid }
        result.append(buffer, count: count)
    }
    _ = buffer.withUnsafeMutableBytes { $0.initializeMemory(as: UInt8.self, repeating: 0) }
    return result
}

private func run() throws {
    umask(0o077)
    let arguments = try parseArguments()
    if arguments.mode == "enumerate" { try enumerate(root: arguments.first, output: arguments.second) }
    else { try inspect(mapPath: arguments.first, output: arguments.second) }
}

do {
    try run()
    exit(0)
} catch let error as HelperError {
    let message = "OS1_METADATA_HELPER_ERROR:\(error.rawValue)\n"
    _ = message.withCString { Darwin.write(STDERR_FILENO, $0, strlen($0)) }
    exit(error == .invalidArguments || error == .nonabsolutePath ? ExitCode.invalidArguments.rawValue : ExitCode.failure.rawValue)
} catch {
    let message = "OS1_METADATA_HELPER_ERROR:INTEGRITY_INCIDENT\n"
    _ = message.withCString { Darwin.write(STDERR_FILENO, $0, strlen($0)) }
    exit(ExitCode.failure.rawValue)
}
