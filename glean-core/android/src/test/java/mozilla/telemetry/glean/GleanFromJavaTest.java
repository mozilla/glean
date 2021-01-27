/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean;

import android.content.Context;

import androidx.work.testing.WorkManagerTestInitHelper;

import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.robolectric.RobolectricTestRunner;

import java.util.HashMap;
import java.util.Map;

import mozilla.telemetry.glean.GleanMetrics.GleanInternalMetrics;
import mozilla.telemetry.glean.GleanMetrics.Pings;
import mozilla.telemetry.glean.config.Configuration;

@RunWith(RobolectricTestRunner.class)
public class GleanFromJavaTest {
    // The only purpose of these tests is to make sure the Glean API is
    // callable from Java. If something goes wrong, it should complain about missing
    // methods at build-time.

    private Context appContext = TestUtilKt.getContextWithMockedInfo("java-test");

    @Before
    public void setup() {
        WorkManagerTestInitHelper.initializeTestWorkManager(appContext);
    }

    @Test
    public void testInitGleanWithDefaults() {
        Glean.INSTANCE.initialize(appContext, true);
    }

    @Test
    public void testInitGleanWithConfiguration() {
        Configuration config =
                new Configuration(Configuration.DEFAULT_TELEMETRY_ENDPOINT, "test-channel");
        Glean.INSTANCE.initialize(appContext, true, config);
    }

    @Test
    public void testGleanExperimentsAPIWithDefaults() {
        Glean.INSTANCE.setExperimentActive("test-exp-id-1", "test-branch-1");
    }

    @Test
    public void testGleanExperimentsAPIWithOptional() {
        Map<String, String> experimentProperties = new HashMap<>();
        experimentProperties.put("test-prop1", "test-prop-result1");

        Glean.INSTANCE.setExperimentActive(
                "test-exp-id-1",
                "test-branch-1",
                experimentProperties
        );
    }

    @Test
    public void testCanAccessGleanTimerId() {
        // Users are not really meant to instantiate this. Moreover, the constructor
        // visibility is `internal`, but looks like Java ignores it.
        GleanTimerId testId = new GleanTimerId(100);
    }

    @Test
    public void testCanSendPing() {
        // submit() can be called without parameters.
        Pings.INSTANCE.baseline().submit();
        // submit() can be called with a `reason`.
        Pings.INSTANCE.baseline().submit(Pings.baselineReasonCodes.inactive);
    }
}
