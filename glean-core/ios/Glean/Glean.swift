//
//  Glean.swift
//  Glean
//
//  Created by Jan-Erik Rediger on 27.03.19.
//  Copyright © 2019 Jan-Erik Rediger. All rights reserved.
//

import Foundation

public class Counter {
    var handle: UInt64

    public init(name: String, disabled: Bool) {
        self.handle = glean_new_counter_metric("glean", name, nil, 0, 0, 0)
    }

    public func add(amount: Int32 = 1) {
        glean_counter_add(Glean.shared.handle, self.handle, amount)
    }
}

public class Glean {
    public static let shared = Glean()

    private var initialized: Bool = false
    public let handle: UInt64

    private init() {
        var cfg = FfiConfiguration(data_dir: "/tmp", package_name: "ios", upload_enabled: 1, max_events: nil)
        self.handle = glean_initialize(&cfg)

        self.initialized = true
    }

    deinit {
        self.initialized = false
    }
}
