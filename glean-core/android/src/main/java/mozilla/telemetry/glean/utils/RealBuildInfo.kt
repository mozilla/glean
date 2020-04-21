package mozilla.telemetry.glean.utils

import android.os.Build

class RealBuildInfo : AndroidBuildInfo {
    // https://developer.android.com/reference/android/os/Build.VERSION
    override fun getSdkVersion(): String = Build.VERSION.SDK_INT.toString()
    override fun getVersionString(): String = Build.VERSION.RELEASE
    override fun getDeviceManufacturer(): String = Build.MANUFACTURER
    override fun getDeviceModel(): String = Build.MODEL
    override fun getPreferredABI(): String = Build.SUPPORTED_ABIS[0]
}