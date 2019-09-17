/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

class PingUploadOperation: GleanOperation {
    var dataTask: URLSessionDataTask?
    let request: URLRequest
    let callback: (Bool, Error?) -> Void

    init(request: URLRequest, callback: @escaping (Bool, Error?) -> Void) {
        self.request = request
        self.callback = callback
    }

    public override func cancel() {
        dataTask?.cancel()
        super.cancel()
    }

    override func start() {
        if self.isCancelled {
            finish(true)
            return
        }

        // Create the data task with appropriate status code handling
        dataTask = URLSession.shared.dataTask(with: request) { _, response, error in
            let httpResponse = response as? HTTPURLResponse
            let statusCode = httpResponse?.statusCode ?? 0
            switch statusCode {
            case 200 ..< 300:
                // Known success errors (2xx):
                // 200 - OK. Request accepted into the pipeline.

                // We treat all success codes as successful upload even though we only expect 200.
                self.callback(true, nil)
            case 400 ..< 500:
                // Known client (4xx) errors:
                // 404 - not found - POST/PUT to an unknown namespace
                // 405 - wrong request type (anything other than POST/PUT)
                // 411 - missing content-length header
                // 413 - request body too large (Note that if we have badly-behaved clients that
                //       retry on 4XX, we should send back 202 on body/path too long).
                // 414 - request path too long (See above)

                // Something our client did is not correct. It's unlikely that the client is going
                // to recover from this by re-trying again, so we just log an error and report a
                // successful upload to the service.
                self.callback(true, error)
            default:
                // Known other errors:
                // 500 - internal error

                // For all other errors we log a warning and try again at a later time.
                self.callback(false, error)
            }

            self.executing(false)
            self.finish(true)
        }

        executing(true)
        main()
    }

    override func main() {
        dataTask?.resume()
    }
}
