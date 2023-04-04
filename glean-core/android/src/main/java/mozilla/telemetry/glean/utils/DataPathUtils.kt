package mozilla.telemetry.glean.utils

import java.io.File

/**
 * Check if the data path provided is valid and writable.
 *
 * @param dataPath A [String] provided by the user to specify the path to store data.
 * @return True if the database path is valid and writable.
 */
fun canWriteToDatabasePath(dataPath: String): Boolean {
    // Do not allow empty strings.
    if (dataPath.isEmpty()) {
        return false
    }

    // If the file exists we need to ensure we can write to it.
    val file = File(dataPath)
    if (file.exists()) {
        if (!file.canWrite()) {
            return false
        }
    }

    // The database path is valid and writable.
    return true
}
