/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Glean
@testable import GleanSampleFramework
import XCTest

class GleanSampleFrameworkTests: XCTestCase {
    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
        Glean.shared.enableTestingMode()
    }

    func testDoTheThing() {
        XCTAssertFalse(GleanMetrics.SampleFramework.didTheThing.testHasValue())
        GleanSampleFramework.doTheThing(withIntensity: 9002)
        XCTAssertTrue(GleanMetrics.SampleFramework.didTheThing.testHasValue())
        XCTAssertTrue(GleanMetrics.SampleFramework.thingCount.testHasValue())
        let counterValue = try! GleanMetrics.SampleFramework.thingCount.testGetValue()
        XCTAssertEqual(counterValue, 9002)
    }
}
