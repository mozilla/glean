/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.net

import android.util.Log
import androidx.annotation.VisibleForTesting
import java.io.ByteArrayOutputStream
import java.io.IOException
import java.net.CookieHandler
import java.net.CookieManager
import java.net.HttpURLConnection
import java.net.MalformedURLException
import java.net.URL

/**
 * A simple ping Uploader, which implements a "send once" policy, never
 * storing or attempting to send the ping again.
 */
class HttpURLConnectionUploader : PingUploader {
    companion object {
        private const val LOG_TAG: String = "glean/HttpConnUploader"
        /**
         * The timeout, in milliseconds, to use when connecting to the server.
         */
        const val DEFAULT_CONNECTION_TIMEOUT = 10000
        /**
         * The timeout, in milliseconds, to use when reading from the server.
         */
        const val DEFAULT_READ_TIMEOUT = 30000
    }

    /**
     * Synchronously upload a ping to a server.
     *
     * @param url the URL path to upload the data to
     * @param data the serialized text data to send
     * @param headers a [HeadersList] containing String to String [Pair] with
     *        the first entry being the header name and the second its value.
     * @param isGzipped whether or not the payload is gzipped
     *
     * @return return the status code of the upload response,
     *         or null in case unable to upload.
     */
    @Suppress("ReturnCount", "MagicNumber")
    override fun upload(url: String, data: ByteArray, headers: HeadersList): UploadResult {
        var connection: HttpURLConnection? = null
        try {
            connection = openConnection(url)
            connection.requestMethod = "POST"
            connection.connectTimeout = DEFAULT_CONNECTION_TIMEOUT
            connection.readTimeout = DEFAULT_READ_TIMEOUT
            connection.doOutput = true

            headers.forEach { (headerName, headerValue) ->
                connection.setRequestProperty(headerName, headerValue)
            }

            // Make sure we are not sending cookies. Unfortunately, HttpURLConnection doesn't
            // offer a better API to do that, so we nuke all cookies going to our telemetry
            // endpoint.
            removeCookies(url)

            // Finally upload.
            val statusCode = doUpload(connection, data)
            return HttpResponse(statusCode)
        } catch (e: MalformedURLException) {
            // There's nothing we can do to recover from this here. So let's just log an error and
            // notify the service that this job has been completed - even though we didn't upload
            // anything to the server.
            Log.e(LOG_TAG, "Could not upload telemetry due to malformed URL", e)
            return UnrecoverableFailure
        } catch (e: IOException) {
            Log.w(LOG_TAG, "IOException while uploading ping", e)
            return RecoverableFailure
        } finally {
            connection?.disconnect()
        }
    }

    /**
     * Remove all the cookies related to the server endpoint, so
     * that nothing other than ping data travels to the endpoint.
     *
     * @param submissionUrl the submissionUrl, including server address and path
     */
    internal fun removeCookies(submissionUrl: String) {
        (CookieHandler.getDefault() as? CookieManager)?.let { cookieManager ->
            val url = try {
                val fullUrl = URL(submissionUrl)
                // We just need the protocol, host and port.
                val onlyHostUrl = fullUrl.protocol + "://" + fullUrl.host + ":" + fullUrl.port
                URL(onlyHostUrl)
            } catch (e: MalformedURLException) {
                null
            }

            url?.let {
                val uri = it.toURI()
                for (cookie in cookieManager.cookieStore.get(uri)) {
                    cookieManager.cookieStore.remove(uri, cookie)
                }
            }
        }
    }

    @Throws(IOException::class)
    internal fun doUpload(connection: HttpURLConnection, data: ByteArray): Int {
        connection.outputStream.use {
            val byteOutputStream = ByteArrayOutputStream()
            byteOutputStream.write(data)
            byteOutputStream.writeTo(it)
            it.flush()
        }

        return connection.responseCode
    }

    @VisibleForTesting @Throws(IOException::class)
    internal fun openConnection(url: String): HttpURLConnection {
        return URL(url).openConnection() as HttpURLConnection
    }
}
