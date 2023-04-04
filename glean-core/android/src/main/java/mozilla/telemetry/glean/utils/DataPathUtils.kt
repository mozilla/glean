package mozilla.telemetry.glean.utils

import mozilla.telemetry.glean.GleanInternalAPI
import java.io.File

/**
 * Takes a base data directory and a custom data path and creates
 * the data storage location on the device.
 *
 * @param dataDir A [String] that specifies the base directory where we will
 *     store data.
 * @param customDataPath An optional [String] provided by the user to specify
 *     the specific path to store data.
 */
fun generateGleanStoragePath(dataDir: String, customDataPath: String?): File {
    val file: File = customDataPath?.let { safeDataPath ->
        File(dataDir, safeDataPath)
    } ?: run {
        File(
            dataDir,
            GleanInternalAPI.GLEAN_DATA_DIR
        )
    }
    return file
}

/**
 * Check if the data path provided is valid and writable.
 *
 * @param dataDir A [String] that specifies the base directory where we will
 *     store data.
 * @param customDataPath A [String] provided by the user to specify the path to store data.
 */
fun canWriteToDatabasePath(dataDir: String, customDataPath: String): Boolean {
    // Do not allow empty strings.
    if (customDataPath.isEmpty()) {
        return false
    }

    // Generate the full path that we want to write to using our custom data path.
    val fullFilePath = generateGleanStoragePath(dataDir, customDataPath).absolutePath

    // If the file exists we need to ensure we can write to it.
    val file = File(fullFilePath)
    if (file.exists()) {
        if (!file.canWrite()) {
            return false
        }
    }

    // The database path is valid and writable.
    return true
}
