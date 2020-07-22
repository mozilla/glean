/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

// swiftlint:disable force_cast
// REASON: Used in a test
class JweMetricTests: XCTestCase {
    // swiftlint:disable line_length
    private let header: String = "eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ"
    private let key: String = "OKOawDo13gRp2ojaHV7LFpZcgV7T6DVZKTyKOMTYUmKoTCVJRgckCL9kiMT03JGeipsEdY3mx_etLbbWSrFr05kLzcSr4qKAq7YN7e9jwQRb23nfa6c9d-StnImGyFDbSv04uVuxIp5Zms1gNxKKK2Da14B8S4rzVRltdYwam_lDp5XnZAYpQdb76FdIKLaVmqgfwX7XWRxv2322i-vDxRfqNzo_tETKzpVLzfiwQyeyPGLBIO56YJ7eObdv0je81860ppamavo35UgoRdbYaBcoh9QcfylQr66oc6vFWXRcZ_ZT2LawVCWTIy3brGPi6UklfCpIMfIjf7iGdXKHzg"
    private let initVector: String = "48V1_ALb6US04U3b"
    private let cipherText: String = "5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A"
    private let authTag: String = "XFBoMYUZodetZdvTiFvSkQ"
    private let jwe: String = "eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ.OKOawDo13gRp2ojaHV7LFpZcgV7T6DVZKTyKOMTYUmKoTCVJRgckCL9kiMT03JGeipsEdY3mx_etLbbWSrFr05kLzcSr4qKAq7YN7e9jwQRb23nfa6c9d-StnImGyFDbSv04uVuxIp5Zms1gNxKKK2Da14B8S4rzVRltdYwam_lDp5XnZAYpQdb76FdIKLaVmqgfwX7XWRxv2322i-vDxRfqNzo_tETKzpVLzfiwQyeyPGLBIO56YJ7eObdv0je81860ppamavo35UgoRdbYaBcoh9QcfylQr66oc6vFWXRcZ_ZT2LawVCWTIy3brGPi6UklfCpIMfIjf7iGdXKHzg.48V1_ALb6US04U3b.5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A.XFBoMYUZodetZdvTiFvSkQ"
    private let minimumJwe: String = "eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ...5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A."
    // swiftlint:enable line_length

    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
    }

    func testJweSavesToStorage() {
        let jweMetric = JweMetricType(
            category: "telemetry",
            name: "jwe_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        XCTAssertFalse(jweMetric.testHasValue())

        jweMetric.set(self.header, self.key, self.initVector, self.cipherText, self.authTag)

        XCTAssert(jweMetric.testHasValue())
        XCTAssertEqual(self.jwe, try jweMetric.testGetCompactRepresentation())

        jweMetric.set(self.header, "", "", self.cipherText, "")

        XCTAssert(jweMetric.testHasValue())
        XCTAssertEqual(self.minimumJwe, try jweMetric.testGetCompactRepresentation())
    }

    func testJweMustNotRecordIfDisabled() {
        let jweMetric = JweMetricType(
            category: "telemetry",
            name: "jwe_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: true
        )

        XCTAssertFalse(jweMetric.testHasValue())

        jweMetric.setWithCompactRepresentation(self.jwe)

        XCTAssertFalse(jweMetric.testHasValue(), "JWEs must not be recorded if they are disabled")
    }

    func testJweGetValueThrowsExceptionIfNothingIsStored() {
        let jweMetric = JweMetricType(
            category: "telemetry",
            name: "jwe_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        XCTAssertThrowsError(try jweMetric.testGetValue()) { error in
            XCTAssertEqual(error as! String, "Missing value")
        }
    }

    func testJweGetValueReturnsCorrectJweDataRepresentation() {
        let jweMetric = JweMetricType(
            category: "telemetry",
            name: "jwe_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        jweMetric.set(self.header, self.key, self.initVector, self.cipherText, self.authTag)

        let data = try! jweMetric.testGetValue()
        XCTAssertEqual(data.header, self.header)
        XCTAssertEqual(data.key, self.key)
        XCTAssertEqual(data.initVector, self.initVector)
        XCTAssertEqual(data.cipherText, self.cipherText)
        XCTAssertEqual(data.authTag, self.authTag)
    }

    func testJweSavesToSecondaryPings() {
        let jweMetric = JweMetricType(
            category: "telemetry",
            name: "jwe_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        )

        jweMetric.set(self.header, self.key, self.initVector, self.cipherText, self.authTag)

        XCTAssert(jweMetric.testHasValue("store2"))
        XCTAssertEqual(jwe, try jweMetric.testGetCompactRepresentation())

        jweMetric.set(self.header, "", "", self.cipherText, "")

        XCTAssert(jweMetric.testHasValue())
        XCTAssertEqual(self.minimumJwe, try jweMetric.testGetCompactRepresentation())
    }

    func testSettingInvalidValuesRecordsErrors() {
        let jweMetric = JweMetricType(
            category: "telemetry",
            name: "jwe_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        // Too long elements should yield a InvalidOverflow error
        jweMetric.set(String(repeating: "X", count: 1025), self.key, self.initVector, self.cipherText, self.authTag)
        XCTAssertEqual(1, jweMetric.testGetNumRecordedErrors(ErrorType.invalidOverflow))

        // Invalid compact string representation yield a InvalidValue error
        jweMetric.setWithCompactRepresentation("")
        XCTAssertEqual(1, jweMetric.testGetNumRecordedErrors(ErrorType.invalidValue))
    }
}
