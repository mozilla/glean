/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation
import Swifter

// Create a new Glean endpoint HTTP server on localhost and react only for the specified ping type
func mockServer(expectPingType: String, callback: @escaping ([String: Any]?) -> Void) -> HttpServer {
    let server = HttpServer()

    server["/submit/:appid/:ping/:schema/:pinguuid"] = { request in
        let pingName = request.params[":ping"]!
        if pingName == expectPingType {
            let body = String(bytes: request.body, encoding: .utf8)!
            let data = body.data(using: .utf8)!
            print("Received data: \(body)")
            let json = try! JSONSerialization.jsonObject(with: data, options: []) as? [String: Any]
            callback(json)
        }
        return HttpResponse.ok(.text("OK"))
    }
    // For logging purposes:
    server.middleware.append { request in
        print("Middleware: \(request.address ?? "unknown address") -> \(request.method) -> \(request.path)")
        return nil
    }

    try! server.start(9080)
    return server
}
