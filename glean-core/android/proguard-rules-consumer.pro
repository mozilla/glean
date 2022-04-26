# ProGuard rules for consumers of this library.

# JNA specific rules
# See https://github.com/java-native-access/jna/blob/master/www/FrequentlyAskedQuestions.md#jna-on-android
-dontwarn java.awt.*
-keep class com.sun.jna.* { *; }
-keepclassmembers class * extends com.sun.jna.* { public *; }

# Glean specific rules
-keep class mozilla.telemetry.** { *; }

# The Glean SDK ships with classes used for tests as well. They are disabled
# and not directly usable in production code: they throw if used there. They
# can be used in tests just fine but, outside of tests, the test dependency
# they use won't be there, hence the warning. It's safe to suppress these.
-dontwarn mozilla.telemetry.glean.testing.**
