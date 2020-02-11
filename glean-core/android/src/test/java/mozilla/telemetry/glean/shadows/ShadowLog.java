/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.shadows;

import android.util.Log;

import org.robolectric.annotation.Implementation;
import org.robolectric.annotation.Implements;

import java.util.HashMap;

/**
 * Provide a custom `ShadowLog` for filtering annoying long lines in tests,
 * when using Robolectric.
 *
 * This class needs to be used by either configuring specific classes with
 * `@Configura(shadows=[mozilla.telemetry.glean.shadows.ShadowLog])` or by
 * defining a `robolectric.properties` file with
 * `shadows=mozilla.telemetry.glean.shadows.ShadowLog` (this would apply to
 * all tests).
 *
 * Disclaimer: as far as I can tell from the ShadowLog source here
 * https://github.com/robolectric/robolectric/blob/5e4746718e9818c4777f8ce437274e8f01b65669/shadows/framework/src/main/java/org/robolectric/shadows/ShadowLog.java#L121
 * this should have the same behaviour of calling `ShadowLog.setLoggable` without
 * having to resort to these custom shadows. However, it doesn't seem to work and
 * I have no idea why.
 */
@Implements(Log.class)
public class ShadowLog extends org.robolectric.shadows.ShadowLog {
    // Note: the following block creates an anonymous subclass for performing
    // the adds. However, this should be safe in the context of "static" objects.
    private static HashMap<String, Integer> blockList = new HashMap<String, Integer>() {{
        // Unfortunately, since we're providing our own mock for android.util.Log in
        // ./glean-core/test/java/android/util/Log.java, it means we cannot reference
        // Log.ERR. Since this is test only and we know that Log.ERR = 6, let's stick
        // to using that constant.
        put("CursorWindowStats", 6);
        put("SQLiteCursor", 6);
        put("SQLiteConnectionPool", 6);
    }};

    @Implementation
    public static synchronized boolean isLoggable(String tag, int level) {
        if (blockList.containsKey(tag)) {
            return level >= blockList.get(tag);
        }

        return org.robolectric.shadows.ShadowLog.isLoggable(tag, level);
    }
}

