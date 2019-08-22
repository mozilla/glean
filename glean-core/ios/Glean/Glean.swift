//
//  Glean.swift
//  Glean
//
//  Created by Jan-Erik Rediger on 27.03.19.
//  Copyright © 2019 Jan-Erik Rediger. All rights reserved.
//

import Foundation

func scan<
    S : Sequence, U
    >(_ seq: S, _ initial: U, _ combine: (U, S.Iterator.Element) -> U) -> [U] {
    var result: [U] = []
    result.reserveCapacity(seq.underestimatedCount)
    var runningResult = initial
    for element in seq {
        runningResult = combine(runningResult, element)
        result.append(runningResult)
    }
    return result
}

func withArrayOfCStrings<R>(
    _ args: [String],
    _ body: ([UnsafeMutablePointer<CChar>?]) -> R
    ) -> R {
    let argsCounts = Array(args.map { $0.utf8.count + 1 })
    let argsOffsets = [ 0 ] + scan(argsCounts, 0, +)
    let argsBufferSize = argsOffsets.last!

    var argsBuffer: [UInt8] = []
    argsBuffer.reserveCapacity(argsBufferSize)
    for arg in args {
        argsBuffer.append(contentsOf: arg.utf8)
        argsBuffer.append(0)
    }

    return argsBuffer.withUnsafeMutableBufferPointer {
        (argsBuffer) in
        let ptr = UnsafeMutableRawPointer(argsBuffer.baseAddress!).bindMemory(
            to: CChar.self, capacity: argsBuffer.count)
        var cStrings: [UnsafeMutablePointer<CChar>?] = argsOffsets.map { ptr + $0 }
        cStrings[cStrings.count - 1] = nil
        return body(cStrings)
    }
}

public class Counter {
    var handle: UInt64

    public init(name: String, disabled: Bool) {
        let sendInPings = ["baseline"]
        self.handle = withArrayOfCStrings(sendInPings) {
            args in
            glean_new_counter_metric("glean", name, args, Int32(sendInPings.count), 0, 0)
        }
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
