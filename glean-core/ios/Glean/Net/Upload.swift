/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

struct PingRequest {
    let uuid: String
    let path: String
    let body: String
    let headers: [String: String]

    init(uuid: String, path: String, body: String, headers: [String: String]) {
        self.uuid = uuid
        self.path = path
        self.body = body

        var headers = headers
        if let pingTag = Glean.shared.configuration?.pingTag {
            headers["X-Debug-ID"] = pingTag
        }

        self.headers = headers
    }
}

enum PingUploadTask {
    case upload(PingRequest)
    case wait
    case done
}

extension FfiPingUploadTask {
    func toPingUploadTask() -> PingUploadTask {
        switch UInt32(self.tag) {
        case FfiPingUploadTask_Upload.rawValue:
            return .upload(self.upload.toPingRequest())
        case FfiPingUploadTask_Wait.rawValue:
            return .wait
        case FfiPingUploadTask_Done.rawValue:
            return .done
        default:
            // Tag can only be one of the enum values,
            // therefore we can't reach this point
            assertUnreachable()
        }
    }
}

extension FfiPingUploadTask_Upload_Body {
    func toPingRequest() -> PingRequest {
        let uuid = String(cString: self.uuid)
        let path = String(cString: self.path)
        let body = String(cString: self.body)

        // Decode the header object from JSON
        let json = String(cString: self.headers)
        let data = json.data(using: .utf8)!
        let headers = try? JSONSerialization.jsonObject(with: data, options: []) as? [String: String]

        return PingRequest(uuid: uuid, path: path, body: body, headers: headers ?? [String: String]())
    }
}
