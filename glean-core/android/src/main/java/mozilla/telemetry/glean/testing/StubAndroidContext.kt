/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.testing

import android.content.ComponentName
import android.content.Context
import android.content.ContentResolver
import android.content.Intent
import android.content.IntentFilter
import android.content.IntentSender
import android.content.ServiceConnection
import android.content.SharedPreferences
import android.content.BroadcastReceiver
import android.content.pm.ActivityInfo
import android.content.pm.ApplicationInfo
import android.content.pm.ChangedPackages
import android.content.pm.FeatureInfo
import android.content.pm.InstrumentationInfo
import android.content.pm.PackageInfo
import android.content.pm.PackageInstaller
import android.content.pm.PackageManager
import android.content.pm.PermissionGroupInfo
import android.content.pm.PermissionInfo
import android.content.pm.ProviderInfo
import android.content.pm.ResolveInfo
import android.content.pm.ServiceInfo
import android.content.pm.SharedLibraryInfo
import android.content.pm.VersionedPackage
import android.content.res.AssetManager
import android.content.res.Configuration
import android.content.res.Resources
import android.content.res.XmlResourceParser
import android.database.DatabaseErrorHandler
import android.database.sqlite.SQLiteDatabase
import android.graphics.Bitmap
import android.graphics.Rect
import android.graphics.drawable.Drawable
import android.net.Uri
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.os.UserHandle
import android.view.Display
import java.io.File
import java.io.FileInputStream
import java.io.FileOutputStream
import java.io.InputStream

/**
 * A stub [Context] used in unit tests for the Glean SDK.
 * This is additionally used by the SDK test rules.
 *
 * @param fakeDataDir the path to an existing directory to be reported by
 *        `ApplicationInfo.dataDir`.
 * @param fakePackageName the package name to be reported by `getPackageName`.
 */
internal class StubAndroidContext(
    private val fakeDataDir: String,
    private val fakePackageName: String
) : Context() {
    override fun getApplicationContext(): Context {
        TODO("Not yet implemented")
    }

    override fun setWallpaper(bitmap: Bitmap?) {
        TODO("Not yet implemented")
    }

    override fun setWallpaper(data: InputStream?) {
        TODO("Not yet implemented")
    }

    override fun removeStickyBroadcastAsUser(intent: Intent?, user: UserHandle?) {
        TODO("Not yet implemented")
    }

    override fun checkCallingOrSelfPermission(permission: String): Int {
        TODO("Not yet implemented")
    }

    override fun getClassLoader(): ClassLoader {
        TODO("Not yet implemented")
    }

    override fun checkCallingOrSelfUriPermission(uri: Uri?, modeFlags: Int): Int {
        TODO("Not yet implemented")
    }

    override fun getObbDir(): File {
        TODO("Not yet implemented")
    }

    override fun checkUriPermission(uri: Uri?, pid: Int, uid: Int, modeFlags: Int): Int {
        TODO("Not yet implemented")
    }

    override fun checkUriPermission(uri: Uri?, readPermission: String?, writePermission: String?, pid: Int, uid: Int, modeFlags: Int): Int {
        TODO("Not yet implemented")
    }

    override fun getExternalFilesDirs(type: String?): Array<File> {
        TODO("Not yet implemented")
    }

    override fun getPackageResourcePath(): String {
        TODO("Not yet implemented")
    }

    override fun deleteSharedPreferences(name: String?): Boolean {
        TODO("Not yet implemented")
    }

    override fun checkPermission(permission: String, pid: Int, uid: Int): Int {
        TODO("Not yet implemented")
    }

    override fun startIntentSender(intent: IntentSender?, fillInIntent: Intent?, flagsMask: Int, flagsValues: Int, extraFlags: Int) {
        TODO("Not yet implemented")
    }

    override fun startIntentSender(intent: IntentSender?, fillInIntent: Intent?, flagsMask: Int, flagsValues: Int, extraFlags: Int, options: Bundle?) {
        TODO("Not yet implemented")
    }

    override fun getSharedPreferences(name: String?, mode: Int): SharedPreferences = MockSharedPreferences()

    override fun sendStickyBroadcastAsUser(intent: Intent?, user: UserHandle?) {
        TODO("Not yet implemented")
    }

    override fun getDataDir(): File {
        TODO("Not yet implemented")
    }

    override fun getWallpaper(): Drawable {
        TODO("Not yet implemented")
    }

    override fun isDeviceProtectedStorage(): Boolean {
        TODO("Not yet implemented")
    }

    override fun getExternalFilesDir(type: String?): File? {
        TODO("Not yet implemented")
    }

    override fun sendBroadcastAsUser(intent: Intent?, user: UserHandle?) {
        TODO("Not yet implemented")
    }

    override fun sendBroadcastAsUser(intent: Intent?, user: UserHandle?, receiverPermission: String?) {
        TODO("Not yet implemented")
    }

    override fun getExternalCacheDir(): File? {
        TODO("Not yet implemented")
    }

    override fun getDatabasePath(name: String?): File {
        TODO("Not yet implemented")
    }

    override fun getFileStreamPath(name: String?): File {
        TODO("Not yet implemented")
    }

    override fun stopService(service: Intent?): Boolean {
        TODO("Not yet implemented")
    }

    override fun checkSelfPermission(permission: String): Int {
        TODO("Not yet implemented")
    }

    override fun registerReceiver(receiver: BroadcastReceiver?, filter: IntentFilter?): Intent? {
        TODO("Not yet implemented")
    }

    override fun registerReceiver(receiver: BroadcastReceiver?, filter: IntentFilter?, flags: Int): Intent? {
        TODO("Not yet implemented")
    }

    override fun registerReceiver(receiver: BroadcastReceiver?, filter: IntentFilter?, broadcastPermission: String?, scheduler: Handler?): Intent? {
        TODO("Not yet implemented")
    }

    override fun registerReceiver(receiver: BroadcastReceiver?, filter: IntentFilter?, broadcastPermission: String?, scheduler: Handler?, flags: Int): Intent? {
        TODO("Not yet implemented")
    }

    override fun getSystemServiceName(serviceClass: Class<*>): String? {
        TODO("Not yet implemented")
    }

    override fun getMainLooper(): Looper {
        TODO("Not yet implemented")
    }

    override fun enforceCallingOrSelfPermission(permission: String, message: String?) {
        TODO("Not yet implemented")
    }

    override fun getPackageCodePath(): String {
        TODO("Not yet implemented")
    }

    override fun checkCallingUriPermission(uri: Uri?, modeFlags: Int): Int {
        TODO("Not yet implemented")
    }

    override fun getWallpaperDesiredMinimumWidth(): Int {
        TODO("Not yet implemented")
    }

    override fun createDeviceProtectedStorageContext(): Context {
        TODO("Not yet implemented")
    }

    override fun openFileInput(name: String?): FileInputStream {
        TODO("Not yet implemented")
    }

    override fun getCodeCacheDir(): File {
        TODO("Not yet implemented")
    }

    override fun bindService(service: Intent?, conn: ServiceConnection, flags: Int): Boolean {
        TODO("Not yet implemented")
    }

    override fun deleteDatabase(name: String?): Boolean {
        TODO("Not yet implemented")
    }

    override fun getAssets(): AssetManager {
        TODO("Not yet implemented")
    }

    override fun getNoBackupFilesDir(): File {
        TODO("Not yet implemented")
    }

    override fun startActivities(intents: Array<out Intent>?) {
        TODO("Not yet implemented")
    }

    override fun startActivities(intents: Array<out Intent>?, options: Bundle?) {
        TODO("Not yet implemented")
    }

    override fun getResources(): Resources {
        TODO("Not yet implemented")
    }

    override fun fileList(): Array<String> {
        TODO("Not yet implemented")
    }

    override fun setTheme(resid: Int) {
        TODO("Not yet implemented")
    }

    override fun unregisterReceiver(receiver: BroadcastReceiver?) {
        TODO("Not yet implemented")
    }

    override fun enforcePermission(permission: String, pid: Int, uid: Int, message: String?) {
        TODO("Not yet implemented")
    }

    override fun openFileOutput(name: String?, mode: Int): FileOutputStream {
        TODO("Not yet implemented")
    }

    override fun sendStickyOrderedBroadcast(intent: Intent?, resultReceiver: BroadcastReceiver?, scheduler: Handler?, initialCode: Int, initialData: String?, initialExtras: Bundle?) {
        TODO("Not yet implemented")
    }

    override fun createConfigurationContext(overrideConfiguration: Configuration): Context {
        TODO("Not yet implemented")
    }

    override fun getFilesDir(): File {
        TODO("Not yet implemented")
    }

    override fun sendBroadcast(intent: Intent?) {
        TODO("Not yet implemented")
    }

    override fun sendBroadcast(intent: Intent?, receiverPermission: String?) {
        TODO("Not yet implemented")
    }

    override fun sendOrderedBroadcastAsUser(intent: Intent?, user: UserHandle?, receiverPermission: String?, resultReceiver: BroadcastReceiver?, scheduler: Handler?, initialCode: Int, initialData: String?, initialExtras: Bundle?) {
        TODO("Not yet implemented")
    }

    override fun grantUriPermission(toPackage: String?, uri: Uri?, modeFlags: Int) {
        TODO("Not yet implemented")
    }

    override fun enforceCallingUriPermission(uri: Uri?, modeFlags: Int, message: String?) {
        TODO("Not yet implemented")
    }

    override fun getCacheDir(): File {
        TODO("Not yet implemented")
    }

    override fun clearWallpaper() {
        TODO("Not yet implemented")
    }

    override fun sendStickyOrderedBroadcastAsUser(intent: Intent?, user: UserHandle?, resultReceiver: BroadcastReceiver?, scheduler: Handler?, initialCode: Int, initialData: String?, initialExtras: Bundle?) {
        TODO("Not yet implemented")
    }

    override fun startActivity(intent: Intent?) {
        TODO("Not yet implemented")
    }

    override fun startActivity(intent: Intent?, options: Bundle?) {
        TODO("Not yet implemented")
    }

    override fun getPackageManager(): PackageManager = object : PackageManager() {
        override fun getLaunchIntentForPackage(packageName: String): Intent? {
            TODO("Not yet implemented")
        }

        override fun getResourcesForApplication(app: ApplicationInfo): Resources {
            TODO("Not yet implemented")
        }

        override fun getResourcesForApplication(packageName: String): Resources {
            TODO("Not yet implemented")
        }

        override fun getReceiverInfo(component: ComponentName, flags: Int): ActivityInfo {
            TODO("Not yet implemented")
        }

        override fun queryIntentActivityOptions(caller: ComponentName?, specifics: Array<out Intent>?, intent: Intent, flags: Int): MutableList<ResolveInfo> {
            TODO("Not yet implemented")
        }

        override fun getApplicationIcon(info: ApplicationInfo): Drawable {
            TODO("Not yet implemented")
        }

        override fun getApplicationIcon(packageName: String): Drawable {
            TODO("Not yet implemented")
        }

        override fun extendVerificationTimeout(id: Int, verificationCodeAtTimeout: Int, millisecondsToDelay: Long) {
            TODO("Not yet implemented")
        }

        override fun getApplicationEnabledSetting(packageName: String): Int {
            TODO("Not yet implemented")
        }

        override fun queryIntentServices(intent: Intent, flags: Int): MutableList<ResolveInfo> {
            TODO("Not yet implemented")
        }

        override fun isPermissionRevokedByPolicy(permissionName: String, packageName: String): Boolean {
            TODO("Not yet implemented")
        }

        override fun checkPermission(permissionName: String, packageName: String): Int {
            TODO("Not yet implemented")
        }

        override fun checkSignatures(packageName1: String, packageName2: String): Int {
            TODO("Not yet implemented")
        }

        override fun checkSignatures(uid1: Int, uid2: Int): Int {
            TODO("Not yet implemented")
        }

        override fun removePackageFromPreferred(packageName: String) {
            TODO("Not yet implemented")
        }

        override fun addPermission(info: PermissionInfo): Boolean {
            TODO("Not yet implemented")
        }

        override fun getDrawable(packageName: String, resid: Int, appInfo: ApplicationInfo?): Drawable? {
            TODO("Not yet implemented")
        }

        override fun getChangedPackages(sequenceNumber: Int): ChangedPackages? {
            TODO("Not yet implemented")
        }

        override fun getPackageInfo(packageName: String, flags: Int): PackageInfo {
            val pi = PackageInfo()
            @Suppress("DEPRECATION")
            pi.versionCode = 12
            pi.versionName = "glean-vname"
            return pi
        }

        override fun getPackageInfo(versionedPackage: VersionedPackage, flags: Int): PackageInfo {
            TODO("Not yet implemented")
        }

        override fun getPackagesHoldingPermissions(permissions: Array<String>, flags: Int): MutableList<PackageInfo> {
            TODO("Not yet implemented")
        }

        override fun addPermissionAsync(info: PermissionInfo): Boolean {
            TODO("Not yet implemented")
        }

        override fun getSystemAvailableFeatures(): Array<FeatureInfo> {
            TODO("Not yet implemented")
        }

        override fun getSystemSharedLibraryNames(): Array<String>? {
            TODO("Not yet implemented")
        }

        override fun queryIntentContentProviders(intent: Intent, flags: Int): MutableList<ResolveInfo> {
            TODO("Not yet implemented")
        }

        override fun getApplicationBanner(info: ApplicationInfo): Drawable? {
            TODO("Not yet implemented")
        }

        override fun getApplicationBanner(packageName: String): Drawable? {
            TODO("Not yet implemented")
        }

        override fun getPackageGids(packageName: String): IntArray {
            TODO("Not yet implemented")
        }

        override fun getPackageGids(packageName: String, flags: Int): IntArray {
            TODO("Not yet implemented")
        }

        override fun getResourcesForActivity(activityName: ComponentName): Resources {
            TODO("Not yet implemented")
        }

        override fun getPackagesForUid(uid: Int): Array<String>? {
            TODO("Not yet implemented")
        }

        override fun getPermissionGroupInfo(permissionName: String, flags: Int): PermissionGroupInfo {
            TODO("Not yet implemented")
        }

        override fun addPackageToPreferred(packageName: String) {
            TODO("Not yet implemented")
        }

        override fun getComponentEnabledSetting(componentName: ComponentName): Int {
            TODO("Not yet implemented")
        }

        override fun getLeanbackLaunchIntentForPackage(packageName: String): Intent? {
            TODO("Not yet implemented")
        }

        override fun getInstalledPackages(flags: Int): MutableList<PackageInfo> {
            TODO("Not yet implemented")
        }

        override fun getAllPermissionGroups(flags: Int): MutableList<PermissionGroupInfo> {
            TODO("Not yet implemented")
        }

        override fun getNameForUid(uid: Int): String? {
            TODO("Not yet implemented")
        }

        override fun updateInstantAppCookie(cookie: ByteArray?) {
            TODO("Not yet implemented")
        }

        override fun getApplicationLogo(info: ApplicationInfo): Drawable? {
            TODO("Not yet implemented")
        }

        override fun getApplicationLogo(packageName: String): Drawable? {
            TODO("Not yet implemented")
        }

        override fun getApplicationLabel(info: ApplicationInfo): CharSequence {
            TODO("Not yet implemented")
        }

        override fun getPreferredActivities(outFilters: MutableList<IntentFilter>, outActivities: MutableList<ComponentName>, packageName: String?): Int {
            TODO("Not yet implemented")
        }

        override fun setApplicationCategoryHint(packageName: String, categoryHint: Int) {
            TODO("Not yet implemented")
        }

        override fun setInstallerPackageName(targetPackage: String, installerPackageName: String?) {
            TODO("Not yet implemented")
        }

        override fun getUserBadgedLabel(label: CharSequence, user: UserHandle): CharSequence {
            TODO("Not yet implemented")
        }

        override fun canRequestPackageInstalls(): Boolean {
            TODO("Not yet implemented")
        }

        override fun isInstantApp(): Boolean {
            TODO("Not yet implemented")
        }

        override fun isInstantApp(packageName: String): Boolean {
            TODO("Not yet implemented")
        }

        override fun getActivityIcon(activityName: ComponentName): Drawable {
            TODO("Not yet implemented")
        }

        override fun getActivityIcon(intent: Intent): Drawable {
            TODO("Not yet implemented")
        }

        override fun canonicalToCurrentPackageNames(packageNames: Array<String>): Array<String> {
            TODO("Not yet implemented")
        }

        override fun getProviderInfo(component: ComponentName, flags: Int): ProviderInfo {
            TODO("Not yet implemented")
        }

        override fun clearPackagePreferredActivities(packageName: String) {
            TODO("Not yet implemented")
        }

        override fun getPackageInstaller(): PackageInstaller {
            TODO("Not yet implemented")
        }

        override fun resolveService(intent: Intent, flags: Int): ResolveInfo? {
            TODO("Not yet implemented")
        }

        override fun verifyPendingInstall(id: Int, verificationCode: Int) {
            TODO("Not yet implemented")
        }

        override fun getInstantAppCookie(): ByteArray {
            TODO("Not yet implemented")
        }

        override fun getText(packageName: String, resid: Int, appInfo: ApplicationInfo?): CharSequence? {
            TODO("Not yet implemented")
        }

        override fun resolveContentProvider(authority: String, flags: Int): ProviderInfo? {
            TODO("Not yet implemented")
        }

        override fun hasSystemFeature(featureName: String): Boolean {
            TODO("Not yet implemented")
        }

        override fun hasSystemFeature(featureName: String, version: Int): Boolean {
            TODO("Not yet implemented")
        }

        override fun getInstrumentationInfo(className: ComponentName, flags: Int): InstrumentationInfo {
            TODO("Not yet implemented")
        }

        override fun getInstalledApplications(flags: Int): MutableList<ApplicationInfo> {
            TODO("Not yet implemented")
        }

        override fun getUserBadgedDrawableForDensity(drawable: Drawable, user: UserHandle, badgeLocation: Rect?, badgeDensity: Int): Drawable {
            TODO("Not yet implemented")
        }

        override fun getInstantAppCookieMaxBytes(): Int {
            TODO("Not yet implemented")
        }

        override fun getDefaultActivityIcon(): Drawable {
            TODO("Not yet implemented")
        }

        override fun getPreferredPackages(flags: Int): MutableList<PackageInfo> {
            TODO("Not yet implemented")
        }

        override fun addPreferredActivity(filter: IntentFilter, match: Int, set: Array<ComponentName>?, activity: ComponentName) {
            TODO("Not yet implemented")
        }

        override fun getSharedLibraries(flags: Int): MutableList<SharedLibraryInfo> {
            TODO("Not yet implemented")
        }

        override fun queryIntentActivities(intent: Intent, flags: Int): MutableList<ResolveInfo> {
            TODO("Not yet implemented")
        }

        override fun getActivityBanner(activityName: ComponentName): Drawable? {
            TODO("Not yet implemented")
        }

        override fun getActivityBanner(intent: Intent): Drawable? {
            TODO("Not yet implemented")
        }

        override fun setComponentEnabledSetting(componentName: ComponentName, newState: Int, flags: Int) {
            TODO("Not yet implemented")
        }

        override fun getApplicationInfo(packageName: String, flags: Int): ApplicationInfo {
            TODO("Not yet implemented")
        }

        override fun resolveActivity(intent: Intent, flags: Int): ResolveInfo? {
            TODO("Not yet implemented")
        }

        override fun queryBroadcastReceivers(intent: Intent, flags: Int): MutableList<ResolveInfo> {
            TODO("Not yet implemented")
        }

        override fun getXml(packageName: String, resid: Int, appInfo: ApplicationInfo?): XmlResourceParser? {
            TODO("Not yet implemented")
        }

        override fun getActivityLogo(activityName: ComponentName): Drawable? {
            TODO("Not yet implemented")
        }

        override fun getActivityLogo(intent: Intent): Drawable? {
            TODO("Not yet implemented")
        }

        override fun queryPermissionsByGroup(permissionGroup: String, flags: Int): MutableList<PermissionInfo> {
            TODO("Not yet implemented")
        }

        override fun queryContentProviders(processName: String?, uid: Int, flags: Int): MutableList<ProviderInfo> {
            TODO("Not yet implemented")
        }

        override fun getPermissionInfo(permissionName: String, flags: Int): PermissionInfo {
            TODO("Not yet implemented")
        }

        override fun removePermission(permissionName: String) {
            TODO("Not yet implemented")
        }

        override fun queryInstrumentation(targetPackage: String, flags: Int): MutableList<InstrumentationInfo> {
            TODO("Not yet implemented")
        }

        override fun clearInstantAppCookie() {
            TODO("Not yet implemented")
        }

        override fun currentToCanonicalPackageNames(packageNames: Array<String>): Array<String> {
            TODO("Not yet implemented")
        }

        override fun getPackageUid(packageName: String, flags: Int): Int {
            TODO("Not yet implemented")
        }

        override fun getUserBadgedIcon(drawable: Drawable, user: UserHandle): Drawable {
            TODO("Not yet implemented")
        }

        override fun getActivityInfo(component: ComponentName, flags: Int): ActivityInfo {
            TODO("Not yet implemented")
        }

        override fun isSafeMode(): Boolean {
            TODO("Not yet implemented")
        }

        override fun getInstallerPackageName(packageName: String): String? {
            TODO("Not yet implemented")
        }

        override fun setApplicationEnabledSetting(packageName: String, newState: Int, flags: Int) {
            TODO("Not yet implemented")
        }

        override fun getServiceInfo(component: ComponentName, flags: Int): ServiceInfo {
            TODO("Not yet implemented")
        }

    }

    override fun openOrCreateDatabase(name: String?, mode: Int, factory: SQLiteDatabase.CursorFactory?): SQLiteDatabase {
        TODO("Not yet implemented")
    }

    override fun openOrCreateDatabase(name: String?, mode: Int, factory: SQLiteDatabase.CursorFactory?, errorHandler: DatabaseErrorHandler?): SQLiteDatabase {
        TODO("Not yet implemented")
    }

    override fun deleteFile(name: String?): Boolean {
        TODO("Not yet implemented")
    }

    override fun startService(service: Intent?): ComponentName? {
        TODO("Not yet implemented")
    }

    override fun revokeUriPermission(uri: Uri?, modeFlags: Int) {
        TODO("Not yet implemented")
    }

    override fun revokeUriPermission(toPackage: String?, uri: Uri?, modeFlags: Int) {
        TODO("Not yet implemented")
    }

    override fun moveDatabaseFrom(sourceContext: Context?, name: String?): Boolean {
        TODO("Not yet implemented")
    }

    override fun startInstrumentation(className: ComponentName, profileFile: String?, arguments: Bundle?): Boolean {
        TODO("Not yet implemented")
    }

    override fun sendOrderedBroadcast(intent: Intent?, receiverPermission: String?) {
        TODO("Not yet implemented")
    }

    override fun sendOrderedBroadcast(intent: Intent, receiverPermission: String?, resultReceiver: BroadcastReceiver?, scheduler: Handler?, initialCode: Int, initialData: String?, initialExtras: Bundle?) {
        TODO("Not yet implemented")
    }

    override fun unbindService(conn: ServiceConnection) {
        TODO("Not yet implemented")
    }

    override fun getApplicationInfo(): ApplicationInfo {
        val ai = ApplicationInfo()
        ai.dataDir = fakeDataDir
        return ai
    }

    override fun getWallpaperDesiredMinimumHeight(): Int {
        TODO("Not yet implemented")
    }

    override fun createDisplayContext(display: Display): Context {
        TODO("Not yet implemented")
    }

    override fun createContextForSplit(splitName: String?): Context {
        TODO("Not yet implemented")
    }

    override fun getTheme(): Resources.Theme {
        TODO("Not yet implemented")
    }

    override fun getPackageName(): String = fakePackageName

    override fun getContentResolver(): ContentResolver {
        TODO("Not yet implemented")
    }

    override fun getObbDirs(): Array<File> {
        TODO("Not yet implemented")
    }

    override fun enforceCallingOrSelfUriPermission(uri: Uri?, modeFlags: Int, message: String?) {
        TODO("Not yet implemented")
    }

    override fun moveSharedPreferencesFrom(sourceContext: Context?, name: String?): Boolean {
        TODO("Not yet implemented")
    }

    override fun getExternalMediaDirs(): Array<File> {
        TODO("Not yet implemented")
    }

    override fun checkCallingPermission(permission: String): Int {
        TODO("Not yet implemented")
    }

    override fun getExternalCacheDirs(): Array<File> {
        TODO("Not yet implemented")
    }

    override fun sendStickyBroadcast(intent: Intent?) {
        TODO("Not yet implemented")
    }

    override fun enforceCallingPermission(permission: String, message: String?) {
        TODO("Not yet implemented")
    }

    override fun peekWallpaper(): Drawable {
        TODO("Not yet implemented")
    }

    override fun getSystemService(name: String): Any? {
        TODO("Not yet implemented")
    }

    override fun startForegroundService(service: Intent?): ComponentName? {
        TODO("Not yet implemented")
    }

    override fun getDir(name: String?, mode: Int): File {
        TODO("Not yet implemented")
    }

    override fun databaseList(): Array<String> {
        TODO("Not yet implemented")
    }

    override fun createPackageContext(packageName: String?, flags: Int): Context {
        TODO("Not yet implemented")
    }

    override fun enforceUriPermission(uri: Uri?, pid: Int, uid: Int, modeFlags: Int, message: String?) {
        TODO("Not yet implemented")
    }

    override fun enforceUriPermission(uri: Uri?, readPermission: String?, writePermission: String?, pid: Int, uid: Int, modeFlags: Int, message: String?) {
        TODO("Not yet implemented")
    }

    override fun removeStickyBroadcast(intent: Intent?) {
        TODO("Not yet implemented")
    }
}
