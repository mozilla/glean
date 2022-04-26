/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/* This file is based on the tests in the Glean android-components implentation.
 *
 * Care should be taken to not reorder elements in this file so it will be easier
 * to track changes in Glean android-components.
 */

package mozilla.telemetry.glean.private

import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import mozilla.telemetry.glean.testing.ErrorType
import mozilla.telemetry.glean.testing.GleanTestRule
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import java.lang.NullPointerException

@RunWith(AndroidJUnit4::class)
@Suppress("MaxLineLength")
class JweMetricTypeTest {
    companion object {
        const val HEADER: String = "eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ"
        const val KEY: String = "OKOawDo13gRp2ojaHV7LFpZcgV7T6DVZKTyKOMTYUmKoTCVJRgckCL9kiMT03JGeipsEdY3mx_etLbbWSrFr05kLzcSr4qKAq7YN7e9jwQRb23nfa6c9d-StnImGyFDbSv04uVuxIp5Zms1gNxKKK2Da14B8S4rzVRltdYwam_lDp5XnZAYpQdb76FdIKLaVmqgfwX7XWRxv2322i-vDxRfqNzo_tETKzpVLzfiwQyeyPGLBIO56YJ7eObdv0je81860ppamavo35UgoRdbYaBcoh9QcfylQr66oc6vFWXRcZ_ZT2LawVCWTIy3brGPi6UklfCpIMfIjf7iGdXKHzg"
        const val INIT_VECTOR: String = "48V1_ALb6US04U3b"
        const val CIPHER_TEXT: String = "5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A"
        const val AUTH_TAG: String = "XFBoMYUZodetZdvTiFvSkQ"
        const val JWE: String = "eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ.OKOawDo13gRp2ojaHV7LFpZcgV7T6DVZKTyKOMTYUmKoTCVJRgckCL9kiMT03JGeipsEdY3mx_etLbbWSrFr05kLzcSr4qKAq7YN7e9jwQRb23nfa6c9d-StnImGyFDbSv04uVuxIp5Zms1gNxKKK2Da14B8S4rzVRltdYwam_lDp5XnZAYpQdb76FdIKLaVmqgfwX7XWRxv2322i-vDxRfqNzo_tETKzpVLzfiwQyeyPGLBIO56YJ7eObdv0je81860ppamavo35UgoRdbYaBcoh9QcfylQr66oc6vFWXRcZ_ZT2LawVCWTIy3brGPi6UklfCpIMfIjf7iGdXKHzg.48V1_ALb6US04U3b.5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A.XFBoMYUZodetZdvTiFvSkQ"
        const val MINIMUM_JWE: String = "eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ...5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A."
    }

    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

    @Test
    fun `The API saves to its storage engine`() {
        // Define a 'jweMetric' jwe metric, which will be stored in "store1"
        val jweMetric = JweMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "jwe_metric",
            sendInPings = listOf("store1")
        )

        // Record two JWEs of the same type, with a little delay.
        jweMetric.set(HEADER, KEY, INIT_VECTOR, CIPHER_TEXT, AUTH_TAG)

        // Check that data was properly recorded.
        assertTrue(jweMetric.testHasValue())
        assertEquals(JWE, jweMetric.testGetCompactRepresentation())

        jweMetric.set(HEADER, "", "", CIPHER_TEXT, "")
        // Check that data was properly recorded.
        assertTrue(jweMetric.testHasValue())
        assertEquals(MINIMUM_JWE, jweMetric.testGetCompactRepresentation())
    }

    @Test
    fun `disabled JWEs must not record data`() {
        // Define a 'jweMetric' jwe metric, which will be stored in "store1". It's disabled
        // so it should not record anything.
        val jweMetric = JweMetricType(
            disabled = true,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "jwe_metric",
            sendInPings = listOf("store1")
        )

        // Attempt to store the JWE.
        jweMetric.set(HEADER, KEY, INIT_VECTOR, CIPHER_TEXT, AUTH_TAG)
        // Check that nothing was recorded.
        assertFalse(
            "JWEs must not be recorded if they are disabled",
            jweMetric.testHasValue()
        )
    }

    @Test(expected = NullPointerException::class)
    fun `testGetValue() throws NullPointerException if nothing is stored`() {
        val jweMetric = JweMetricType(
            disabled = true,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "jwe_metric",
            sendInPings = listOf("store1")
        )
        jweMetric.testGetValue()
    }

    @Test
    fun `testGetValue() returns correct JweData representation`() {
        // Define a 'jweMetric' jwe metric, which will be stored in "store1".
        val jweMetric = JweMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "jwe_metric",
            sendInPings = listOf("store1")
        )

        // Attempt to store the JWE.
        jweMetric.set(HEADER, KEY, INIT_VECTOR, CIPHER_TEXT, AUTH_TAG)

        val data = jweMetric.testGetValue()
        assertEquals(data.header, HEADER)
        assertEquals(data.key, KEY)
        assertEquals(data.initVector, INIT_VECTOR)
        assertEquals(data.cipherText, CIPHER_TEXT)
        assertEquals(data.authTag, AUTH_TAG)
    }

    @Test
    fun `The API saves to secondary pings`() {
        // Define a 'jweMetric' jwe metric, which will be stored in "store1" and "store2"
        val jweMetric = JweMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "jwe_metric",
            sendInPings = listOf("store1", "store2")
        )

        // Record two JWEs, with a little delay.
        jweMetric.set(HEADER, KEY, INIT_VECTOR, CIPHER_TEXT, AUTH_TAG)

        // Check that data was properly recorded in the second ping.
        assertTrue(jweMetric.testHasValue("store2"))
        assertEquals(JWE, jweMetric.testGetCompactRepresentation("store2"))

        jweMetric.set(HEADER, "", "", CIPHER_TEXT, "")
        // Check that data was properly recorded in the second ping.
        assertTrue(jweMetric.testHasValue("store2"))
        assertEquals(MINIMUM_JWE, jweMetric.testGetCompactRepresentation())
    }

    @Test
    fun `Trying to set invalid values records errors`() {
        // Define a 'jweMetric' jwe metric, which will be stored in "store1" and "store2"
        val jweMetric = JweMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "jwe_metric",
            sendInPings = listOf("store1", "store2")
        )

        // Too long elements should yield a InvalidOverflow error
        jweMetric.set("X".repeat(1025), KEY, INIT_VECTOR, CIPHER_TEXT, AUTH_TAG)
        assertEquals(1, jweMetric.testGetNumRecordedErrors(ErrorType.InvalidOverflow))

        // Invalid compact string representation yield a InvalidValue error
        jweMetric.setWithCompactRepresentation("")
        assertEquals(1, jweMetric.testGetNumRecordedErrors(ErrorType.InvalidValue))
    }
}
