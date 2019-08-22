//
//  Glean.swift
//  Glean
//
//  Created by Jan-Erik Rediger on 27.03.19.
//  Copyright © 2019 Jan-Erik Rediger. All rights reserved.
//

import Foundation

public enum Lifetime: Int32 {
    case ping = 0
    case application = 1
    case user = 2
}

public func withArrayOfCStrings<R>(
    _ args: [String],
    _ body: ([UnsafePointer<CChar>?]) -> R
) -> R {
    var cStrings = args.map { strdup($0) }
    cStrings.append(nil)
    defer {
        cStrings.forEach { free($0) }
    }
    return body(cStrings)
}

public class CounterMetricType {
    var handle: UInt64

    public init(category: String, name: String, sendInPings: [String], lifetime: Lifetime, disabled: Bool) {
        self.handle = withArrayOfCStrings(sendInPings) {
            args in
            glean_new_counter_metric(category, name, args, Int32(sendInPings.count), lifetime.rawValue, disabled ? 1 : 0)
        }
    }

    public func add(amount: Int32 = 1) {
        glean_counter_add(Glean.shared.handle, self.handle, amount)
    }
}

public class Glean {
    public static let shared = Glean()

    public var handle: UInt64 = 0

    private init() {
        // intentionally left blank
    }

    deinit {
        if self.handle != 0 {
            var err = ExternError(code: 0, message: nil)
            glean_destroy_glean(self.handle, &err)
        }
    }

    public func initialize(uploadEnabled: Bool = true) {
        var cfg = FfiConfiguration(data_dir: "/tmp", package_name: "ios", upload_enabled: uploadEnabled ? 1 : 0, max_events: nil)
        self.handle = glean_initialize(&cfg)
    }
}
