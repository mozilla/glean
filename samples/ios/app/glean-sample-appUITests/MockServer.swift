/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation
import Gzip
import Swifter

// Create a new Glean endpoint HTTP server on localhost and react only for the specified ping type
func mockServer(expectPingType: String, port: UInt16 = 0, callback: @escaping ([String: Any]?) -> Void) -> HttpServer {
    let server = HttpServer()

    server["/submit/:appid/:ping/:schema/:pinguuid"] = { request in
        let pingName = request.params[":ping"]!
        if pingName == expectPingType {
            var data = Data(request.body)

            // Swifter lowercases all headers.
            if request.headers["content-encoding"] == "gzip" {
                data = try! data.gunzipped()
            }

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

    try! server.start(port)
    return server
}
