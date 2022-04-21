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

import java.util.Calendar;
import java.util.HashMap;
import java.util.Map;

import mozilla.telemetry.glean.GleanMetrics.GleanInternalMetrics;
import mozilla.telemetry.glean.GleanMetrics.Pings;
import mozilla.telemetry.glean.internal.TimerId;
import mozilla.telemetry.glean.config.Configuration;

import static org.junit.Assert.assertEquals;

@RunWith(RobolectricTestRunner.class)
public class GleanFromJavaTest {
    // The only purpose of these tests is to make sure the Glean API is
    // callable from Java. If something goes wrong, it should complain about missing
    // methods at build-time.

    private Context appContext = TestUtilKt.getContext();
    private BuildInfo buildInfo = new BuildInfo("java-test", "java-test", Calendar.getInstance());

    @Before
    public void setup() {
        WorkManagerTestInitHelper.initializeTestWorkManager(appContext);
    }

    @Test
    public void testInitGleanWithDefaults() {
        Configuration config = new Configuration();
        Glean.INSTANCE.initialize(appContext, true, config, buildInfo);
    }

    @Test
    public void testInitGleanWithConfiguration() {
        Configuration config =
                new Configuration(Configuration.DEFAULT_TELEMETRY_ENDPOINT, "test-channel");
        Glean.INSTANCE.initialize(appContext, true, config, buildInfo);
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
        // FIXME: Java has no access to Kotlin typealias'd types,
        // so above we import the internal name.
        // Additionally we can't create it, because the constructor is private.
        // But that's ok, users should never create their own TimerId.
        TimerId testId;
    }

    @Test
    public void testCanSendPing() {
        // submit() can be called without parameters.
        Pings.INSTANCE.baseline().submit();
        // submit() can be called with a `reason`.
        Pings.INSTANCE.baseline().submit(Pings.baselineReasonCodes.inactive);
    }

    @Test
    public void testPassingExplicitBuildInfo() {
        Configuration config =
            new Configuration(Configuration.DEFAULT_TELEMETRY_ENDPOINT, "test-channel");
        BuildInfo buildInfo =
            new BuildInfo("foo", "c0ffee", Calendar.getInstance());
        Glean.INSTANCE.initialize(appContext, true, config, buildInfo);

        // The testing API doesn't seem to be available from these Java tests, so the best
        // we do right now is ensure the syntax above works.
        //   "To use the testing API, apply the GleanTestRule to set up a disposable Glean instance. e.g. GleanTestRule(ApplicationProvider.getApplicationContext())"
        // See bug 1692506

        // assertEquals("c0ffee", GleanInternalMetrics.INSTANCE.appBuild().testGetValue());
        // assertEquals("foo", GleanInternalMetrics.INSTANCE.appDisplayVersion().testGetValue());
    }
}
