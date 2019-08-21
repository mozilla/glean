//
//  Glean.swift
//  Glean
//
//  Created by Jan-Erik Rediger on 27.03.19.
//  Copyright © 2019 Jan-Erik Rediger. All rights reserved.
//

import Foundation

public class Counter {
    var count: Int = 0
    var name: String
    var disabled: Bool

    public init(name: String, disabled: Bool) {
        self.name = name
        self.disabled = disabled
    }

    public func add(amount: Int = 1) {
        if (!self.disabled) {
            return
        }

        self.count += amount
    }
}

public class Glean {
    public static let shared = Glean()

    private var initialized: Bool = false
    private init() {
        self.initialized = true
    }

    deinit {
        self.initialized = false
    }

    func t() {
        let c = Counter(name: "hello", disabled: true)
        c.add(amount: 3)
    }
}
