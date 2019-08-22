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
    var cStrings = args.map { UnsafePointer(strdup($0)) }
    defer {
        cStrings.forEach { free(UnsafeMutableRawPointer(mutating: $0)) }
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

    public func testGetValue() -> Int32 {
        return glean_counter_test_get_value(Glean.shared.handle, self.handle, "metrics")
    }

    public func testHasValue() -> Bool {
        return glean_counter_test_has_value(Glean.shared.handle, self.handle, "metrics") != 0
    }
}

func withConfig<R>(data_dir: String, package_name: String, upload_enabled: Bool, _ body: (FfiConfiguration) -> R) -> R {
    let data_dir = strdup(data_dir)
    let package_name = strdup(package_name)
    defer {
        free(data_dir)
        free(package_name)
    }
    let cfg = FfiConfiguration(data_dir: data_dir, package_name: package_name, upload_enabled: upload_enabled ? 1 : 0, max_events: nil)
    return body(cfg)
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
        glean_enable_logging();
        self.handle = withConfig(data_dir: "/tmp/ios-glean", package_name: "ios", upload_enabled: uploadEnabled) { cfg in
            var cfg = cfg
            return glean_initialize(&cfg)
        }
    }

    public func testClearAllStores() {
        glean_test_clear_all_stores(self.handle)
    }

    public func setUploadEnabled(_ upload: Bool) {
        glean_set_upload_enabled(self.handle, upload ? 1 : 0)
    }

    public func sendPings(_ pings: [String]) -> Bool {
        return withArrayOfCStrings(pings) { pingNames in
            return glean_send_pings_by_name(self.handle, pingNames, Int32(pings.count), 1) != 0
        }
    }
}
