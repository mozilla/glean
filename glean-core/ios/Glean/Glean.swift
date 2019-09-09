//
//  Glean.swift
//  Glean
//
//  Created by Jan-Erik Rediger on 27.03.19.
//  Copyright © 2019 Jan-Erik Rediger. All rights reserved.
//

import Foundation

public class Glean {
    public static let shared = Glean()

    private var initialized: Bool = false
    private init() {
        self.initialized = true
    }

    deinit {
        self.initialized = false
    }

    /// Handle background event and send appropriate pings
    internal func handleBackgroundEvent() {
        // sendPings()
    }
}
